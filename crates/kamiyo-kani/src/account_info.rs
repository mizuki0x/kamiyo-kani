//! Solana `AccountInfo` generators and invariants for Kani proofs.

use solana_program::{account_info::AccountInfo, pubkey::Pubkey, rent::Rent};
use std::{cell::RefCell, ops::RangeInclusive, rc::Rc};

#[derive(Clone, Debug)]
enum Lamports {
    Any,
    Exact(u64),
    Range(RangeInclusive<u64>),
}

#[derive(Clone, Debug)]
pub struct AccountConfig {
    key: Option<Pubkey>,
    owner: Option<Pubkey>,
    lamports: Lamports,
    rent_exempt: bool,
    pub is_signer: bool,
    pub is_writable: bool,
    pub executable: bool,
    pub rent_epoch: u64,
}

impl Default for AccountConfig {
    fn default() -> Self {
        Self {
            key: None,
            owner: None,
            lamports: Lamports::Any,
            rent_exempt: true,
            is_signer: false,
            is_writable: false,
            executable: false,
            rent_epoch: 0,
        }
    }
}

impl AccountConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key(mut self, key: Pubkey) -> Self {
        self.key = Some(key);
        self
    }

    pub fn owner(mut self, owner: Pubkey) -> Self {
        self.owner = Some(owner);
        self
    }

    pub fn signer(mut self) -> Self {
        self.is_signer = true;
        self
    }

    pub fn writable(mut self) -> Self {
        self.is_writable = true;
        self
    }

    pub fn payer(mut self) -> Self {
        self.is_signer = true;
        self.is_writable = true;
        self
    }

    pub fn executable(mut self) -> Self {
        self.executable = true;
        self
    }

    pub fn lamports(mut self, lamports: u64) -> Self {
        self.lamports = Lamports::Exact(lamports);
        self
    }

    pub fn lamports_range(mut self, range: RangeInclusive<u64>) -> Self {
        self.lamports = Lamports::Range(range);
        self
    }

    pub fn rent_exempt(mut self, enabled: bool) -> Self {
        self.rent_exempt = enabled;
        self
    }

    pub fn rent_epoch(mut self, epoch: u64) -> Self {
        self.rent_epoch = epoch;
        self
    }
}

pub fn any_agent_account<const DATA_LEN: usize>(cfg: AccountConfig) -> AccountInfo<'static> {
    any_account_info::<DATA_LEN>(cfg)
}

fn any_pubkey() -> Pubkey {
    Pubkey::new_from_array(kani::any())
}

fn pick_lamports<const DATA_LEN: usize>(cfg: &AccountConfig) -> u64 {
    let lamports = match &cfg.lamports {
        Lamports::Any => kani::any(),
        Lamports::Exact(v) => *v,
        Lamports::Range(r) => {
            let v: u64 = kani::any();
            kani::assume(r.contains(&v));
            v
        }
    };

    if cfg.rent_exempt {
        kani::assume(Rent::default().is_exempt(lamports, DATA_LEN));
    }

    lamports
}

pub fn any_account_info<const DATA_LEN: usize>(cfg: AccountConfig) -> AccountInfo<'static> {
    let key = cfg.key.unwrap_or_else(any_pubkey);
    let owner = cfg.owner.unwrap_or_else(any_pubkey);
    let lamports = pick_lamports::<DATA_LEN>(&cfg);

    let key: &'static Pubkey = Box::leak(Box::new(key));
    let owner: &'static Pubkey = Box::leak(Box::new(owner));

    let lamports: &'static mut u64 = Box::leak(Box::new(lamports));
    let data: &'static mut [u8; DATA_LEN] = Box::leak(Box::new(kani::any()));
    let data: &'static mut [u8] = data;

    AccountInfo {
        key,
        is_signer: cfg.is_signer,
        is_writable: cfg.is_writable,
        lamports: Rc::new(RefCell::new(lamports)),
        data: Rc::new(RefCell::new(data)),
        owner,
        executable: cfg.executable,
        rent_epoch: cfg.rent_epoch,
    }
}

pub fn lamports(account: &AccountInfo<'_>) -> u64 {
    // Proof harnesses construct single-owner accounts; direct access avoids
    // `RefCell` panic paths that create unsupported constructs in Kani.
    unsafe { **account.lamports.as_ptr() }
}

pub fn sum_lamports(accounts: &[&AccountInfo<'_>]) -> u128 {
    accounts.iter().map(|a| lamports(a) as u128).sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LamportSnapshot(pub u128);

impl LamportSnapshot {
    pub fn new(accounts: &[&AccountInfo<'_>]) -> Self {
        Self(sum_lamports(accounts))
    }

    pub fn unchanged(self, accounts: &[&AccountInfo<'_>]) -> bool {
        sum_lamports(accounts) == self.0
    }
}

#[cfg(kani)]
mod proofs {
    use super::*;

    #[kani::proof]
    fn proof_account_config_flags_are_applied() {
        let mut cfg = AccountConfig::new().rent_exempt(false);

        let signer: bool = kani::any();
        let writable: bool = kani::any();
        let executable: bool = kani::any();
        let rent_epoch: u64 = kani::any();

        if signer {
            cfg = cfg.signer();
        }
        if writable {
            cfg = cfg.writable();
        }
        if executable {
            cfg = cfg.executable();
        }
        cfg = cfg.rent_epoch(rent_epoch);

        let account = any_account_info::<16>(cfg);
        kani::assert(account.is_signer == signer, "signer flag mismatch");
        kani::assert(account.is_writable == writable, "writable flag mismatch");
        kani::assert(account.executable == executable, "executable flag mismatch");
        kani::assert(account.rent_epoch == rent_epoch, "rent_epoch mismatch");
    }

    #[kani::proof]
    fn proof_account_lamports_range_is_enforced() {
        let min: u64 = u64::from(kani::any::<u16>());
        let max: u64 = u64::from(kani::any::<u16>());
        kani::assume(min <= max);

        let account = any_account_info::<0>(
            AccountConfig::new()
                .rent_exempt(false)
                .lamports_range(min..=max),
        );
        let value = lamports(&account);
        kani::assert(value >= min, "lamports below configured min");
        kani::assert(value <= max, "lamports above configured max");
    }

    #[kani::proof]
    fn proof_lamport_snapshot_tracks_mutations() {
        let from_start: u64 = u64::from(kani::any::<u16>());
        let to_start: u64 = u64::from(kani::any::<u16>());
        let delta: u64 = u64::from(kani::any::<u16>());

        kani::assume(from_start >= delta);
        kani::assume(to_start.checked_add(delta).is_some());

        let from = any_account_info::<8>(
            AccountConfig::new()
                .rent_exempt(false)
                .writable()
                .lamports(from_start),
        );
        let to = any_account_info::<8>(
            AccountConfig::new()
                .rent_exempt(false)
                .writable()
                .lamports(to_start),
        );

        let before = LamportSnapshot::new(&[&from, &to]);
        unsafe {
            **from.lamports.as_ptr() -= delta;
            **to.lamports.as_ptr() += delta;
        }

        kani::assert(
            before.unchanged(&[&from, &to]),
            "transfer changed total lamports",
        );

        let one_lamport: u64 = 1;
        kani::assume(from_start.checked_add(one_lamport).is_some());
        let single = any_account_info::<8>(
            AccountConfig::new()
                .rent_exempt(false)
                .writable()
                .lamports(from_start),
        );
        let snapshot = LamportSnapshot::new(&[&single]);
        unsafe {
            **single.lamports.as_ptr() += one_lamport;
        }
        kani::assert(
            !snapshot.unchanged(&[&single]),
            "snapshot failed to detect lamport mutation",
        );
    }
}
