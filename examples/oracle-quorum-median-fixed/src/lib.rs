#[cfg(kani)]
use kamiyo_kani::agent::assert_oracle_consensus;

pub fn consensus_fixed(commits: u8, reveals: u8, quorum: u8, median: u8, cap: u8) -> bool {
    reveals <= commits && reveals >= quorum && median <= cap
}

#[cfg(kani)]
mod proofs {
    use super::assert_oracle_consensus;
    use super::consensus_fixed;

    #[kani::proof]
    fn fixed_rejects_insufficient_reveals() {
        let commits = 5u8;
        let reveals = 1u8;
        let quorum = 3u8;
        let median = 50u8;
        let cap = 100u8;

        let accepted = consensus_fixed(commits, reveals, quorum, median, cap);
        kani::assert(!accepted, "insufficient reveals must be rejected");
    }

    #[kani::proof]
    fn fixed_rejects_median_over_cap() {
        let commits = 5u8;
        let reveals = 5u8;
        let quorum = 3u8;
        let median = 200u8;
        let cap = 100u8;

        let accepted = consensus_fixed(commits, reveals, quorum, median, cap);
        kani::assert(!accepted, "median over cap must be rejected");
    }

    #[kani::proof]
    fn fixed_accepts_valid_consensus() {
        let commits = 7u8;
        let reveals = 5u8;
        let quorum = 3u8;
        let median = 88u8;
        let cap = 100u8;

        let accepted = consensus_fixed(commits, reveals, quorum, median, cap);
        kani::assert(accepted, "valid consensus state should be accepted");

        if accepted {
            assert_oracle_consensus(commits, reveals, quorum, median, cap);
        }
    }
}
