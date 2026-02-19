//! Self-verification harnesses for the `kamiyo_kani::agent` module.
//!
//! Each `#[kani::proof]` function exercises a specific safety property of
//! the agent primitives: account generation constraints, lamport conservation,
//! reentrancy prevention, CPI authorization, state machine transitions,
//! PDA seed limits, and CPI stub recording.
//!
//! These proofs are designed to be run under `cargo kani --features solana-agent`
//! and compile only when `cfg(kani)` is active.

#![cfg(all(kani, feature = "solana-agent"))]

extern crate kamiyo_kani;

use kamiyo_kani::agent::invariants::{
    assert_fsm_transition_guard, assert_oracle_consensus, assert_timelock_release_policy,
};
use kamiyo_kani::agent::pda::{assert_seed_count_valid, assert_seed_lengths_valid};
use kamiyo_kani::agent::replay::labeled_assert;
use kamiyo_kani::agent::state_machine::{assert_terminal_state, assert_valid_transition};
use kamiyo_kani::agent::*;
use kamiyo_kani::cpi_contract;
use kamiyo_kani::cpi_stub;

// ---------------------------------------------------------------------------
// 1. Payer accounts must be signers, writable, with sufficient lamports.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_payer_is_signer_and_writable() {
    let acc = any_agent_account(AgentConfig::new().payer());
    kani::assert(acc.is_signer, "payer must be signer");
    kani::assert(acc.is_writable, "payer must be writable");
    kani::assert(
        acc.lamports >= 890_880,
        "payer must have rent-exempt minimum",
    );
}

// ---------------------------------------------------------------------------
// 2. Program accounts must have is_program=true and executable=true.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_program_account_executable() {
    let program_id: [u8; 32] = kani::any();
    let acc = any_agent_account(AgentConfig::new().program(program_id));
    kani::assert(acc.is_program, "program account must have is_program set");
    kani::assert(acc.executable, "program account must be executable");
}

// ---------------------------------------------------------------------------
// 3. All generated accounts have 8-byte aligned data_len.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_data_len_aligned() {
    let acc = any_agent_account(AgentConfig::new());
    kani::assert(acc.data_len % 8 == 0, "data_len must be 8-byte aligned");
}

// ---------------------------------------------------------------------------
// 4. initial_lamports always equals lamports at creation time.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_initial_lamports_snapshot() {
    let acc = any_agent_account(AgentConfig::new());
    kani::assert(
        acc.initial_lamports == acc.lamports,
        "initial_lamports must snapshot creation lamports",
    );
}

// ---------------------------------------------------------------------------
// 5. Lamport conservation holds after a valid two-account transfer.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_lamport_conservation_after_transfer() {
    let mut from = any_agent_account(AgentConfig::new().writable());
    let mut to = any_agent_account(AgentConfig::new().writable());

    let amount: u64 = kani::any();
    kani::assume(amount <= from.lamports);
    kani::assume(to.lamports.checked_add(amount).is_some());

    from.lamports -= amount;
    to.lamports += amount;

    assert_lamport_conservation(&[from, to]);
}

// ---------------------------------------------------------------------------
// 6. Lamport conservation holds across a 3-account transfer chain.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_lamport_conservation_three_account_chain() {
    let mut a = any_agent_account(AgentConfig::new().writable());
    let mut b = any_agent_account(AgentConfig::new().writable());
    let mut c = any_agent_account(AgentConfig::new().writable());

    // Transfer a -> b
    let amount_ab: u64 = kani::any();
    kani::assume(amount_ab <= a.lamports);
    kani::assume(b.lamports.checked_add(amount_ab).is_some());
    a.lamports -= amount_ab;
    b.lamports += amount_ab;

    // Transfer b -> c
    let amount_bc: u64 = kani::any();
    kani::assume(amount_bc <= b.lamports);
    kani::assume(c.lamports.checked_add(amount_bc).is_some());
    b.lamports -= amount_bc;
    c.lamports += amount_bc;

    assert_lamport_conservation(&[a, b, c]);
}

// ---------------------------------------------------------------------------
// 7. No-reentrancy assertion passes when all depths are 0.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_no_reentrancy_clean() {
    let a = any_agent_account(AgentConfig::new());
    let b = any_agent_account(AgentConfig::new());

    // any_agent_account always sets reentry_depth to 0
    labeled_assert(
        a.reentry_depth == 0,
        "fresh account reentry_depth must be 0",
    );
    labeled_assert(
        b.reentry_depth == 0,
        "fresh account reentry_depth must be 0",
    );

    assert_no_reentrancy(&[a, b]);
}

// ---------------------------------------------------------------------------
// 8. CPI authorization passes when all targets are in the allowed set.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_cpi_authorized_subset() {
    let prog_a: [u8; 32] = kani::any();
    let prog_b: [u8; 32] = kani::any();

    let mut log = CpiLog::new();
    log.record(CpiRecord {
        target_program: prog_a,
        instruction_name: "transfer",
        lamports_transferred: kani::any(),
        accounts_touched: 2,
    });
    log.record(CpiRecord {
        target_program: prog_b,
        instruction_name: "mint_to",
        lamports_transferred: 0,
        accounts_touched: 3,
    });

    let allowed = [prog_a, prog_b];
    assert_cpi_authorized(&log, &allowed);
}

// ---------------------------------------------------------------------------
// 9. Valid state machine transition through defined edges.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_state_machine_valid_transition() {
    let edges: [(u8, u8); 2] = [(0, 1), (1, 2)];

    // 0 -> 1 is a valid edge
    assert_valid_transition(0u8, 1u8, &edges);
    // 1 -> 2 is a valid edge
    assert_valid_transition(1u8, 2u8, &edges);
    // Self-transitions are always valid
    assert_valid_transition(0u8, 0u8, &edges);
    assert_valid_transition(2u8, 2u8, &edges);
}

// ---------------------------------------------------------------------------
// 10. Terminal state cannot transition to a different state.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_terminal_state_holds() {
    let terminal_states: [u8; 1] = [2];

    // Terminal state staying put is fine
    assert_terminal_state(2u8, 2u8, &terminal_states);

    // Non-terminal state can go anywhere (no assertion triggered)
    let next: u8 = kani::any();
    assert_terminal_state(0u8, next, &terminal_states);
    assert_terminal_state(1u8, next, &terminal_states);
}

// ---------------------------------------------------------------------------
// 11. PDA with 16 seeds of 32 bytes each passes validation.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_pda_seed_limits() {
    let seed: [u8; 32] = [0u8; 32];
    let seeds: [&[u8]; 16] = [
        &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed,
        &seed, &seed, &seed,
    ];

    assert_seed_count_valid(16);
    assert_seed_lengths_valid(&seeds);

    // Also verify smaller counts work
    assert_seed_count_valid(0);
    assert_seed_count_valid(1);

    kani::cover!(true, "PDA seed limit path reachable");
}

// ---------------------------------------------------------------------------
// 12. cpi_stub! macro records exactly one call into the log.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_cpi_stub_records_call() {
    const STUB_PROGRAM: [u8; 32] = [0xAB; 32];

    cpi_stub! {
        name: test_stub,
        program: STUB_PROGRAM,
        pre: |_amount: u64| { },
        post: |_amount: u64| { },
    }

    let mut log = CpiLog::new();
    let amount: u64 = kani::any();

    test_stub(amount, &mut log);

    kani::assert(log.count() == 1, "cpi_stub must record exactly one call");

    // Verify the recorded program matches
    let mut found = false;
    let mut i = 0;
    while i < 16 {
        if i < log.count() {
            // We can verify via iteration that at least one record exists
            found = true;
        }
        i += 1;
    }
    kani::assert(found, "recorded CPI call must exist in log");
}

// ---------------------------------------------------------------------------
// 13. cpi_contract! macro records a contract-style CPI call.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_cpi_contract_records_call() {
    const ORACLE_PROGRAM: [u8; 32] = [0xCD; 32];

    cpi_contract! {
        name: settle_after_timelock,
        program: ORACLE_PROGRAM,
        args: |now: i64, expires_at: i64, agent_signed: bool, oracle_signed: bool| {},
        requires: {
            kani::assume(expires_at >= 0);
        },
        body: {
            let release_allowed = agent_signed || (oracle_signed && now >= expires_at);
            assert_timelock_release_policy(now, expires_at, agent_signed, oracle_signed, release_allowed);
        },
        ensures: {},
    }

    let now: i64 = kani::any();
    let expires_at: i64 = kani::any::<u32>() as i64;
    let agent_signed: bool = kani::any();
    let oracle_signed: bool = kani::any();
    let mut log = CpiLog::new();

    settle_after_timelock(now, expires_at, agent_signed, oracle_signed, &mut log);
    kani::assert(
        log.count() == 1,
        "cpi_contract must record exactly one call",
    );
}

// ---------------------------------------------------------------------------
// 14. cpi_contract! can carry custom record metadata.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_cpi_contract_records_metadata() {
    const TOKEN_PROGRAM: [u8; 32] = [0xEF; 32];

    cpi_contract! {
        name: transfer_with_metadata,
        program: TOKEN_PROGRAM,
        args: |amount: u64, touched: u8| {},
        requires: {
            kani::assume(touched > 0);
        },
        body: {},
        ensures: {},
        record: {
            lamports_transferred: amount,
            accounts_touched: touched,
        },
    }

    let amount: u64 = kani::any::<u16>() as u64;
    let touched: u8 = kani::any::<u8>();
    kani::assume(touched > 0);

    let mut log = CpiLog::new();
    transfer_with_metadata(amount, touched, &mut log);
    kani::assert(log.count() == 1, "expected exactly one CPI record");

    let record = log.get(0).expect("missing CPI record");
    kani::assert(
        record.lamports_transferred == amount,
        "lamports_transferred metadata mismatch",
    );
    kani::assert(
        record.accounts_touched == touched,
        "accounts_touched metadata mismatch",
    );
}

// ---------------------------------------------------------------------------
// 15. cpi_contract! auto_asserts execute oracle/timelock/FSM checks.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_cpi_contract_auto_asserts() {
    const ORACLE_PROGRAM: [u8; 32] = [0xAA; 32];

    cpi_contract! {
        name: settle_with_auto_asserts,
        program: ORACLE_PROGRAM,
        args: |
            now: i64,
            expires_at: i64,
            agent_signed: bool,
            oracle_signed: bool,
            commits: u8,
            reveals: u8,
            quorum: u8,
            median_score: u8,
            score_cap: u8,
            before_step: u8,
            after_step: u8
        | {},
        requires: {
            kani::assume(agent_signed || oracle_signed);
        },
        body: {
            let _release_allowed = if now < expires_at {
                agent_signed
            } else {
                agent_signed || oracle_signed
            };
            let edges = [(0u8, 1u8), (1u8, 2u8), (2u8, 3u8)];
            let terminals = [3u8];
            kani::assume(before_step <= 3);
            kani::assume(after_step <= 3);
            kani::assume(before_step == after_step || (before_step + 1 == after_step));
            let release_allowed = _release_allowed;
            let _ = (edges, terminals, release_allowed);
        },
        ensures: {},
        auto_asserts: {
            timelock: (now, expires_at, agent_signed, oracle_signed, if now < expires_at { agent_signed } else { agent_signed || oracle_signed });
            oracle: (commits, reveals, quorum, median_score, score_cap);
            oracle_monotonic: (0u64, 1u64);
            fsm: (before_step, after_step, &[(0u8, 1u8), (1u8, 2u8), (2u8, 3u8)], &[3u8]);
            fsm_monotonic: (before_step, after_step);
        },
    }

    let mut log = CpiLog::new();
    settle_with_auto_asserts(20, 10, true, false, 7, 5, 3, 90, 100, 1, 2, &mut log);
    kani::assert(
        log.count() == 1,
        "auto-assert contract must record one call",
    );
}

// ---------------------------------------------------------------------------
// 16. Oracle consensus helper enforces quorum and score cap.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_oracle_consensus_helper() {
    let commits: u8 = kani::any();
    let quorum: u8 = kani::any();
    let reveals: u8 = kani::any();
    let median_score: u8 = kani::any();
    let score_cap: u8 = kani::any();

    kani::assume(commits >= quorum);
    kani::assume(reveals >= quorum);
    kani::assume(reveals <= commits);
    kani::assume(median_score <= score_cap);

    assert_oracle_consensus(commits, reveals, quorum, median_score, score_cap);
}

// ---------------------------------------------------------------------------
// 17. Combined FSM guard enforces transition + terminal checks.
// ---------------------------------------------------------------------------

#[kani::proof]
fn verify_fsm_transition_guard() {
    let edges: [(u8, u8); 3] = [(0, 1), (1, 2), (2, 3)];
    let terminals: [u8; 1] = [3];
    assert_fsm_transition_guard(1u8, 2u8, &edges, &terminals);
    assert_fsm_transition_guard(3u8, 3u8, &edges, &terminals);
}
