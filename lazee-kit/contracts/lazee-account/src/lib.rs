#![no_std]
//! Lazee Account - experimental account helper scaffold.
//!
//! Holds a registry of ed25519 signer public keys and a replay nonce. Implements
//! Soroban custom-account auth (`__check_auth`): every signer in the registry must
//! sign the auth payload. Signer-registry mutations are self-authorized (the account
//! authorizes changes to itself), matching the "user authorization is enforced by
//! the Lazee Account" trust model in 04-system-architecture.md.

use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl, contracttype, crypto::Hash, panic_with_error, vec,
    Bytes, BytesN, Env, Vec,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    /// Vec<BytesN<32>> of authorized ed25519 public keys.
    Signers,
    /// Replay-protection counter.
    Nonce,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NoSigners = 3,
    UnknownSigner = 4,
    BadSignatureCount = 5,
}

#[contracttype]
#[derive(Clone)]
pub struct Signature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Register the initial signer set. Callable once.
    pub fn init(env: Env, signers: Vec<BytesN<32>>) {
        if env.storage().instance().has(&DataKey::Signers) {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }
        if signers.is_empty() {
            panic_with_error!(&env, Error::NoSigners);
        }
        env.storage().instance().set(&DataKey::Signers, &signers);
        env.storage().instance().set(&DataKey::Nonce, &0u64);
    }

    /// Add a signer. Authorized by the account itself (require_auth on current contract).
    pub fn add_signer(env: Env, public_key: BytesN<32>) {
        env.current_contract_address().require_auth();
        let mut signers = Self::signers(&env);
        if !signers.contains(&public_key) {
            signers.push_back(public_key);
            env.storage().instance().set(&DataKey::Signers, &signers);
        }
    }

    /// Remove a signer. The registry must keep at least one signer.
    pub fn remove_signer(env: Env, public_key: BytesN<32>) {
        env.current_contract_address().require_auth();
        let signers = Self::signers(&env);
        let mut next: Vec<BytesN<32>> = vec![&env];
        for s in signers.iter() {
            if s != public_key {
                next.push_back(s);
            }
        }
        if next.is_empty() {
            panic_with_error!(&env, Error::NoSigners);
        }
        env.storage().instance().set(&DataKey::Signers, &next);
    }

    pub fn get_signers(env: Env) -> Vec<BytesN<32>> {
        Self::signers(&env)
    }

    pub fn nonce(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::Nonce).unwrap_or(0)
    }

    fn signers(env: &Env) -> Vec<BytesN<32>> {
        env.storage()
            .instance()
            .get(&DataKey::Signers)
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized))
    }
}

#[contractimpl]
impl CustomAccountInterface for Contract {
    type Signature = Vec<Signature>;
    type Error = Error;

    /// Every registered signer must produce a valid ed25519 signature over the
    /// auth payload. Bumps the nonce on success.
    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signatures: Vec<Signature>,
        _auth_contexts: Vec<Context>,
    ) -> Result<(), Error> {
        let signers: Vec<BytesN<32>> = env
            .storage()
            .instance()
            .get(&DataKey::Signers)
            .ok_or(Error::NotInitialized)?;

        if signatures.len() != signers.len() {
            return Err(Error::BadSignatureCount);
        }

        let payload: Bytes = signature_payload.to_bytes().into();
        for sig in signatures.iter() {
            if !signers.contains(&sig.public_key) {
                return Err(Error::UnknownSigner);
            }
            env.crypto()
                .ed25519_verify(&sig.public_key, &payload, &sig.signature);
        }

        let nonce: u64 = env.storage().instance().get(&DataKey::Nonce).unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::Nonce, &(nonce + 1));
        Ok(())
    }
}

mod test;
