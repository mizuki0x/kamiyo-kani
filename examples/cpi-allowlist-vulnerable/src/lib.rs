#[cfg(kani)]
use kamiyo_kani::agent::{assert_cpi_authorized, CpiLog, CpiRecord};

pub fn cpi_gate_vulnerable(_target_program: [u8; 32], allowed_programs: &[[u8; 32]]) -> bool {
    // Vulnerability: any non-empty allowlist passes, ignoring target program.
    !allowed_programs.is_empty()
}

#[cfg(kani)]
mod proofs {
    use super::assert_cpi_authorized;
    use super::cpi_gate_vulnerable;
    use super::CpiLog;
    use super::CpiRecord;

    #[kani::proof]
    fn vulnerable_allows_unauthorized_program() {
        let allowlisted_program = [0x11; 32];
        let unauthorized_program = [0x22; 32];
        let allowed = [allowlisted_program];

        let mut cpi_log = CpiLog::new();
        cpi_log.record(CpiRecord {
            target_program: unauthorized_program,
            instruction_name: "transfer",
            lamports_transferred: 0,
            accounts_touched: 2,
        });

        let should_invoke = cpi_gate_vulnerable(unauthorized_program, &allowed);
        kani::assert(
            should_invoke,
            "vulnerable gate should (incorrectly) permit unauthorized target",
        );

        // Expected failure: log contains unauthorized target.
        assert_cpi_authorized(&cpi_log, &allowed);
    }
}
