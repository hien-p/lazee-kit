# Lazee Kit: Winning Proposal Draft

## Project Name

**Lazee Kit**

## Tagline

**Lazy easy execution for Stellar apps.**

## Executive Summary

Lazee Kit is an open-source C-address onboarding and execution toolkit for Stellar applications. It gives developers a reusable way to create app experiences where users can create smart accounts with passkey-style UX, complete a first action without upfront XLM, send or claim USDC through links, and grant safe scoped automation sessions without handing over private keys.

The project is intentionally not positioned as a standalone wallet. Lazee Kit is an embeddable developer layer: smart account contracts, sponsor relay, gift vault, session registry, SDK, React UI kit, reference app, docs, tests, and operational runbooks.

## RFP Addressed

Lazee Kit addresses the **C-Address Tooling & Onboarding** RFP under the SCF RFP Track.

## Problem

C-addresses can unlock better UX on Stellar, but they remain difficult for users and developers:

- Users still face account type complexity, reserves, funding, fees, signatures, and wallet setup.
- Apps must assemble onboarding, sponsorship, session grants, receipt tracking, and support flows from scratch.
- Existing approaches often solve only one piece: wallet UI, funding, passkeys, or account contracts.
- Safe automation is hard because agents and apps need permissions without broad key access.

The result is a first-minute experience that feels too technical for mainstream payments, gaming, creator, consumer, and agent products.

## Why Stellar

Stellar is the right network for this because it combines low-cost settlement, fast finality, stable assets, and Soroban smart contracts. A C-address onboarding kit is especially valuable on Stellar because sponsored first actions and USDC claim flows can be practical at small transaction sizes.

Lazee Kit is Stellar-native. It is built around C-address smart accounts, Stellar assets, Soroban contracts, Stellar RPC, and the Stellar account model. It is not a chain-agnostic abstraction.

## Solution

Lazee Kit ships five primitives as one reusable package:

1. **Passkey-style C-address account creation** for user-friendly smart accounts.
2. **Sponsor relay** for first actions, simulation, submission, and fee sponsorship.
3. **Gift and claim links** for USDC flows that receivers can claim without prior wallet setup.
4. **Scoped sessions** for app and agent automation with caps, expiry, allowlists, and revocation.
5. **Developer SDK and React UI kit** so apps can embed the flows without rebuilding them.

## Why Lazee Is Better

Lazee Kit is broader than a wallet, more outcome-driven than a passkey demo, and more reusable than a relayer. It pairs user-facing UX with developer-facing infrastructure.

The first milestone proves the user-facing value: create C-address, sponsor first action, create gift, claim gift, view receipt. The later milestones package that into reusable developer infrastructure, sessions, observability, and mainnet pilot readiness.

## Technical Stack

- Contracts: Soroban Rust smart account, session registry, gift vault, optional sponsor policy
- Client: TypeScript SDK
- UI: React UI kit
- Backend services: sponsor relay, intent orchestrator, indexer/events API
- Data: PostgreSQL or managed database for offchain receipts and operational state
- Observability: OpenTelemetry-compatible traces, dashboards, error tracking
- Network path: testnet first, mainnet pilot behind caps and feature flags

## Decentralization

Lazee Kit uses offchain infrastructure where it improves UX, but keeps user authorization anchored in smart contracts. The sponsor relay pays fees and submits transactions; it does not control user assets. Session permissions are enforced by onchain policy. Gift funds are escrowed in the GiftVault contract, not by a centralized URL service.

The project is open source so apps can self-host relay and indexer components or use a hosted deployment during early adoption.

## Infrastructure

The MVP runs on:

- Stellar testnet contracts
- Sponsor relay service
- Intent/status API
- Event indexer
- Reference app
- Public docs and dashboards

Production readiness includes multiple RPC providers, sponsor budget limits, rate limits, abuse controls, monitoring, and incident runbooks.

## Privacy And User Tracking

Lazee Kit minimizes user tracking. It does not require PII for the core demo. Operational logs should scrub claim secrets and avoid storing raw passkey material. Analytics should focus on aggregate reliability and adoption metrics: account creation success, claim completion, sponsor spend, session rejection reasons, RPC failures, and indexer lag.

## Open Source

Recommended license:

- Code: MIT
- Docs and tutorials: CC BY 4.0
- Diagrams and design assets: CC BY 4.0 unless a third-party asset requires different treatment

## Maintenance

The team commits to:

- Public roadmap
- Monthly release notes
- GitHub issue triage
- Security advisory process
- SDK and UI kit maintenance for at least 12 months after final milestone
- Community updates through SCF and Stellar developer channels

## Closing Ask

Lazee Kit requests **$128,000 over 20 weeks** to ship a reusable, open-source C-address onboarding and execution layer for Stellar apps. The result is not one isolated application; it is infrastructure that can help many Stellar apps onboard users faster and more safely.
