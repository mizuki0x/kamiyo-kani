# x402-svm-agent-payments-fixed

SVM-focused x402-style agentic payment settlement proof.

What the harness proves:
- duplicate request IDs are idempotent only when payload hash matches
- conflicting duplicate request IDs are rejected
- settlement CPI targets only an allowlisted payment program
- lamports are conserved across payer and merchant accounts
- recorded CPI metadata matches the transferred amount

Run:

```bash
cargo kani --manifest-path examples/x402-svm-agent-payments-fixed/Cargo.toml \
  --harness proofs::proof_svm_x402_agentic_payment
```
