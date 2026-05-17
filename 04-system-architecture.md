# System Architecture

## Architecture Goal

Lazee Kit turns C-address onboarding into a reusable system. The architecture separates user authorization, fee sponsorship, asset movement, session policy, receipt indexing, and app integration.

## Containers

| Container | Technology | Responsibility |
|---|---|---|
| Lazee Account | Soroban Rust | C-address smart account, signer registry, authorization, nonces, policy hooks |
| GiftVault | Soroban Rust | Escrows gift assets, validates claims, handles expiry and refunds |
| SessionRegistry | Soroban Rust | Stores scoped app/agent grants, caps, expiry, revocation, counters |
| SponsorPolicy | Soroban Rust, optional | Defines approved sponsored actions and app policy metadata |
| Sponsor Relay | Node/Bun service | Simulates, sponsors, submits, polls, and records transactions |
| Intent Orchestrator | Node/Bun service | Normalizes send, swap, bridge, gift, claim, recurring, DCA intents |
| Indexer/Events API | Worker plus database | Rebuilds account, gift, session, sponsor, and receipt state |
| Lazee SDK | TypeScript | Client API for account, sponsor, gift, session, intent, receipt flows |
| React UI Kit | React | Drop-in onboarding, gift, claim, session, receipt, and risk components |
| Reference App | Web app | Reviewer demo and developer integration example |

## Trust Model

- User authorization is enforced by the Lazee Account and related contracts.
- Sponsor relay pays fees but cannot bypass user or session authorization.
- Gift links do not custody funds by themselves; funds sit in GiftVault.
- Agents receive scoped session credentials, never user private keys.
- Session policies enforce allowed assets, contracts, intent types, caps, expiry, and revocation.
- Indexer state is convenience state; critical actions can be verified against contract state and events.

## Core Flows

### C-Address Onboarding

1. User opens a Lazee-enabled app.
2. App starts passkey-style account creation.
3. SDK builds account initialization request.
4. Sponsor relay simulates and sponsors the transaction.
5. Lazee Account stores signer and emits account event.
6. Indexer records account metadata and receipt.

### Gift Create And Claim

1. Sender selects asset, amount, expiry, and optional message.
2. SDK creates gift intent.
3. Sender authorizes transaction.
4. GiftVault escrows funds and emits GiftCreated.
5. Receiver opens claim URL.
6. Receiver creates or selects a Lazee C-address.
7. Claim transaction proves secret and binds receiver address.
8. GiftVault transfers funds and emits GiftClaimed.

### Scoped Session

1. User reviews permission copy.
2. User authorizes session creation.
3. SessionRegistry stores policy and counters.
4. App or agent executes within scope.
5. Contract rejects attempts beyond caps, expiry, asset allowlist, or contract allowlist.
6. User can revoke session at any time.

## Storage And TTL Strategy

Persistent state includes signer records, session policies, spend counters, gift records, and app registry metadata. Lazee Kit should treat Soroban storage TTL as a production concern:

- Extend account state on login and execution.
- Extend active session state during execution and keeper runs.
- Extend gift state until expiry plus refund window.
- Rebuild UI state from events when possible.
- Alert when important entries approach expiration.

## Operational Architecture

Production deployments should include:

- Primary and secondary Stellar RPC providers
- Sponsor budget caps by app, user, and action type
- Rate limits by app key and IP
- Simulation before sponsor signature
- Receipt polling and retry classification
- Error taxonomy for user cancellations, RPC failures, contract rejects, and sponsor rejects
- Dashboards for account creation, gift claim, session execution, sponsor spend, RPC failover, and indexer lag
