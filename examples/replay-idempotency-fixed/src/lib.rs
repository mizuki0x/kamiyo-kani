#[derive(Clone, Copy, Debug, Default)]
pub struct ReplayState {
    pub seen: bool,
    pub event_id: u64,
    pub payload_hash: u64,
}

pub fn apply_event_fixed(state: &mut ReplayState, event_id: u64, payload_hash: u64) -> bool {
    if !state.seen {
        state.seen = true;
        state.event_id = event_id;
        state.payload_hash = payload_hash;
        return true;
    }

    if state.event_id == event_id {
        // Idempotent replay: accept exact duplicate, reject conflicting payload.
        return state.payload_hash == payload_hash;
    }

    state.event_id = event_id;
    state.payload_hash = payload_hash;
    true
}

#[cfg(kani)]
mod proofs {
    use super::apply_event_fixed;
    use super::ReplayState;
    use kamiyo_kani::agent::replay::labeled_assert;

    #[kani::proof]
    fn fixed_rejects_conflicting_duplicate_event_id() {
        let mut state = ReplayState::default();

        let first = apply_event_fixed(&mut state, 42, 1001);
        kani::assert(first, "first event should be accepted");

        let second = apply_event_fixed(&mut state, 42, 9009);
        labeled_assert(!second, "conflicting duplicate event_id must be rejected");
    }

    #[kani::proof]
    fn fixed_accepts_identical_duplicate_event_id() {
        let mut state = ReplayState::default();

        let first = apply_event_fixed(&mut state, 42, 1001);
        kani::assert(first, "first event should be accepted");

        let second = apply_event_fixed(&mut state, 42, 1001);
        labeled_assert(second, "identical duplicate event_id should be accepted");
    }

    #[kani::proof]
    fn fixed_accepts_new_event_id() {
        let mut state = ReplayState::default();

        let first = apply_event_fixed(&mut state, 42, 1001);
        kani::assert(first, "first event should be accepted");

        let second = apply_event_fixed(&mut state, 43, 1002);
        labeled_assert(second, "new event_id should be accepted");
    }
}
