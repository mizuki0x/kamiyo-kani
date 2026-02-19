# autonomous-payment-oracle-fixed

End-to-end agent flow example for an x402-style autonomous payment oracle.

What it proves in one harness:
- oracle quote acceptance (quorum/cap/round monotonicity)
- idempotent replay semantics for x402 event IDs
- timelock release policy
- allowed CPI target enforcement
- lamport conservation and FSM progression

Run:

```bash
cargo kani --manifest-path examples/autonomous-payment-oracle-fixed/Cargo.toml \
  --harness proofs::proof_autonomous_payment_oracle_flow
```
