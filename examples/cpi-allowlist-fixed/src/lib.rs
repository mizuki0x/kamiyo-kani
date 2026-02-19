#[cfg(kani)]
use kamiyo_kani::agent::{assert_cpi_authorized, CpiLog};
#[cfg(kani)]
use kamiyo_kani::cpi_contract;

pub fn cpi_gate_fixed(target_program: [u8; 32], allowed_programs: &[[u8; 32]]) -> bool {
    allowed_programs
        .iter()
        .any(|allowed| *allowed == target_program)
}

#[cfg(kani)]
mod proofs {
    use super::assert_cpi_authorized;
    use super::cpi_contract;
    use super::cpi_gate_fixed;
    use super::CpiLog;

    const ALLOWLISTED_PROGRAM: [u8; 32] = [0x55; 32];

    cpi_contract! {
        name: invoke_allowlisted_cpi,
        program: ALLOWLISTED_PROGRAM,
        args: |amount: u64| {
            let _ = amount;
        },
        requires: {
            kani::assume(amount <= 10_000);
        },
        body: {},
        ensures: {},
    }

    #[kani::proof]
    fn fixed_rejects_unauthorized_program() {
        let unauthorized_program = [0x99; 32];
        let allowed = [ALLOWLISTED_PROGRAM];

        let should_invoke = cpi_gate_fixed(unauthorized_program, &allowed);
        kani::assert(
            !should_invoke,
            "fixed gate must reject unauthorized program",
        );
    }

    #[kani::proof]
    fn fixed_allows_allowlisted_contract() {
        let allowed = [ALLOWLISTED_PROGRAM];
        let mut cpi_log = CpiLog::new();
        let amount: u64 = kani::any::<u16>() as u64;

        let should_invoke = cpi_gate_fixed(ALLOWLISTED_PROGRAM, &allowed);
        kani::assert(should_invoke, "allowlisted program should be permitted");

        if should_invoke {
            invoke_allowlisted_cpi(amount, &mut cpi_log);
        }

        assert_cpi_authorized(&cpi_log, &allowed);
    }
}
