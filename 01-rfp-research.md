# RFP Research

## Target RFP

Lazee Kit targets the **SCF RFP Track: C-Address Tooling & Onboarding**.

The RFP Track requires a submission to address an active RFP directly, show why the team is technically strong, define testable milestones, explain maintenance, include architecture, describe infrastructure, address decentralization and user tracking, and commit to open-source development and community updates.

Sources:

- SCF RFP Track: https://stellar.gitbook.io/scf-handbook/scf-awards/build-award/rfp-track
- SCF awards directory: https://communityfund.stellar.org/awards
- SCF #41 awards page: https://communityfund.stellar.org/awards/recTLIVf9LOTBtkld

## RFP Fit Map

| RFP Need | Lazee Kit Response | Proof Artifact |
|---|---|---|
| Make C-addresses usable for real users | Passkey-style C-address creation and app-embedded onboarding | Reference app demo video |
| Help users fund or use C-addresses without friction | Sponsored first action, gift/claim links, G-to-C funding docs | Testnet receipts, GiftVault flow |
| Provide reusable developer tooling | TypeScript SDK, React UI kit, integration docs | SDK docs, component catalog |
| Provide reference implementations | Demo app for account creation, sponsored action, gift claim, session grant | Public demo URL |
| Support production readiness | tests, observability, runbooks, security review | Test matrix, runbook docs |
| Build in the open | public repo, MIT code, CC BY docs | GitHub repo |
| Maintain after launch | 12-month maintenance plan, issue SLA, monthly updates | Maintenance doc |

## Prior C-Address Landscape

SCF #41 included several C-address related submissions. This matters because Lazee Kit must not sound like a repeat of an already-funded idea.

Awarded examples from the SCF #41 awards page:

- **Latch: C-Address Onboarding** - $120K, awarded
- **G2C** - $150K, awarded
- **C-Address Tooling - Smart accounts** by KMP Stellar SDK - $14K, awarded

Received but not awarded examples:

- **C-Address Funding SDK & Proxy Contract** - $120K, not awarded
- **JS-Capacitor Smart Account Kit** - $10K, not awarded
- **Passkeys & C-Address Tooling** - $112K, not awarded

## Differentiation

Lazee Kit should position itself as a **developer onboarding layer**, not a wallet and not a one-off funding helper.

| Existing Angle | Risk | Lazee Differentiation |
|---|---|---|
| Wallet-only onboarding | Competes with existing wallets and narrows distribution | Lazee is embedded infrastructure for apps and wallets |
| Funding-only C-address tool | Solves the first transfer but not the app experience | Lazee includes sponsored actions, UI, receipts, gift links, sessions |
| Passkey-only account demo | Impressive authentication but not enough product outcome | Lazee moves assets and shows user-readable outcomes |
| Relayer-only approach | Can feel centralized or opaque | Lazee keeps authorization onchain and treats relay as fee infrastructure |
| Agent automation without policy | Unsafe or custodial | Lazee sessions enforce caps, expiry, allowlists, and revocation |

## Core Research Claim

The ecosystem does not need only another way to create a C-address. It needs a repeatable onboarding stack that answers:

- How does the user create it?
- How does the first action get paid?
- How does USDC arrive?
- How does the app explain what happened?
- How does a developer embed the flow?
- How does automation happen without handing over keys?
- How does the system get maintained after grant funding?

Lazee Kit wins by answering all of those in one staged, open-source package.
