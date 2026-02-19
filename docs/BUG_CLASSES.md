# Bug Classes `kamiyo-kani` Targets

This repository focuses on proving a small set of high-value invariants that frequently break in Solana programs.

## 1. Value conservation bugs

- split math that creates or burns value unexpectedly
- transfer paths that fail to preserve lamports across accounts

Proof modules:
- `risk`
- `token`
- `staking`
- `account_info`

## 2. Bounds and overflow bugs

- unchecked arithmetic in PnL and haircut paths
- invalid percentage/basis-point assumptions

Proof modules:
- `bounds`
- `math`
- `risk`

## 3. State machine bugs

- invalid transitions into terminal states
- replay and idempotency semantics that drift under edge input

Proof modules:
- `agent/state_machine`
- `agent/replay`
- `agent/bench`

## 4. PDA and account-shape bugs

- invalid seed count and seed length assumptions
- signer/writable assumptions that break at runtime

Proof modules:
- `agent/pda`
- `agent/account`
- `agent/invariants`
- `agent/cpi` (`cpi_stub!` and `cpi_contract!`)

## 5. AccountInfo mutation bugs

- unauthorized release behavior in time-locked flow
- mutation on failing path

Proof module:
- `account_info`
