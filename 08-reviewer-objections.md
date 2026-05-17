# Reviewer Objections

## Is this just another wallet?

No. Lazee Kit is not positioned as a standalone wallet. It is an embeddable onboarding and execution kit for Stellar apps and wallet teams. The funded outputs are contracts, SDKs, UI components, relay patterns, tests, and documentation.

## Why fund another C-address project?

SCF #41 funded multiple C-address related projects, which confirms the ecosystem need. Lazee Kit differentiates by combining onboarding, sponsorship, gift/claim UX, SDK/UI packaging, receipts, scoped sessions, and operational runbooks in one app-embedded developer layer.

## How is this different from Latch, G2C, or KMP smart account tooling?

Lazee should acknowledge those projects as complementary. The distinction is the product surface:

- Latch/G2C-style projects prove C-address onboarding and funding are important.
- KMP-style SDK work improves developer primitives.
- Lazee packages the full app flow: create, sponsor, claim, receipt, session, revoke, SDK, components, demo, docs.

## Is the scope too broad?

The full architecture is broad, but the milestone plan is staged. M1 is narrow: C-address creation, sponsored first action, gift create/claim, receipt. M2 packages developer tooling. M3 hardens and pilots. Advanced bridge and agent flows are framed as extensions, not the M1 dependency.

## Is the sponsor relay centralized?

The relay is operational infrastructure, not an authorization authority. It pays fees, simulates transactions, submits, and tracks receipts. User authorization and session policy remain onchain. Apps can self-host the relay or use the reference deployment.

## What if passkey support is risky?

M1 should keep the account model small and include an external signer fallback where needed. Mainnet pilot should be capped and limited until tests, threat model, and external review are complete.

## How do you prevent gift link phishing?

Gift links need clear domain, claim preview, no raw private key handling, expiry, receiver binding, and secret scrubbing in logs. The URL alone should not custody funds; it only carries claim material that must pass contract checks.

## How do agents avoid overreach?

Agents execute through scoped sessions. Policies include allowed assets, allowed contracts, intent types, max per transaction, recurring limits, lifetime limits, expiry, receiver allowlists, and revocation. Out-of-scope attempts fail.

## How will this be maintained?

Maintenance includes monthly release notes, issue triage, SDK compatibility updates, security advisories, monitoring, public roadmap, and a 12-month commitment after final milestone.

## What exists after the grant?

Open-source contracts, SDK, React UI kit, reference app, sponsor relay, indexer, docs, tests, diagrams, demo videos, and runbooks. These remain useful even if a single hosted deployment stops.
