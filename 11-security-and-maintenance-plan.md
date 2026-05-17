# Security And Maintenance Plan

## Threat Model

| Threat | Risk | Mitigation |
|---|---|---|
| Account spoofing | Attacker claims or controls another account | passkey challenge binding, signer registry, nonces |
| Gift double claim | Gift claimed twice | GiftVault status checks and claim events |
| Wrong claim secret | Unauthorized gift claim | hash verification and receiver binding |
| Relay tampering | Relay changes transaction plan | signed intent digest, simulation matching, contract policy |
| Sponsor abuse | Bots drain sponsor budget | rate limits, app quotas, action allowlists, budget alarms |
| Session overreach | Agent exceeds user permission | caps, expiry, allowlists, revocation, counters |
| Replay | Old payload reused | nonces, ledger bounds, digest binding |
| TTL expiration | State becomes inaccessible | keeper jobs, flow-based extension, alerts |
| Indexer inconsistency | UI shows stale state | event replay, direct contract checks for critical paths |
| RPC instability | Transactions fail or duplicate receipts | failover, retries, idempotent status tracking |

## Required Tests

- Authorization matrix
- Nonce and replay tests
- Gift create, claim, double claim, wrong secret, expiry, refund
- Session max per transaction
- Session recurring limit
- Session lifetime limit
- Session expiry
- Session revocation
- Out-of-scope agent call
- Relay budget and rate limits
- RPC failover
- Indexer replay consistency
- Pause and recovery drills

## Operational Runbooks

### Sponsor Relay Abuse

Disable app key, pause affected sponsored action type, preserve logs, rotate sponsor account if needed, publish advisory if user impact exists.

### Contract Bug

Pause affected flows if pause control exists, stop relay submissions, document severity, prepare fix or migration path, publish advisory.

### RPC Degradation

Fail over to secondary RPC, classify pending receipts, retry safely, monitor duplicate submissions.

### Indexer Lag

Switch critical UI state to direct contract reads, throttle non-critical dashboards, replay from last safe cursor.

### Gift Link Phishing

Warn users, verify domain, improve claim preview, rotate public messaging, block known malicious referrers where appropriate.

## Maintenance Commitment

- 12 months after M3
- Monthly release notes
- GitHub issue triage
- Security advisory process
- SDK compatibility updates
- Documentation updates
- Public roadmap
- Community support window after each milestone
