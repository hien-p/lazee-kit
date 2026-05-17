#![no_std]
//! SessionRegistry — scoped app/agent grants with caps, expiry, revocation, counters.
//!
//! Flow (04-system-architecture.md → "Scoped Session"):
//!   create_session — owner authorizes a policy (asset allowlist, spend cap, expiry)
//!   record_spend   — the granted app/agent reports spend; rejected beyond scope
//!   revoke         — owner kills the session at any time
//!
//! The registry enforces allowed assets, caps, and expiry; agents never hold keys.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, Env, Vec,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    NextId,
    Session(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    SessionNotFound = 1,
    Revoked = 2,
    Expired = 3,
    AssetNotAllowed = 4,
    CapExceeded = 5,
    BadInput = 6,
    NotGrantee = 7,
}

#[contracttype]
#[derive(Clone)]
pub struct Session {
    pub owner: Address,
    pub grantee: Address,
    pub assets: Vec<Address>,
    pub cap: i128,
    pub spent: i128,
    pub expiry: u64,
    pub active: bool,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Owner grants `grantee` the right to spend up to `cap` total across
    /// `assets` until `expiry`.
    pub fn create_session(
        env: Env,
        owner: Address,
        grantee: Address,
        assets: Vec<Address>,
        cap: i128,
        expiry: u64,
    ) -> u64 {
        owner.require_auth();
        if cap <= 0 || assets.is_empty() || expiry <= env.ledger().timestamp() {
            panic_with_error!(&env, Error::BadInput);
        }

        let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(0);
        let session = Session {
            owner: owner.clone(),
            grantee: grantee.clone(),
            assets,
            cap,
            spent: 0,
            expiry,
            active: true,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Session(id), &session);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));

        env.events()
            .publish(("session", "created"), (id, owner, grantee));
        id
    }

    /// The grantee reports spend against the session. Enforces active state,
    /// expiry, asset allowlist, and the cumulative cap.
    pub fn record_spend(env: Env, id: u64, asset: Address, amount: i128) {
        let mut s = Self::load(&env, id);
        s.grantee.require_auth();

        if !s.active {
            panic_with_error!(&env, Error::Revoked);
        }
        if env.ledger().timestamp() >= s.expiry {
            panic_with_error!(&env, Error::Expired);
        }
        if !s.assets.contains(&asset) {
            panic_with_error!(&env, Error::AssetNotAllowed);
        }
        if amount <= 0 || s.spent + amount > s.cap {
            panic_with_error!(&env, Error::CapExceeded);
        }

        s.spent += amount;
        env.storage().persistent().set(&DataKey::Session(id), &s);
        env.events()
            .publish(("session", "spend"), (id, asset, amount));
    }

    /// Owner revokes the session immediately.
    pub fn revoke(env: Env, id: u64) {
        let mut s = Self::load(&env, id);
        s.owner.require_auth();
        s.active = false;
        env.storage().persistent().set(&DataKey::Session(id), &s);
        env.events().publish(("session", "revoked"), id);
    }

    pub fn get_session(env: Env, id: u64) -> Session {
        Self::load(&env, id)
    }

    fn load(env: &Env, id: u64) -> Session {
        env.storage()
            .persistent()
            .get(&DataKey::Session(id))
            .unwrap_or_else(|| panic_with_error!(env, Error::SessionNotFound))
    }
}

mod test;
