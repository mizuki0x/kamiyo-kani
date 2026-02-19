pub fn can_release_vulnerable(
    _now: i64,
    _expires_at: i64,
    agent_signed: bool,
    oracle_signed: bool,
) -> bool {
    // Vulnerability: oracle signer can release regardless of expiry.
    agent_signed || oracle_signed
}

#[cfg(kani)]
mod proofs {
    use super::can_release_vulnerable;

    #[kani::proof]
    fn vulnerable_allows_oracle_before_expiry() {
        let now: i64 = kani::any();
        let expires_at: i64 = (kani::any::<u32>() as i64) + 1;
        kani::assume(now < expires_at);

        let release_allowed = can_release_vulnerable(now, expires_at, false, true);

        kani::assert(
            !release_allowed,
            "oracle must not release before expiry without agent signature",
        );
    }
}
