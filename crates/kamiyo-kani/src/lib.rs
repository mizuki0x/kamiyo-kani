//! Reusable Kani formal verification harnesses for Solana program math.
//!
//! This crate is intentionally a no-op in normal builds. Everything is gated
//! behind `cfg(kani)`.

#[cfg(kani)]
pub mod generators;

#[cfg(kani)]
pub mod token;

#[cfg(kani)]
pub mod staking;

#[cfg(kani)]
pub mod bounds;

#[cfg(kani)]
pub mod math;

#[cfg(kani)]
pub mod risk;

#[cfg(all(kani, feature = "solana-agent"))]
pub mod agent;

#[cfg(all(kani, feature = "solana-account-info"))]
pub mod account_info;

#[cfg(test)]
fn coverage_probe(value: u8) -> u8 {
    value.saturating_add(1)
}

#[cfg(test)]
mod tests {
    use super::coverage_probe;

    #[test]
    fn coverage_probe_saturates() {
        assert_eq!(coverage_probe(0), 1);
        assert_eq!(coverage_probe(u8::MAX), u8::MAX);
    }
}
