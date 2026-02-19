#[cfg(kani)]
use kamiyo_kani::agent::{
    any_agent_account, assert_cpi_authorized, assert_fsm_transition_guard,
    assert_lamport_conservation, AgentConfig, CpiLog,
};
#[cfg(kani)]
use kamiyo_kani::cpi_contract;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettlementState {
    Requested,
    Settled,
}

impl SettlementState {
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Requested => 0,
            Self::Settled => 1,
        }
    }
}

pub fn apply_x402_request(
    request_id: u64,
    payload_hash: u64,
    seen: &mut Option<(u64, u64)>,
) -> bool {
    match seen {
        None => {
            *seen = Some((request_id, payload_hash));
            true
        }
        Some((seen_id, seen_hash)) => {
            if *seen_id != request_id {
                *seen = Some((request_id, payload_hash));
                return true;
            }
            *seen_hash == payload_hash
        }
    }
}

#[cfg(kani)]
mod proofs {
    use super::*;

    const PAYMENT_PROGRAM: [u8; 32] = [0xA4; 32];

    cpi_contract! {
        name: settle_svm_payment,
        program: PAYMENT_PROGRAM,
        args: |
            payer: &mut kamiyo_kani::agent::AgentAccount,
            merchant: &mut kamiyo_kani::agent::AgentAccount,
            amount: u64,
            before_state: u8,
            after_state: u8,
            before_nonce: u64,
            after_nonce: u64
        | {
            let _ = (before_state, after_state, before_nonce, after_nonce);
        },
        requires: {
            kani::assume(amount > 0);
            kani::assume(amount <= payer.lamports);
            kani::assume(merchant.lamports.checked_add(amount).is_some());
            kani::assume(before_state == SettlementState::Requested.as_u8());
            kani::assume(after_state == SettlementState::Settled.as_u8());
        },
        body: {
            payer.lamports -= amount;
            merchant.lamports += amount;
        },
        ensures: {
            kani::assert(
                merchant.lamports >= merchant.initial_lamports,
                "merchant balance must not decrease",
            );
        },
        record: {
            lamports_transferred: amount,
            accounts_touched: 2,
        },
        auto_asserts: {
            fsm: (
                before_state,
                after_state,
                &[(SettlementState::Requested.as_u8(), SettlementState::Settled.as_u8())],
                &[SettlementState::Settled.as_u8()]
            );
            oracle_monotonic: (before_nonce, after_nonce);
            fsm_monotonic: (before_state, after_state);
        },
    }

    #[kani::proof]
    fn proof_svm_x402_agentic_payment() {
        let mut payer = any_agent_account(AgentConfig::new().payer());
        let mut merchant = any_agent_account(AgentConfig::new().writable());

        let amount: u64 = kani::any::<u16>() as u64;
        kani::assume(amount > 0);
        kani::assume(amount <= payer.lamports);
        kani::assume(merchant.lamports.checked_add(amount).is_some());

        let request_id: u64 = kani::any::<u16>() as u64;
        let payload_hash: u64 = kani::any::<u16>() as u64;
        let mut seen_request: Option<(u64, u64)> = None;

        kani::assert(
            apply_x402_request(request_id, payload_hash, &mut seen_request),
            "first x402 request should be accepted",
        );
        kani::assert(
            apply_x402_request(request_id, payload_hash, &mut seen_request),
            "identical duplicate x402 request should be accepted",
        );
        kani::assert(
            !apply_x402_request(request_id, payload_hash + 1, &mut seen_request),
            "conflicting duplicate x402 request should be rejected",
        );

        let mut cpi_log = CpiLog::new();
        settle_svm_payment(
            &mut payer,
            &mut merchant,
            amount,
            SettlementState::Requested.as_u8(),
            SettlementState::Settled.as_u8(),
            11,
            12,
            &mut cpi_log,
        );

        assert_cpi_authorized(&cpi_log, &[PAYMENT_PROGRAM]);
        assert_lamport_conservation(&[payer, merchant]);
        assert_fsm_transition_guard(
            SettlementState::Requested.as_u8(),
            SettlementState::Settled.as_u8(),
            &[(SettlementState::Requested.as_u8(), SettlementState::Settled.as_u8())],
            &[SettlementState::Settled.as_u8()],
        );

        let record = cpi_log
            .get(0)
            .expect("first CPI call should be recorded");
        kani::assert(
            record.lamports_transferred == amount,
            "recorded lamports must equal settled amount",
        );
    }
}
