//! CPI contract stubs for formal verification of cross-program invocations.

/// A single recorded CPI call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CpiRecord {
    pub target_program: [u8; 32],
    pub instruction_name: &'static str,
    pub lamports_transferred: u64,
    pub accounts_touched: u8,
}

/// Fixed-size CPI call log. No heap allocation.
pub struct CpiLog {
    records: [Option<CpiRecord>; 16],
    count: usize,
}

/// Helper: produces a `[None; 16]` array in const context without requiring
/// nightly `const { None }` syntax.
const fn none_array() -> [Option<CpiRecord>; 16] {
    [
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None,
    ]
}

impl CpiLog {
    /// Creates an empty log.
    pub const fn new() -> Self {
        Self {
            records: none_array(),
            count: 0,
        }
    }

    /// Appends a CPI record. Panics if the log is full.
    pub fn record(&mut self, r: CpiRecord) {
        assert!(self.count < 16, "CpiLog overflow: max 16 records");
        self.records[self.count] = Some(r);
        self.count += 1;
    }

    /// Number of recorded CPI calls.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Iterates over all recorded CPI calls.
    pub fn iter(&self) -> impl Iterator<Item = &CpiRecord> {
        self.records[..self.count]
            .iter()
            .filter_map(|slot| slot.as_ref())
    }

    /// Returns a record by index.
    pub fn get(&self, index: usize) -> Option<&CpiRecord> {
        if index >= self.count {
            return None;
        }
        self.records[index].as_ref()
    }
}

impl Default for CpiLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Generates a CPI contract stub function with pre/post conditions.
///
/// The generated function executes the precondition body, records the CPI
/// call into a [`CpiLog`], then executes the postcondition body.
///
/// # Example
///
/// ```ignore
/// cpi_stub! {
///     name: token_transfer,
///     program: TOKEN_PROGRAM_ID,
///     pre: |from: &mut AgentAccount, to: &mut AgentAccount, amount: u64| {
///         kani::assume(from.lamports >= amount);
///     },
///     post: |from: &mut AgentAccount, to: &mut AgentAccount, amount: u64| {
///         from.lamports -= amount;
///         to.lamports += amount;
///     },
/// }
/// ```
#[macro_export]
macro_rules! cpi_stub {
    (
        name: $name:ident,
        program: $program:expr,
        pre: |$($pre_arg:ident : $pre_ty:ty),* $(,)?| $pre_body:block,
        post: |$($post_arg:ident : $post_ty:ty),* $(,)?| $post_body:block $(,)?
    ) => {
        fn $name(
            $($pre_arg : $pre_ty,)*
            cpi_log: &mut $crate::agent::cpi::CpiLog,
        ) {
            $pre_body
            cpi_log.record($crate::agent::cpi::CpiRecord {
                target_program: $program,
                instruction_name: stringify!($name),
                lamports_transferred: 0,
                accounts_touched: 0,
            });
            $post_body
        }
    };
}

/// Generates a contract-style CPI stub with explicit requires/body/ensures blocks.
///
/// This macro is intended for larger harnesses where pushing constraints into a
/// `requires` block helps reduce path explosion.
///
/// # Example
///
/// ```ignore
/// cpi_contract! {
///     name: settle_after_timelock,
///     program: ORACLE_PROGRAM,
///     args: |now: i64, expires_at: i64, agent_signed: bool, oracle_signed: bool| {},
///     requires: {
///         kani::assume(expires_at >= 0);
///     },
///     body: {
///         let _authorized = agent_signed || (oracle_signed && now >= expires_at);
///     },
///     ensures: {
///         // Additional postconditions here.
///     },
///     record: {
///         lamports_transferred: 0,
///         accounts_touched: 2,
///     },
///     auto_asserts: {
///         timelock: (now, expires_at, agent_signed, oracle_signed, _authorized);
///     },
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __kamiyo_cpi_contract_auto_asserts {
    () => {};

    (timelock: ($now:expr, $expires_at:expr, $agent_signed:expr, $oracle_signed:expr, $release_allowed:expr); $($rest:tt)*) => {
        $crate::agent::invariants::assume_timelock_well_formed($now, $expires_at);
        $crate::agent::assert_timelock_release_policy(
            $now,
            $expires_at,
            $agent_signed,
            $oracle_signed,
            $release_allowed,
        );
        $crate::__kamiyo_cpi_contract_auto_asserts!($($rest)*);
    };

    (oracle: ($commits:expr, $reveals:expr, $quorum:expr, $median_score:expr, $score_cap:expr); $($rest:tt)*) => {
        $crate::agent::invariants::assume_oracle_well_formed(
            $commits,
            $reveals,
            $quorum,
            $median_score,
            $score_cap,
        );
        $crate::agent::assert_oracle_consensus(
            $commits,
            $reveals,
            $quorum,
            $median_score,
            $score_cap,
        );
        $crate::__kamiyo_cpi_contract_auto_asserts!($($rest)*);
    };

    (oracle_monotonic: ($before:expr, $after:expr); $($rest:tt)*) => {
        $crate::agent::invariants::assume_nondecreasing_u64($before, $after);
        $crate::__kamiyo_cpi_contract_auto_asserts!($($rest)*);
    };

    (fsm: ($before:expr, $after:expr, $valid_edges:expr, $terminal_states:expr); $($rest:tt)*) => {
        $crate::agent::assert_fsm_transition_guard($before, $after, $valid_edges, $terminal_states);
        $crate::__kamiyo_cpi_contract_auto_asserts!($($rest)*);
    };

    (fsm_monotonic: ($before:expr, $after:expr); $($rest:tt)*) => {
        $crate::agent::invariants::assume_nondecreasing_u8($before, $after);
        $crate::__kamiyo_cpi_contract_auto_asserts!($($rest)*);
    };

    ($unknown:tt $($rest:tt)*) => {
        compile_error!("unsupported cpi_contract auto_asserts clause");
    };
}

#[macro_export]
macro_rules! cpi_contract {
    (
        name: $name:ident,
        program: $program:expr,
        args: |$($arg:ident : $arg_ty:ty),* $(,)?| $args_body:block,
        requires: $requires:block,
        body: $body:block,
        ensures: $ensures:block,
        record: {
            lamports_transferred: $lamports_transferred:expr,
            accounts_touched: $accounts_touched:expr $(,)?
        },
        auto_asserts: { $($auto_asserts:tt)* } $(,)?
    ) => {
        fn $name(
            $($arg: $arg_ty,)*
            cpi_log: &mut $crate::agent::cpi::CpiLog,
        ) {
            $args_body
            $requires
            let lamports_transferred: u64 = $lamports_transferred;
            let accounts_touched: u8 = $accounts_touched;
            cpi_log.record($crate::agent::cpi::CpiRecord {
                target_program: $program,
                instruction_name: stringify!($name),
                lamports_transferred,
                accounts_touched,
            });
            $body
            $crate::__kamiyo_cpi_contract_auto_asserts!($($auto_asserts)*);
            $ensures
        }
    };

    (
        name: $name:ident,
        program: $program:expr,
        args: |$($arg:ident : $arg_ty:ty),* $(,)?| $args_body:block,
        requires: $requires:block,
        body: $body:block,
        ensures: $ensures:block,
        record: {
            lamports_transferred: $lamports_transferred:expr,
            accounts_touched: $accounts_touched:expr $(,)?
        } $(,)?
    ) => {
        $crate::cpi_contract! {
            name: $name,
            program: $program,
            args: |$($arg: $arg_ty),*| $args_body,
            requires: $requires,
            body: $body,
            ensures: $ensures,
            record: {
                lamports_transferred: $lamports_transferred,
                accounts_touched: $accounts_touched,
            },
            auto_asserts: {},
        }
    };

    (
        name: $name:ident,
        program: $program:expr,
        args: |$($arg:ident : $arg_ty:ty),* $(,)?| $args_body:block,
        requires: $requires:block,
        body: $body:block,
        ensures: $ensures:block,
        auto_asserts: { $($auto_asserts:tt)* } $(,)?
    ) => {
        $crate::cpi_contract! {
            name: $name,
            program: $program,
            args: |$($arg: $arg_ty),*| $args_body,
            requires: $requires,
            body: $body,
            ensures: $ensures,
            record: {
                lamports_transferred: 0,
                accounts_touched: 0,
            },
            auto_asserts: { $($auto_asserts)* },
        }
    };

    (
        name: $name:ident,
        program: $program:expr,
        args: |$($arg:ident : $arg_ty:ty),* $(,)?| $args_body:block,
        requires: $requires:block,
        body: $body:block,
        ensures: $ensures:block $(,)?
    ) => {
        $crate::cpi_contract! {
            name: $name,
            program: $program,
            args: |$($arg: $arg_ty),*| $args_body,
            requires: $requires,
            body: $body,
            ensures: $ensures,
            record: {
                lamports_transferred: 0,
                accounts_touched: 0,
            },
            auto_asserts: {},
        }
    };
}
