# Program interface

## Accounts

- Config: authority, factory, relayer, Arcium verifier.
- Vault: treasury, threshold, member count, signer root/version, proposal count, status.
- Proposal: payload hash, root/version snapshots, counts, status, execution result, and fee protections.
- Nullifier: vault, proposal, action domain, root version, and nullifier hash.

## Product configurations

- Personal Vault: one member and threshold one.
- Private Multisig: multiple members and a valid threshold.

## Instructions

Initialize config; create vault; create proposal; record private approve/reject action; execute transfer, protected withdrawal, threshold, profile, or signer-set proposal.

## CPI boundaries

Protected withdrawal invokes the pinned ZK Pool program to record a funded note. Private actions validate receipts owned by the pinned Arcium verifier program.

## Known boundary issue

The adjacent ZK Pool fee-receipt consume instruction does not independently bind its signer named `factory` to this program's configured factory. That issue is not hidden by this repository and remains under review.
