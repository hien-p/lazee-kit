#![no_std]
//! GiftVault — escrows gift assets, validates claims, handles expiry and refunds.
//!
//! Flow (04-system-architecture.md → "Gift Create And Claim"):
//!   create_gift  — sender escrows a token amount behind sha256(secret), with expiry
//!   claim_gift   — anyone presenting the secret before expiry receives the funds
//!   refund       — after expiry the sender reclaims unclaimed funds
//!
//! Funds sit in this contract, never in the link itself, matching the trust model.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, token, Address,
    Bytes, BytesN, Env,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    NextId,
    Gift(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    GiftNotFound = 1,
    AlreadySettled = 2,
    Expired = 3,
    NotExpired = 4,
    BadSecret = 5,
    NotSender = 6,
    BadAmount = 7,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Open,
    Claimed,
    Refunded,
}

#[contracttype]
#[derive(Clone)]
pub struct Gift {
    pub sender: Address,
    pub token: Address,
    pub amount: i128,
    pub claim_hash: BytesN<32>,
    pub expiry: u64,
    pub status: Status,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Sender escrows `amount` of `token`. `claim_hash` = sha256(secret).
    /// `expiry` is a ledger timestamp; after it, only refund is allowed.
    pub fn create_gift(
        env: Env,
        sender: Address,
        token: Address,
        amount: i128,
        claim_hash: BytesN<32>,
        expiry: u64,
    ) -> u64 {
        sender.require_auth();
        if amount <= 0 {
            panic_with_error!(&env, Error::BadAmount);
        }
        if expiry <= env.ledger().timestamp() {
            panic_with_error!(&env, Error::Expired);
        }

        token::TokenClient::new(&env, &token).transfer(
            &sender,
            &env.current_contract_address(),
            &amount,
        );

        let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(0);
        let gift = Gift {
            sender: sender.clone(),
            token,
            amount,
            claim_hash,
            expiry,
            status: Status::Open,
        };
        env.storage().persistent().set(&DataKey::Gift(id), &gift);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));

        env.events().publish(("gift", "created"), (id, sender, amount));
        id
    }

    /// Present the secret before expiry; funds go to `receiver`.
    pub fn claim_gift(env: Env, id: u64, secret: Bytes, receiver: Address) {
        let mut gift = Self::load(&env, id);
        if gift.status != Status::Open {
            panic_with_error!(&env, Error::AlreadySettled);
        }
        if env.ledger().timestamp() >= gift.expiry {
            panic_with_error!(&env, Error::Expired);
        }
        if env.crypto().sha256(&secret).to_bytes() != gift.claim_hash {
            panic_with_error!(&env, Error::BadSecret);
        }

        token::TokenClient::new(&env, &gift.token).transfer(
            &env.current_contract_address(),
            &receiver,
            &gift.amount,
        );

        gift.status = Status::Claimed;
        env.storage().persistent().set(&DataKey::Gift(id), &gift);
        env.events()
            .publish(("gift", "claimed"), (id, receiver, gift.amount));
    }

    /// After expiry the original sender reclaims the escrowed funds.
    pub fn refund(env: Env, id: u64) {
        let mut gift = Self::load(&env, id);
        gift.sender.require_auth();
        if gift.status != Status::Open {
            panic_with_error!(&env, Error::AlreadySettled);
        }
        if env.ledger().timestamp() < gift.expiry {
            panic_with_error!(&env, Error::NotExpired);
        }

        token::TokenClient::new(&env, &gift.token).transfer(
            &env.current_contract_address(),
            &gift.sender,
            &gift.amount,
        );

        gift.status = Status::Refunded;
        env.storage().persistent().set(&DataKey::Gift(id), &gift);
        env.events()
            .publish(("gift", "refunded"), (id, gift.sender.clone()));
    }

    pub fn get_gift(env: Env, id: u64) -> Gift {
        Self::load(&env, id)
    }

    fn load(env: &Env, id: u64) -> Gift {
        env.storage()
            .persistent()
            .get(&DataKey::Gift(id))
            .unwrap_or_else(|| panic_with_error!(env, Error::GiftNotFound))
    }
}

mod test;
