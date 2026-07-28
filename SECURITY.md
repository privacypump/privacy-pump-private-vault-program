# Security policy

## Status

This repository is experimental devnet program. Publication is not a security audit, and no external audit is claimed.

## Coordinated disclosure

Do not open a public issue containing a vulnerability, secret, private wallet relationship, member information, transaction linkage, or exploit instructions. Use GitHub private vulnerability reporting if enabled for this repository. If it is unavailable, wait for the verified Privacy Pump security contact to be published in the organization profile.

Do not send seed phrases, private keys, API keys, session tokens, access packages, chat contents, or production logs.

## Supported versions

No production-supported version is currently declared. Devnet and reference tags may change.

## Public trust assumptions

- Inspected devnet programs are upgradeable and shared one upgrade authority at the audit date.
- Factory and relayer are separate roles in the protocol, but deployments may use one signer.
- Production signer topology is private.
- ZK fee-receipt factory binding and verifier completeness remain under review.
- Arcium and transport infrastructure have independent liveness, privacy, and correctness assumptions.
- Browser and RPC metadata can reduce privacy.

## Out of scope

This repository does not authorize testing against production services, wallets, infrastructure, users, or third-party networks.
