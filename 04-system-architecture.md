# System Architecture

## Architecture Goal

Lazee Kit turns sponsored first actions into a reusable system. The architecture separates user approval, fee sponsorship, transaction submission, receipt display, and app integration.

## Containers

| Container | Technology | Responsibility |
|---|---|---|
| Reference App | Web app | Reviewer demo and developer integration example |
| Lazee SDK | TypeScript | Client API for sponsored actions, receipt status, and integration helpers |
| React UI Kit | React | Drop-in onboarding, sponsor status, receipt, and error-state components |
| Sponsor Helper | Node/Bun service | Simulates, sponsors, submits, and polls transactions |
| Receipt Store | Lightweight database, optional | Convenience cache for demo status and analytics |
| Soroban Examples | Soroban Rust, optional | Small reference contracts only when a demo action needs a contract target |

Advanced modules such as claim links, sessions, orchestration, indexing, and smart-account-specific account work should not be presented as required Build scope. They are too broad for the current application unless a later milestone explicitly proves the need.

## Trust Model

- The user approves the action with their wallet or supported signer.
- Sponsor helper pays fees but cannot replace user approval.
- The helper does not custody user assets and does not become the source of truth.
- Receipt state is convenience state; critical actions are verified through Stellar RPC / Horizon and transaction hashes.
- Apps can self-host the sponsor helper and configure their own budgets, limits, and allowed actions.

## Core Flows

### Sponsored First Action

1. User opens a Lazee-enabled app.
2. App presents one clearly described Stellar action.
3. User approves the action with their wallet or supported signer.
4. SDK builds the transaction plan and expected result.
5. Sponsor helper simulates the transaction.
6. Sponsor helper applies fee sponsorship and submits.
7. Reference app displays success/failure, tx hash, receipt, and next-step guidance.

### Developer Integration

1. Developer installs the SDK and React UI kit.
2. Developer configures allowed actions, sponsor budget, and RPC settings.
3. App calls SDK methods to prepare, sponsor, submit, and poll.
4. UI components render sponsor progress, failure states, and receipt details.
5. Runbooks explain operational handling for RPC failures, abuse, and budget limits.

## Storage And TTL Strategy

Persistent onchain state should stay minimal. The MVP should prefer transaction hashes, RPC / Horizon reads, and lightweight receipt state over a custom event service.

- Store only what is required for demo status and support.
- Rebuild critical UI state from network reads when possible.
- Alert on sponsor budget, failed submissions, and RPC degradation.

## Operational Architecture

Production deployments should include:

- Primary and secondary Stellar RPC providers
- Sponsor budget caps by app, user, and action type
- Rate limits by app key and IP
- Simulation before sponsor signature
- Receipt polling and retry classification
- Error taxonomy for user cancellations, RPC failures, rejected transactions, and sponsor rejects
- Dashboards for sponsored attempts, sponsor spend, RPC failures, and completion rate
