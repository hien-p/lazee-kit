# SCF Build Fit Research

## Target Focus

Lazee Kit should be submitted as **Open Track / Developer Tooling**.

The submission should not claim a track or mandate unless the active SCF form explicitly offers that option. The strongest fit is direct ecosystem value: a narrow, testable toolkit for sponsored first actions, transaction status, reusable UI, SDK methods, docs, tests, and operational guidance.

Sources:

- SCF Build Award: https://stellar.gitbook.io/scf-handbook/scf-awards/build-award
- SCF awards directory: https://communityfund.stellar.org/awards

## Fit Map

| Ecosystem Need | Lazee Kit Response | Proof Artifact |
|---|---|---|
| Reduce first-action friction | App sponsors the first meaningful onchain action | Testnet transaction receipts |
| Help developers avoid custom sponsorship plumbing | Minimal sponsor helper plus SDK methods | Public repo and integration docs |
| Make transaction status understandable | Receipt helpers and user-readable status UI | Demo app and screenshots |
| Provide reusable frontend components | React UI kit for onboarding, sponsor status, and receipts | Component catalog |
| Support production readiness | Tests, threat model, rate limits, budget controls, runbooks | Test matrix and runbook docs |
| Build in the open | Public repo, MIT code, CC BY docs | GitHub repo |
| Maintain after launch | 12-month maintenance plan, issue SLA, monthly updates | Maintenance doc |

## Differentiation

Lazee Kit should position itself as a **developer onboarding layer**, not a wallet, not a broad payment app, and not a multi-product automation platform.

| Existing Angle | Risk | Lazee Differentiation |
|---|---|---|
| Wallet-only onboarding | User still meets funding and transaction-status friction inside the app | Lazee focuses on the app's first action and receipt flow |
| Funding-only helper | Pays fees but leaves UI, SDK, and support paths unfinished | Lazee packages sponsor helper, SDK, UI, docs, and runbooks together |
| Relayer-only approach | Can feel centralized or opaque | Lazee treats sponsorship as fee infrastructure with clear limits and receipts |
| Broad automation platform | Too much surface area for an SCF Build application | Lazee keeps advanced execution flows outside the core scope |
| One-off demo | Useful once, hard for teams to reuse | Lazee ships reference code and integration docs |

## Core Research Claim

The ecosystem does not need another vague infrastructure deck. It needs a repeatable onboarding stack that answers:

- What is the first action a new user can complete?
- How is that transaction sponsored safely?
- How does the app explain what happened?
- How does a developer embed the flow?
- How does the system get maintained after grant funding?

Lazee Kit wins by answering those in one staged, open-source package.
