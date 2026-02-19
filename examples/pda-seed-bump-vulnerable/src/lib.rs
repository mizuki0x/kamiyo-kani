pub fn pda_validate_vulnerable(seeds: &[&[u8]]) -> bool {
    // Vulnerable: off-by-one seed count and oversized seed allowance.
    seeds.len() <= 17 && seeds.iter().all(|seed| seed.len() <= 64)
}

#[cfg(kani)]
mod proofs {
    use super::pda_validate_vulnerable;

    #[kani::proof]
    fn vulnerable_accepts_invalid_shape() {
        let oversized_seed = [0u8; 33];
        let seeds: [&[u8]; 17] = [
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
            &oversized_seed,
        ];

        let accepted = pda_validate_vulnerable(&seeds);
        kani::assert(!accepted, "invalid PDA seed shape must be rejected");
    }
}
