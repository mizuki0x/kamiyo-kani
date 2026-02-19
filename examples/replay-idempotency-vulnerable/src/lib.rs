#[derive(Clone, Copy, Debug, Default)]
pub struct ReplayState {
    pub seen: bool,
    pub event_id: u64,
    pub payload_hash: u64,
}

pub fn apply_event_vulnerable(state: &mut ReplayState, event_id: u64, payload_hash: u64) -> bool {
    if !state.seen {
        state.seen = true;
        state.event_id = event_id;
        state.payload_hash = payload_hash;
        return true;
    }

    if state.event_id == event_id {
        // Vulnerability: duplicate event_id is accepted even with conflicting payload.
        return true;
    }

    state.event_id = event_id;
    state.payload_hash = payload_hash;
    true
}

#[cfg(kani)]
mod proofs {
    use super::apply_event_vulnerable;
    use super::ReplayState;
    use kamiyo_kani::agent::replay::labeled_assert;

    #[kani::proof]
    fn vulnerable_accepts_conflicting_duplicate_event_id() {
        let mut state = ReplayState::default();

        let first = apply_event_vulnerable(&mut state, 42, 1001);
        kani::assert(first, "first event should be accepted");

        let second = apply_event_vulnerable(&mut state, 42, 9009);

        // Expected invariant: conflicting duplicate must be rejected.
        // This fails for the vulnerable implementation.
        labeled_assert(!second, "conflicting duplicate event_id must be rejected");
    }
}
