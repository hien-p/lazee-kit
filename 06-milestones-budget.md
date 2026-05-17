# Milestones And Budget

## Recommended Ask

**$128,000 over 20 weeks**

This ask is large enough to cover real infrastructure work, but below the $150K cap. It signals ambition without looking like a maximum-by-default request.

## Tranche Structure

| Milestone | Timeline | Amount | Purpose |
|---|---:|---:|---|
| M1: Demo MVP and Proof of Intent | Weeks 1-4 | $12,800 | Prove core C-address onboarding and sponsored gift claim |
| M2: Developer Kit and Integration Readiness | Weeks 5-12 | $51,200 | Package SDK, UI kit, sessions, indexer, tests, integrations |
| M3: Production Readiness and Mainnet Pilot | Weeks 13-20 | $64,000 | Harden, review, monitor, pilot, document, maintain |

## Milestone 1: Demo MVP And Proof Of Intent

Goal: show a reviewer that Lazee Kit works as a real C-address onboarding flow, not a concept deck.

Deliverables:

- Architecture spec and threat model
- Lazee Account v1 deployed on testnet
- Sponsor relay MVP for account creation and first action
- GiftVault MVP with create, claim, expiry, refund path
- Reference app demo: create C-address, sponsor first action, create gift, claim gift
- Public 3-minute video
- Testnet transaction hashes and contract IDs

Reviewer verification:

- Demo URL
- GitHub repo
- Contract IDs
- Transaction hashes
- Short test summary

## Milestone 2: Developer Kit And Integration Readiness

Goal: turn the demo into reusable developer infrastructure.

Deliverables:

- TypeScript SDK alpha
- React UI kit alpha
- SessionRegistry v1
- Intent schemas for send, swap, bridge_in, gift_create, gift_claim, gift_refund, recurring_payment, DCA
- Indexer/events API
- Account, gift, relay, and session test suite
- Two reference integrations or partner test apps
- Integration docs and component guide

Reviewer verification:

- SDK docs
- Component screenshots
- Test results
- Integration guides
- Partner/test app links

## Milestone 3: Production Readiness And Mainnet Pilot

Goal: make Lazee Kit credible for controlled mainnet usage and post-grant maintenance.

Deliverables:

- Production hardening
- External review or audit-prep package
- Critical and high issues remediated or documented
- Observability dashboard
- RPC failover and sponsor budget controls
- Mainnet pilot behind caps and allowlists
- Complete runbooks
- Public community demo
- Maintenance plan and roadmap

Reviewer verification:

- Mainnet pilot address or deployment notes
- Security package
- Monitoring dashboard screenshots
- Runbook links
- Public update posts

## Budget Breakdown

| Category | Amount | Notes |
|---|---:|---|
| Soroban contracts | $30,000 | Lazee Account, GiftVault, SessionRegistry, SponsorPolicy specs and tests |
| SDK and UI kit | $24,000 | TypeScript SDK, React components, docs, examples |
| Sponsor relay and indexer | $22,000 | simulation, sponsorship, receipts, event replay, status APIs |
| Security and testing | $18,000 | threat model, authorization tests, replay tests, external review prep |
| Demo, docs, and community | $16,000 | reference app polish, video, pitch, integration guides, updates |
| Infrastructure and monitoring | $8,000 | RPC, hosting, database, observability, error tracking |
| Contingency | $10,000 | provider changes, testnet/mainnet differences, bug fixes |
| **Total** | **$128,000** | |

## Budget Defense

The budget maps to reusable ecosystem infrastructure. The grant is not paying for a single consumer app. It funds open-source primitives that other Stellar teams can adopt: contracts, relay patterns, SDK methods, UI components, docs, test cases, and operational runbooks.
