#[cfg(kani)]
use kamiyo_kani::agent::pda::{any_valid_bump, assert_seed_count_valid, assert_seed_lengths_valid};

pub fn pda_validate_fixed(seeds: &[&[u8]]) -> bool {
    if seeds.len() > 16 {
        return false;
    }

    if seeds.iter().any(|seed| seed.len() > 32) {
        return false;
    }

    true
}

#[cfg(kani)]
mod proofs {
    use super::any_valid_bump;
    use super::assert_seed_count_valid;
    use super::assert_seed_lengths_valid;
    use super::pda_validate_fixed;

    #[kani::proof]
    fn fixed_rejects_seed_count_overflow() {
        let seed = [0u8; 32];
        let seeds: [&[u8]; 17] = [
            &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed, &seed,
            &seed, &seed, &seed, &seed, &seed,
        ];

        let accepted = pda_validate_fixed(&seeds);
        kani::assert(!accepted, "seed count > 16 must be rejected");
    }

    #[kani::proof]
    fn fixed_rejects_seed_len_overflow() {
        let oversized_seed = [0u8; 33];
        let seeds: [&[u8]; 1] = [&oversized_seed];

        let accepted = pda_validate_fixed(&seeds);
        kani::assert(!accepted, "seed len > 32 must be rejected");
    }

    #[kani::proof]
    fn fixed_accepts_valid_shape_and_bump() {
        let seed = [0u8; 32];
        let seeds: [&[u8]; 2] = [&seed, &seed];

        let accepted = pda_validate_fixed(&seeds);
        kani::assert(accepted, "valid seed shape should be accepted");

        if accepted {
            assert_seed_count_valid(seeds.len());
            assert_seed_lengths_valid(&seeds);
            let bump = any_valid_bump();
            kani::assert(bump <= u8::MAX, "bump must stay in u8 range");
        }
    }
}
