#[cfg(kani)]
use kamiyo_kani::agent::assert_oracle_consensus;

pub fn consensus_vulnerable(commits: u8, reveals: u8, _quorum: u8, median: u8, cap: u8) -> bool {
    // Vulnerability: only checks reveals <= commits, ignores quorum and cap.
    reveals <= commits && median <= u8::MAX && cap <= u8::MAX
}

#[cfg(kani)]
mod proofs {
    use super::assert_oracle_consensus;
    use super::consensus_vulnerable;

    #[kani::proof]
    fn vulnerable_accepts_insufficient_reveals() {
        let commits = 5u8;
        let reveals = 1u8;
        let quorum = 3u8;
        let median = 50u8;
        let cap = 100u8;

        let accepted = consensus_vulnerable(commits, reveals, quorum, median, cap);
        kani::assert(accepted, "vulnerable logic should accept this invalid case");

        // Expected failure: reveals < quorum should be rejected.
        assert_oracle_consensus(commits, reveals, quorum, median, cap);
    }

    #[kani::proof]
    fn vulnerable_accepts_median_over_cap() {
        let commits = 5u8;
        let reveals = 5u8;
        let quorum = 3u8;
        let median = 200u8;
        let cap = 100u8;

        let accepted = consensus_vulnerable(commits, reveals, quorum, median, cap);
        kani::assert(accepted, "vulnerable logic should accept this invalid case");

        // Expected failure: median > cap should be rejected.
        assert_oracle_consensus(commits, reveals, quorum, median, cap);
    }
}
