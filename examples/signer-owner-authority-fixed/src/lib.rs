pub fn can_mutate_authority_fixed(
    authority_is_signer: bool,
    authority_owner: [u8; 32],
    expected_owner: [u8; 32],
) -> bool {
    authority_is_signer && authority_owner == expected_owner
}

#[cfg(kani)]
mod proofs {
    use super::can_mutate_authority_fixed;
    use kamiyo_kani::agent::invariants::{assert_is_signer, assert_owner};
    use kamiyo_kani::agent::{any_agent_account, AgentConfig};

    #[kani::proof]
    fn fixed_rejects_unsigned_authority() {
        let expected_owner = [0xAA; 32];
        let authority = any_agent_account(AgentConfig::new().owner(expected_owner));
        kani::assume(!authority.is_signer);

        let allowed =
            can_mutate_authority_fixed(authority.is_signer, authority.owner, expected_owner);
        kani::assert(!allowed, "unsigned authority must be rejected");
    }

    #[kani::proof]
    fn fixed_rejects_wrong_owner() {
        let expected_owner = [0xAA; 32];
        let wrong_owner = [0xBB; 32];

        let authority = any_agent_account(AgentConfig::new().signer().owner(wrong_owner));
        let allowed =
            can_mutate_authority_fixed(authority.is_signer, authority.owner, expected_owner);
        kani::assert(!allowed, "wrong owner must be rejected");
    }

    #[kani::proof]
    fn fixed_accepts_signed_expected_owner() {
        let expected_owner = [0xAA; 32];
        let authority = any_agent_account(AgentConfig::new().signer().owner(expected_owner));

        let allowed =
            can_mutate_authority_fixed(authority.is_signer, authority.owner, expected_owner);
        kani::assert(allowed, "valid authority should be accepted");

        if allowed {
            assert_is_signer(&authority);
            assert_owner(&authority, &expected_owner);
        }
    }
}
