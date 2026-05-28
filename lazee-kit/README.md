# Lazee Kit - Soroban Workspace

Soroban dApp scaffold for the **Lazee Kit** proposal (see `../` for the proposal packet).
Generated with the Stellar contract-init template
(<https://developers.stellar.org/docs/build/guides/dapps/soroban-contract-init-template>):
a Cargo workspace of Rust contracts plus the SDF Astro frontend, wired together by
`initialize.js` (funds an account, builds, deploys, binds TS clients, imports them).

## Contracts

The current Build scope should stay focused on sponsored first actions, SDK/UI, reference app, docs, tests, and runbooks. Contract work should stay minimal and only support the demo when it is clearly required.

| Crate | Status | Responsibility |
|---|---|---|
| `contracts/lazee-account` | Experimental scaffold | Early account-helper experiment, not core Build scope |

The remaining crate is still close to the contract-init sample. Implement the sponsor helper, SDK/UI, reference app, and receipt flow first; only promote a contract into the SCF critical path after the demo proves it is required.

## Quick start

```bash
# prerequisites: Docker running + local network
stellar network container start local

cd lazee-kit
cp .env.example .env        # set STELLAR_ACCOUNT / network vars
npm install
npm run dev                 # init.js: fund -> build -> deploy -> bind -> import, then astro dev
```

Build/test contracts only (no network needed):

```bash
stellar contract build
cargo test
```

## Layout

- `contracts/*` - Rust contract crates (workspace members)
- `initialize.js` - generic build/deploy/bind/import pipeline (auto-discovers all contracts)
- `src/` - Astro frontend; `src/pages/index.astro` calls `lazee-account.hello`
- `packages/` - generated TypeScript contract clients (created by `npm run dev`)
