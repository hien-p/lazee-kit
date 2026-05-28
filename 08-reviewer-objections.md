# Reviewer Objections

## Is this just another wallet?

No. Lazee Kit is not positioned as a standalone wallet. It is an embeddable sponsored-onboarding kit for Stellar apps. The funded outputs are a sponsor helper, SDK, UI components, reference app, tests, docs, and operational runbooks.

## Why should SCF fund this?

Stellar apps repeatedly rebuild the same first-action infrastructure: fee sponsorship, transaction submission, user-readable status, error handling, and support flows. Lazee Kit turns that repeated work into an open-source package that other teams can inspect, fork, and embed.

## Is this a smart-account project?

No. The proposal should not be framed around a smart-account rebuild unless the submitted product actually depends on it. Lazee Kit's core value is sponsored first-action tooling for Stellar apps.

## Is the scope too broad?

The scope is intentionally narrow. M1 proves one sponsored Stellar action and receipt. M2 packages the SDK, UI kit, tests, and docs. M3 hardens the release and runbooks. Advanced flows stay outside the core Build request.

## Is the sponsor helper centralized?

The sponsor helper is operational infrastructure, not an authorization authority. It pays fees, simulates transactions, submits, and tracks receipts. The user still approves the action. Apps can self-host the helper or adapt the reference implementation.

## What prevents sponsor-budget abuse?

The MVP should include app keys, action allowlists, per-user and per-app caps, rate limits, simulation checks, monitoring, and emergency disable paths. Sponsored actions should start capped and expand only after testnet evidence.

## What if RPC or submission fails?

Lazee Kit should classify failures clearly: user canceled, simulation failed, sponsor rejected, RPC failed, transaction pending, transaction failed, or transaction succeeded. The UI should show next steps and the runbooks should explain safe retry behavior.

## How will this be maintained?

Maintenance includes monthly release notes, issue triage, SDK compatibility updates, security advisories, monitoring, public roadmap, and a 12-month commitment after final milestone.

## What exists after the grant?

Open-source sponsor helper, SDK, React UI kit, reference app, docs, tests, diagrams, demo videos, and runbooks. These remain useful even if a single hosted deployment stops.
