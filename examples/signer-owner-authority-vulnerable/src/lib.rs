pub fn can_mutate_authority_vulnerable(
    authority_is_signer: bool,
    authority_owner: [u8; 32],
    expected_owner: [u8; 32],
) -> bool {
    // Vulnerability: owner/signer checks bypassed.
    let _ = authority_owner;
    let _ = expected_owner;
    authority_is_signer || true
}

#[cfg(kani)]
mod proofs {
    use super::can_mutate_authority_vulnerable;
    use kamiyo_kani::agent::invariants::{assert_is_signer, assert_owner};
    use kamiyo_kani::agent::{any_agent_account, AgentConfig};

    #[kani::proof]
    fn vulnerable_allows_unsigned_wrong_owner_authority() {
        let expected_owner = [0xAA; 32];
        let wrong_owner = [0xBB; 32];

        let authority = any_agent_account(AgentConfig::new().owner(wrong_owner));
        kani::assume(!authority.is_signer);

        let allowed =
            can_mutate_authority_vulnerable(authority.is_signer, authority.owner, expected_owner);
        kani::assert(allowed, "vulnerable path should incorrectly allow mutation");

        // Expected failure: authority must be signer and expected owner.
        assert_is_signer(&authority);
        assert_owner(&authority, &expected_owner);
    }
}
