# Lazee Kit Proposal Packet

Lazee Kit is an SCF Build proposal packet for **Stellar app onboarding and sponsored first actions**. It frames Lazee Kit as an open-source developer kit that helps Stellar apps let users complete a first onchain action without dealing with XLM funding, fee setup, or custom transaction plumbing upfront.

This repository is intentionally proposal-first. The root folder contains the SCF proposal packet, architecture, research, pitch, and design-system context. The nested `lazee-kit/` folder contains early experimental scaffolds and should not define the grant scope by itself.

## Reading Order

1. [Executive Brief](00-executive-brief.md)
2. [SCF Build Fit Research](01-rfp-research.md)
3. [Stellar Impact Thesis](02-stellar-impact-thesis.md)
4. [Winning Proposal](03-winning-proposal.md)
5. [System Architecture](04-system-architecture.md)
6. [Demo MVP Plan](05-demo-mvp-plan.md)
7. [Milestones and Budget](06-milestones-budget.md)
8. [Pitch and Demo Script](07-pitch-and-demo-script.md)
9. [Reviewer Objections](08-reviewer-objections.md)
10. [Application Form Answers](09-application-form-answers.md)
11. [Community and Adoption Plan](10-community-and-adoption-plan.md)
12. [Security and Maintenance Plan](11-security-and-maintenance-plan.md)
13. [Design System](design-system/README.md)

## Target Program

- Program: Stellar Community Fund Build Award
- Track: Open Track / Developer Tooling
- Focus: sponsored onboarding, SDK/UI, reference app, docs, tests, and runbooks for Stellar apps
- Round: SCF #44
- Default ask: $72,000 in XLM
- Timeline: 16 weeks

## Core Positioning

> Lazee Kit lets a Stellar app sponsor a user's first meaningful action and show a clear receipt. Developers get reusable SDK methods, React UI components, a reference app, tests, and runbooks instead of rebuilding onboarding, sponsorship, and transaction-status flows from scratch.

## Scope Guardrails

The proposal should not claim a track unless the active SCF form explicitly offers one. It should also avoid over-scoping into advanced modules or smart-account-specific work unless those become directly required by the shipped MVP.

## What This Folder Must Not Contain

- No private keys, API keys, or sponsor credentials
- No generated build artifacts
- No unreviewed production deployments
- No public attribution strings from code-generation tools

## Source Links

- SCF Build Award: https://stellar.gitbook.io/scf-handbook/scf-awards/build-award
- SCF awards directory: https://communityfund.stellar.org/awards
