#[cfg(kani)]
use kamiyo_kani::agent::{
    any_agent_account, assert_cpi_authorized, assert_fsm_transition_guard,
    assert_lamport_conservation, AgentConfig, CpiLog,
};
#[cfg(kani)]
use kamiyo_kani::cpi_contract;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaymentState {
    QuoteRequested,
    QuoteLocked,
    Settled,
}

impl PaymentState {
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::QuoteRequested => 0,
            Self::QuoteLocked => 1,
            Self::Settled => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PaymentIntent {
    pub intent_id: u64,
    pub payload_hash: u64,
    pub expires_at: i64,
    pub quote_lamports: u64,
    pub oracle_round: u64,
    pub state: PaymentState,
}

pub fn apply_x402_event(event_id: u64, payload_hash: u64, seen: &mut Option<(u64, u64)>) -> bool {
    match seen {
        None => {
            *seen = Some((event_id, payload_hash));
            true
        }
        Some((seen_id, seen_hash)) => {
            if *seen_id != event_id {
                *seen = Some((event_id, payload_hash));
                return true;
            }
            *seen_hash == payload_hash
        }
    }
}

pub fn quote_accepted(
    intent: &mut PaymentIntent,
    now: i64,
    commits: u8,
    reveals: u8,
    quorum: u8,
    median_score: u8,
    score_cap: u8,
    next_round: u64,
) -> bool {
    let accepted = now <= intent.expires_at
        && reveals <= commits
        && reveals >= quorum
        && median_score <= score_cap
        && next_round >= intent.oracle_round;

    if accepted {
        intent.quote_lamports = median_score as u64;
        intent.oracle_round = next_round;
        intent.state = PaymentState::QuoteLocked;
    }

    accepted
}

#[cfg(kani)]
mod proofs {
    use super::*;

    const TOKEN_PROGRAM: [u8; 32] = [0x42; 32];

    cpi_contract! {
        name: invoke_x402_settlement,
        program: TOKEN_PROGRAM,
        args: |
            payer: &mut kamiyo_kani::agent::AgentAccount,
            merchant: &mut kamiyo_kani::agent::AgentAccount,
            amount: u64,
            now: i64,
            expires_at: i64,
            agent_signed: bool,
            oracle_signed: bool,
            commits: u8,
            reveals: u8,
            quorum: u8,
            median_score: u8,
            score_cap: u8,
            round_before: u64,
            round_after: u64,
            before_step: u8,
            after_step: u8
        | {
            let _ = (
                now,
                expires_at,
                agent_signed,
                oracle_signed,
                commits,
                reveals,
                quorum,
                median_score,
                score_cap,
                round_before,
                round_after,
                before_step,
                after_step,
            );
        },
        requires: {
            kani::assume(amount > 0);
            kani::assume(amount <= payer.lamports);
            kani::assume(merchant.lamports.checked_add(amount).is_some());
            kani::assume(before_step == 1);
            kani::assume(after_step == 2);
            kani::assume(agent_signed || oracle_signed);
        },
        body: {
            payer.lamports -= amount;
            merchant.lamports += amount;
        },
        ensures: {
            kani::assert(
                merchant.lamports >= merchant.initial_lamports,
                "merchant balance must not decrease",
            );
        },
        record: {
            lamports_transferred: amount,
            accounts_touched: 2,
        },
        auto_asserts: {
            timelock: (
                now,
                expires_at,
                agent_signed,
                oracle_signed,
                if now < expires_at {
                    agent_signed
                } else {
                    agent_signed || oracle_signed
                }
            );
            oracle: (commits, reveals, quorum, median_score, score_cap);
            oracle_monotonic: (round_before, round_after);
            fsm: (before_step, after_step, &[(0u8, 1u8), (1u8, 2u8)], &[2u8]);
            fsm_monotonic: (before_step, after_step);
        },
    }

    #[kani::proof]
    fn proof_autonomous_payment_oracle_flow() {
        let mut payer = any_agent_account(AgentConfig::new().payer());
        let mut merchant = any_agent_account(AgentConfig::new().writable());

        let now: i64 = kani::any::<u16>() as i64;
        let expires_at: i64 = now + 10;
        let commits: u8 = 6;
        let reveals: u8 = 4;
        let quorum: u8 = 3;
        let median_score: u8 = kani::any::<u8>();
        let score_cap: u8 = 100;
        kani::assume(median_score <= score_cap);

        let intent_id: u64 = kani::any::<u16>() as u64;
        let payload_hash: u64 = kani::any::<u16>() as u64;
        let mut intent = PaymentIntent {
            intent_id,
            payload_hash,
            expires_at,
            quote_lamports: 0,
            oracle_round: 7,
            state: PaymentState::QuoteRequested,
        };

        let accepted = quote_accepted(
            &mut intent,
            now,
            commits,
            reveals,
            quorum,
            median_score,
            score_cap,
            8,
        );
        kani::assert(accepted, "quote should be accepted in valid flow");

        kani::assume(intent.quote_lamports > 0);
        kani::assume(intent.quote_lamports <= payer.lamports);

        let mut seen_event: Option<(u64, u64)> = None;
        kani::assert(
            apply_x402_event(intent.intent_id, intent.payload_hash, &mut seen_event),
            "first x402 event must be accepted",
        );
        kani::assert(
            apply_x402_event(intent.intent_id, intent.payload_hash, &mut seen_event),
            "identical duplicate x402 event must be accepted",
        );
        kani::assert(
            !apply_x402_event(intent.intent_id, intent.payload_hash + 1, &mut seen_event),
            "conflicting duplicate x402 event must be rejected",
        );

        let mut cpi_log = CpiLog::new();
        invoke_x402_settlement(
            &mut payer,
            &mut merchant,
            intent.quote_lamports,
            now,
            expires_at,
            true,
            false,
            commits,
            reveals,
            quorum,
            median_score,
            score_cap,
            7,
            8,
            PaymentState::QuoteLocked.as_u8(),
            PaymentState::Settled.as_u8(),
            &mut cpi_log,
        );

        intent.state = PaymentState::Settled;

        assert_cpi_authorized(&cpi_log, &[TOKEN_PROGRAM]);
        assert_lamport_conservation(&[payer, merchant]);
        assert_fsm_transition_guard(
            PaymentState::QuoteLocked.as_u8(),
            intent.state.as_u8(),
            &[(0u8, 1u8), (1u8, 2u8)],
            &[2u8],
        );
    }
}
