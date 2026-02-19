//! Benchmark-oriented proof harnesses for agent flow checks.

use super::{any_agent_account, assert_lamport_conservation, AgentConfig};

/// Compact end-to-end harness used for CI runtime benchmarking.
#[kani::proof]
pub fn verify_agent_flow_end_to_end() {
    let mut payer = any_agent_account(AgentConfig::new().payer());
    let mut escrow = any_agent_account(AgentConfig::new().writable());
    let mut payee = any_agent_account(AgentConfig::new().writable());

    let amount: u64 = kani::any::<u16>() as u64;
    kani::assume(amount <= payer.lamports);
    kani::assume(escrow.lamports.checked_add(amount).is_some());

    payer.lamports -= amount;
    escrow.lamports += amount;

    let settles: bool = kani::any();
    if settles {
        kani::assume(escrow.lamports >= amount);
        kani::assume(payee.lamports.checked_add(amount).is_some());
        escrow.lamports -= amount;
        payee.lamports += amount;
    }

    assert_lamport_conservation(&[payer, escrow, payee]);
}
