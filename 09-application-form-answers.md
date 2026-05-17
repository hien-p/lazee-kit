# Application Form Answers

## Short Summary

Lazee Kit is an open-source C-address onboarding and execution kit for Stellar apps: passkey-style smart accounts, sponsored first actions, gift/claim links, SDK/UI components, and safe scoped sessions.

## Long Summary

Lazee Kit helps Stellar apps onboard users into C-address smart accounts without seed phrases, upfront XLM, or repeated low-value signing prompts. The toolkit includes smart account contracts, sponsor relay, GiftVault, SessionRegistry, TypeScript SDK, React UI kit, reference app, docs, tests, observability, and deployment runbooks.

## RFP Selected

SCF RFP Track: C-Address Tooling & Onboarding.

## Problem

C-addresses can improve Stellar UX, but users and developers still face friction around account creation, funding, fees, receipts, session permissions, and app integration. Every app has to rebuild the same onboarding and sponsorship stack.

## Solution

Lazee Kit provides reusable infrastructure for C-address onboarding:

- Create C-address smart accounts with passkey-style UX
- Sponsor account creation and first actions
- Send and claim USDC through gift links
- Provide receipts and status tracking
- Let apps and agents execute through scoped, revocable sessions
- Give developers SDK methods and React components

## Why Stellar

Stellar's low fees, fast finality, stable assets, and Soroban contracts make C-address onboarding especially practical. Sponsored first actions and small USDC claim flows can work economically on Stellar in a way that is harder on high-fee networks.

## Technical Approach

Contracts enforce user authorization, gift escrow, and session policy. Offchain services handle sponsorship, simulation, status tracking, indexing, and route orchestration. Client libraries expose the flows through SDK methods and UI components.

## Decentralization

The relay does not custody user funds or control authorization. User and session permissions are enforced by contracts. Gift funds are escrowed onchain. The code is open source so teams can self-host or fork the infrastructure.

## Infrastructure

Testnet deployment uses Stellar RPC, sponsor relay, indexer/events API, reference app, database for receipts, and observability. Production pilot adds RPC failover, budget controls, rate limits, monitoring, and incident runbooks.

## User Tracking And Privacy

The core system does not require PII. Logs should scrub claim secrets and passkey material. Metrics focus on aggregate reliability: account creation success, claim completion, sponsor spend, session rejects, RPC failures, and indexer lag.

## Open Source

Code will be MIT licensed. Docs and tutorials will be CC BY 4.0. Milestone deliverables will be public before tranche review.

## Maintenance

The team commits to at least 12 months of maintenance after final milestone, including issue triage, monthly releases, security advisories, docs updates, and community support.

## Milestones

- M1: Demo MVP and proof of intent
- M2: Developer kit and integration readiness
- M3: Production readiness and mainnet pilot

## Budget

Requested budget: $128,000 over 20 weeks.

## Community Updates

Weekly build notes during active milestones, public demo after M1, integration guide after M2, mainnet pilot report after M3, and regular updates in Stellar developer/community channels.

## Links To Fill Before Submission

- GitHub:
- Demo:
- Video:
- Contract IDs:
- Transaction receipts:
- Referral code:
- Partner feedback:
