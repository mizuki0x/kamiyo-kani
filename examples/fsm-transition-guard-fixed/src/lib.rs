#[cfg(kani)]
use kamiyo_kani::agent::assert_fsm_transition_guard;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EscrowState {
    Init,
    Funded,
    Revealed,
    Settled,
}

pub fn can_transition_fixed(before: EscrowState, after: EscrowState) -> bool {
    before == after
        || matches!(
            (before, after),
            (EscrowState::Init, EscrowState::Funded)
                | (EscrowState::Funded, EscrowState::Revealed)
                | (EscrowState::Revealed, EscrowState::Settled)
        )
}

#[cfg(kani)]
mod proofs {
    use super::{assert_fsm_transition_guard, can_transition_fixed, EscrowState};

    fn model() -> ([(EscrowState, EscrowState); 3], [EscrowState; 1]) {
        let valid_edges = [
            (EscrowState::Init, EscrowState::Funded),
            (EscrowState::Funded, EscrowState::Revealed),
            (EscrowState::Revealed, EscrowState::Settled),
        ];
        let terminal_states = [EscrowState::Settled];
        (valid_edges, terminal_states)
    }

    #[kani::proof]
    fn fixed_rejects_skip_to_terminal() {
        let before = EscrowState::Funded;
        let after = EscrowState::Settled;

        let allowed = can_transition_fixed(before, after);
        kani::assert(!allowed, "must reject skipping state-machine steps");
    }

    #[kani::proof]
    fn fixed_rejects_terminal_exit() {
        let before = EscrowState::Settled;
        let after = EscrowState::Funded;

        let allowed = can_transition_fixed(before, after);
        kani::assert(!allowed, "must reject terminal->non-terminal transition");
    }

    #[kani::proof]
    fn fixed_accepts_valid_progression() {
        let before = EscrowState::Funded;
        let after = EscrowState::Revealed;

        let allowed = can_transition_fixed(before, after);
        kani::assert(allowed, "valid transition should be accepted");

        if allowed {
            let (valid_edges, terminal_states) = model();
            assert_fsm_transition_guard(before, after, &valid_edges, &terminal_states);
        }
    }
}
