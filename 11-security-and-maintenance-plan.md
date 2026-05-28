# Security And Maintenance Plan

## Threat Model

| Threat | Risk | Mitigation |
|---|---|---|
| Transaction-plan tampering | User approves one action but helper submits another | signed/approved transaction preview, simulation matching, exact operation display |
| Sponsor abuse | Bots drain sponsor budget | app keys, action allowlists, rate limits, user caps, app quotas, budget alarms |
| Replay | Old payload reused | ledger bounds, idempotency keys, digest binding, duplicate submission checks |
| RPC instability | Transactions fail or duplicate receipts | failover, retries, idempotent status tracking |
| Receipt mismatch | UI shows success or failure incorrectly | verify status through Stellar RPC / Horizon and transaction hashes |
| Leaky logs | Signing material, tokens, or private data enters logs | log redaction, structured logging, secret scanning |
| Misconfigured app integration | App sponsors unintended actions | configuration validation, safe defaults, test fixtures, deployment checklist |
| Sponsor key compromise | Attacker spends sponsor funds | limited sponsor balances, rotation process, alerts, emergency disable path |

## Required Tests

- Transaction-plan preview and simulation matching
- Ledger-bound and replay tests
- Sponsor budget and rate-limit tests
- Allowed-action and rejected-action tests
- RPC failover and retry tests
- Receipt status consistency tests
- Log redaction checks
- Emergency disable and sponsor-key rotation drill
- Integration fixture tests for SDK methods and UI states

## Operational Runbooks

### Sponsor Abuse

Disable app key, pause affected sponsored action type, preserve logs, rotate sponsor account if needed, publish advisory if user impact exists.

### Sponsor Key Incident

Disable sponsorship, move remaining funds, rotate keys, review logs, publish incident note if any user-facing action was affected.

### RPC Degradation

Fail over to secondary RPC, classify pending receipts, retry safely, monitor duplicate submissions.

### Receipt Mismatch

Switch UI to network-verified status, mark affected receipts as pending review, replay transaction checks from known hashes, publish correction if needed.

### Integration Misconfiguration

Disable the app key, compare configured allowlist with submitted operations, ship config fix, add fixture coverage for the failed case.

## Maintenance Commitment

- 12 months after M3
- Monthly release notes
- GitHub issue triage
- Security advisory process
- SDK compatibility updates
- Documentation updates
- Public roadmap
- Community support window after each milestone
