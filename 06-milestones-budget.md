# Milestones And Budget

## Recommended Ask

**$72,000 over 16 weeks**

This ask keeps the scope focused on sponsored onboarding and developer tooling instead of expanding into smart-account work or advanced modules before the core need is proven.

## Tranche Structure

| Milestone | Timeline | Amount | Purpose |
|---|---:|---:|---|
| M1: Sponsored First-Action MVP | Weeks 1-4 | $14,400 | Prove a real sponsored Stellar action and receipt |
| M2: Developer Kit and Integration Readiness | Weeks 5-10 | $28,800 | Package SDK, UI kit, tests, docs, and reference integration |
| M3: Testnet Release and Maintenance Package | Weeks 11-16 | $28,800 | Harden, document, release, and prepare mainnet guidance |

## Milestone 1: Demo MVP And Proof Of Intent

Goal: show a reviewer that Lazee Kit works as a real sponsored-action flow, not a concept deck.

Deliverables:

- Architecture spec and threat model
- Minimal sponsor helper for simulation, fee sponsorship, submission, and receipt polling
- Reference app demo: approve action, sponsor action, submit transaction, view receipt
- Public 3-minute video
- Testnet transaction hashes
- Short test summary

Reviewer verification:

- Demo URL
- GitHub repo
- Transaction hashes
- Sponsor-helper logs or screenshots with private data removed
- Short test summary

## Milestone 2: Developer Kit And Integration Readiness

Goal: turn the demo into reusable developer infrastructure.

Deliverables:

- TypeScript SDK alpha
- React UI kit alpha
- Sponsor helper test suite
- Receipt display and transaction-state helpers backed by Stellar RPC / Horizon
- One reference integration or partner test app
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

- Testnet release hardening
- External review or audit-prep package
- Critical and high issues remediated or documented
- Basic reliability dashboard
- RPC failover and sponsor budget controls
- Mainnet integration guide and capped pilot plan
- Complete runbooks
- Public community demo
- Maintenance plan and roadmap

Reviewer verification:

- Release notes
- Security package
- Monitoring dashboard screenshots
- Runbook links
- Public update posts

## Budget Breakdown

| Category | Amount | Notes |
|---|---:|---|
| Sponsor helper and transaction flow | $16,000 | simulation, sponsorship, submission, polling, status helpers |
| SDK and UI kit | $18,000 | TypeScript SDK, React components, docs, examples |
| Reference app and demo | $10,000 | demo polish, video, public proof artifacts |
| Security and testing | $10,000 | threat model, replay tests, sponsor-abuse tests, review prep |
| Documentation and integration support | $8,000 | integration guide, examples, community support |
| Infrastructure and monitoring | $5,000 | RPC, hosting, observability, error tracking |
| Contingency | $5,000 | provider changes, testnet/mainnet differences, bug fixes |
| **Total** | **$72,000** | |

## Budget Defense

The budget maps to reusable ecosystem tooling. The grant is not paying for a single consumer app or a broad infrastructure stack. It funds the narrow pieces other Stellar teams need to sponsor first actions safely: sponsor helper patterns, SDK methods, UI components, docs, test cases, and operational runbooks.
