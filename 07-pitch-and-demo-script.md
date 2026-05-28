# Pitch And Demo Script

## 30-Second Pitch

Lazee Kit helps Stellar apps get users to their first real onchain action faster. Instead of forcing new users through XLM funding, fee setup, repeated signing confusion, and unclear transaction states, Lazee provides a sponsor helper, SDK, React UI components, receipts, docs, tests, and runbooks that apps can embed.

## 1-Minute Pitch

Stellar is fast and affordable, but many apps still lose users before the first useful action. New users hit funding, fees, wallet prompts, trustlines, and transaction uncertainty before they understand why the app matters.

Lazee Kit turns that first moment into reusable developer tooling. The demo takes a fresh user from opening an app to completing a sponsored Stellar transaction with a readable receipt. The developer kit packages that pattern into SDK methods, UI components, a sponsor helper, reference app, tests, and operational runbooks.

We are applying through SCF Build Open Track / Developer Tooling because this is reusable infrastructure for the Stellar ecosystem, not a one-off consumer app.

## 3-Minute Pitch

### 0:00-0:20 - Problem

"Stellar has strong rails, but the first minute inside many apps is still too technical. Users should not need to understand funding, fees, trustlines, and transaction status before they experience value."

### 0:20-0:45 - Product

"Lazee Kit is an open-source sponsored onboarding kit. It lets apps present one clear Stellar action, sponsor the fee, submit the transaction, and show a receipt through reusable SDK and UI components."

### 0:45-1:35 - Demo

Show the reference app:

1. Fresh user opens the app.
2. App shows one clear action and expected result.
3. User approves with a supported Stellar signer.
4. Sponsor helper simulates, sponsors, submits, and confirms.
5. Receipt appears with tx hash and status.
6. Developer view shows the SDK/UI pieces behind the flow.

Narration:

"This is the user experience we want Stellar app developers to be able to embed. The transaction is still real, but funding and network mechanics are no longer the first thing a user has to solve."

### 1:35-2:05 - Developer Layer

Show SDK/UI cards:

- prepare sponsored action
- simulate transaction
- sponsor and submit
- poll receipt
- render sponsor status
- render receipt and error states

"The output is not one app. It is reusable sponsor infrastructure, SDKs, UI components, docs, tests, and runbooks."

### 2:05-2:35 - Scope Discipline

"We are not asking SCF to fund advanced modules or a smart-account rewrite. The Build request stays focused on the smallest useful developer layer: sponsored first actions and the reusable app components around them."

### 2:35-3:00 - Close

"The win is not that Lazee hides Stellar. The win is that Lazee lets users experience Stellar before they have to understand every operational detail, and gives developers a reusable way to do that safely."

## Demo Close Line

"From first visit to first Stellar action in under 60 seconds."
