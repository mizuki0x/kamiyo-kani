#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EscrowState {
    Init,
    Funded,
    Revealed,
    Settled,
}

pub fn can_transition_vulnerable(before: EscrowState, after: EscrowState) -> bool {
    let _ = after;
    // Vulnerability: any non-terminal state can jump to any state.
    before != EscrowState::Settled
}

#[cfg(kani)]
mod proofs {
    use super::{can_transition_vulnerable, EscrowState};
    use kamiyo_kani::agent::assert_fsm_transition_guard;

    #[kani::proof]
    fn vulnerable_allows_skip_to_terminal() {
        let valid_edges = [
            (EscrowState::Init, EscrowState::Funded),
            (EscrowState::Funded, EscrowState::Revealed),
            (EscrowState::Revealed, EscrowState::Settled),
        ];
        let terminal_states = [EscrowState::Settled];

        let before = EscrowState::Funded;
        let after = EscrowState::Settled;

        let allowed = can_transition_vulnerable(before, after);
        kani::assert(
            allowed,
            "vulnerable path incorrectly allows skipping directly to terminal",
        );

        // Expected failure: funded->settled is not in valid_edges.
        assert_fsm_transition_guard(before, after, &valid_edges, &terminal_states);
    }
}
