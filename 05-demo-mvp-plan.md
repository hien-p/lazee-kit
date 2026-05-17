# Demo MVP Plan

## Demo Title

**From zero wallet to claimed USDC in under 60 seconds.**

## Demo Goal

The demo should prove that Lazee Kit is not just architecture. It should show a complete C-address onboarding journey that a reviewer can understand without installing anything.

## 3-Minute Flow

### Beat 1: No Wallet, No XLM

Open the reference app with a fresh browser profile. The page says:

> No wallet yet? Create a Stellar smart account with a passkey-style flow.

Show that the user does not need seed phrases, a funded account, or upfront XLM.

### Beat 2: Create C-Address

Click "Create Lazee Account." The app shows:

- passkey-style account setup
- sponsor relay status
- generated C-address
- account creation receipt

### Beat 3: Sponsor First Action

Show the sponsor relay status moving through:

1. Simulate
2. Sponsor
3. Submit
4. Confirm

Copy:

> Your first action is sponsored by this app.

### Beat 4: Create Gift Link

Sender creates a USDC gift link with amount and expiry. The UI shows asset, amount, expiry, and refund rule.

### Beat 5: Claim Gift

Receiver opens claim URL in a second browser context. Receiver creates or selects a Lazee C-address and claims.

### Beat 6: Receipt And Balance

Show:

- receiver C-address
- USDC balance
- transaction receipt
- gift status: Claimed
- event history

### Beat 7: Scoped Session

Show a DCA bot permission:

> Allow DCA Bot to swap up to 100 USDC per day for 30 days using approved routes only. The bot cannot transfer to arbitrary addresses and can be revoked anytime.

Create the session, show active policy, revoke it, and show an attempted out-of-scope action failing.

## Why This Looks Better

| Alternative | Demo Weakness | Lazee Demo Advantage |
|---|---|---|
| Wallet-only onboarding | User still needs to understand wallet setup | App embeds the flow directly |
| Passkey-only demo | No value moves | User claims USDC and gets receipt |
| Funding-only tool | First transfer works, but app experience remains incomplete | Lazee includes UI, SDK, sponsor, gifts, receipts |
| Generic relayer | Trust model unclear | Relay pays fees but contracts enforce authorization |
| Agent key sharing | Unsafe | Scoped sessions are capped and revocable |

## Demo Artifacts

- Public demo URL
- 3-minute video
- Contract IDs
- Transaction hashes
- Architecture diagram
- SDK/UI component screenshot
- Test summary
