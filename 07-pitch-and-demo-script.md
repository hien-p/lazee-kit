# Pitch And Demo Script

## 30-Second Pitch

Lazee Kit makes C-addresses usable inside real Stellar apps. A user can create a smart account with passkey-style UX, complete a sponsored first action without XLM, claim USDC through a link, and later grant an agent a capped session without sharing keys. For developers, Lazee ships the contracts, SDK, React components, relay, docs, and reference app needed to embed that flow.

## 1-Minute Pitch

C-addresses can unlock better UX on Stellar, but today they are still too hard to create, fund, and explain. Lazee Kit turns C-address onboarding into reusable infrastructure for apps. The first demo takes a user from no wallet and no XLM to a sponsored C-address action and claimed USDC. The developer kit then packages that into smart account contracts, sponsor relay, gift and claim links, scoped sessions, SDK methods, React components, tests, and runbooks. We are applying to the SCF RFP Track because this directly addresses C-Address Tooling & Onboarding and gives the ecosystem a reusable layer rather than another isolated wallet.

## 3-Minute Pitch

### 0:00-0:20 - Problem

"C-addresses are one of the clearest paths to better Stellar UX, but the first minute is still too hard. Users should not need to understand account types, reserves, funding, and signatures before they can receive value."

### 0:20-0:45 - Product

"Lazee Kit is an open-source onboarding and execution kit. It lets apps create a C-address with passkey-style UX, sponsor the first action, and move USDC through normal product flows like gift and claim links."

### 0:45-1:35 - Demo

Show the reference app:

1. Fresh user, no wallet, no XLM.
2. Create Lazee Account.
3. Sponsor relay simulates and submits.
4. Sender creates USDC gift link.
5. Receiver claims with a new C-address.
6. Receipt and balance appear.

Narration:

"This is the user experience we want app developers to be able to embed. The network details are still real, but they are no longer the first thing a user has to understand."

### 1:35-2:05 - Developer Layer

Show SDK/UI cards:

- create account
- sponsor transaction
- create gift
- claim gift
- list receipt
- create scoped session
- revoke session

"The output is not one app. It is reusable contracts, SDKs, UI components, docs, tests, and runbooks."

### 2:05-2:35 - Safe Sessions

Show DCA permission preview:

"Agents and automations should never need private keys. Lazee sessions have caps, expiry, allowed routes, and revocation enforced by contract policy."

### 2:35-3:00 - Close

"SCF #41 proved that C-address tooling matters. Lazee Kit is the next layer: app-embedded onboarding, sponsorship, claim flows, safe sessions, and developer tooling that can compound across the Stellar ecosystem."

## Demo Close Line

"The win is not that Lazee hides Stellar. The win is that Lazee lets users experience Stellar before they have to understand every Stellar detail."
