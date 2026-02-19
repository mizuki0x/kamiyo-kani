#[cfg(kani)]
use kamiyo_kani::agent::assert_timelock_release_policy;

pub fn can_release_fixed(
    now: i64,
    expires_at: i64,
    agent_signed: bool,
    oracle_signed: bool,
) -> bool {
    agent_signed || (oracle_signed && now >= expires_at)
}

#[cfg(kani)]
mod proofs {
    use super::assert_timelock_release_policy;
    use super::can_release_fixed;

    #[kani::proof]
    fn fixed_blocks_oracle_before_expiry() {
        let now: i64 = kani::any();
        let expires_at: i64 = (kani::any::<u32>() as i64) + 1;
        kani::assume(now < expires_at);

        let release_allowed = can_release_fixed(now, expires_at, false, true);
        assert_timelock_release_policy(now, expires_at, false, true, release_allowed);
        kani::assert(!release_allowed, "oracle must be blocked before expiry");
    }

    #[kani::proof]
    fn fixed_allows_oracle_after_expiry() {
        let now: i64 = kani::any::<u32>() as i64;
        let expires_at: i64 = kani::any::<u16>() as i64;
        kani::assume(now >= expires_at);

        let release_allowed = can_release_fixed(now, expires_at, false, true);
        assert_timelock_release_policy(now, expires_at, false, true, release_allowed);
        kani::assert(release_allowed, "oracle signer may release after expiry");
    }
}
