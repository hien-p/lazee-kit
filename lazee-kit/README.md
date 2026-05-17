# Lazee Kit — Soroban Workspace

Soroban dApp scaffold for the **Lazee Kit** proposal (see `../` for the proposal packet).
Generated with the Stellar contract-init template
(<https://developers.stellar.org/docs/build/guides/dapps/soroban-contract-init-template>):
a Cargo workspace of Rust contracts plus the SDF Astro frontend, wired together by
`initialize.js` (funds an account, builds, deploys, binds TS clients, imports them).

## Contracts

Maps to the containers in `../04-system-architecture.md`:

| Crate | Architecture container | Responsibility |
|---|---|---|
| `contracts/lazee-account` | Lazee Account | C-address smart account: signer registry, authorization, nonces, policy hooks |
| `contracts/gift-vault` | GiftVault | Escrows gift assets, validates claims, handles expiry and refunds |
| `contracts/session-registry` | SessionRegistry | Scoped app/agent grants: caps, expiry, revocation, counters |

Each crate is the contract-init sample (`hello`) — replace the bodies with the
flows described in `../04-system-architecture.md`. `SponsorPolicy` is optional in
the architecture and not yet scaffolded; add it with
`stellar contract init . --name sponsor-policy` when needed.

## Quick start

```bash
# prerequisites: Docker running + local network
stellar network container start local

cd lazee-kit
cp .env.example .env        # set STELLAR_ACCOUNT / network vars
npm install
npm run dev                 # init.js: fund → build → deploy → bind → import, then astro dev
```

Build/test contracts only (no network needed):

```bash
stellar contract build
cargo test
```

## Layout

- `contracts/*` — Rust contract crates (workspace members)
- `initialize.js` — generic build/deploy/bind/import pipeline (auto-discovers all contracts)
- `src/` — Astro frontend; `src/pages/index.astro` calls `lazee-account.hello`
- `packages/` — generated TypeScript contract clients (created by `npm run dev`)
