#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Bytes, Env};

fn setup() -> (Env, ContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    let token = sac.address();
    let mint = soroban_sdk::token::StellarAssetClient::new(&env, &token);

    let sender = Address::generate(&env);
    mint.mint(&sender, &1_000);

    let id = env.register(Contract, ());
    let client = ContractClient::new(&env, &id);
    (env, client, token, sender, id)
}

fn hash(env: &Env, secret: &Bytes) -> BytesN<32> {
    env.crypto().sha256(secret).to_bytes()
}

#[test]
fn create_then_claim() {
    let (env, client, token, sender, cid) = setup();
    let secret = Bytes::from_slice(&env, b"open-sesame");
    let receiver = Address::generate(&env);

    let gid = client.create_gift(&sender, &token, &500, &hash(&env, &secret), &1000);

    let tok = soroban_sdk::token::TokenClient::new(&env, &token);
    assert_eq!(tok.balance(&cid), 500);
    assert_eq!(tok.balance(&sender), 500);

    client.claim_gift(&gid, &secret, &receiver);
    assert_eq!(tok.balance(&receiver), 500);
    assert_eq!(client.get_gift(&gid).status, Status::Claimed);
}

#[test]
#[should_panic]
fn wrong_secret_rejected() {
    let (env, client, token, sender, _) = setup();
    let secret = Bytes::from_slice(&env, b"right");
    let gid = client.create_gift(&sender, &token, &100, &hash(&env, &secret), &1000);
    client.claim_gift(&gid, &Bytes::from_slice(&env, b"wrong"), &Address::generate(&env));
}

#[test]
fn refund_after_expiry() {
    let (env, client, token, sender, _) = setup();
    let secret = Bytes::from_slice(&env, b"s");
    let gid = client.create_gift(&sender, &token, &300, &hash(&env, &secret), &1000);

    env.ledger().with_mut(|l| l.timestamp = 2000);
    client.refund(&gid);

    let tok = soroban_sdk::token::TokenClient::new(&env, &token);
    assert_eq!(tok.balance(&sender), 1_000);
    assert_eq!(client.get_gift(&gid).status, Status::Refunded);
}
