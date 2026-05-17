#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{vec, Env};

fn setup() -> (Env, ContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let owner = Address::generate(&env);
    let grantee = Address::generate(&env);
    let asset = Address::generate(&env);
    let id = env.register(Contract, ());
    (env.clone(), ContractClient::new(&env, &id), owner, grantee, asset)
}

#[test]
fn spend_within_cap_then_cap_exceeded() {
    let (env, client, owner, grantee, asset) = setup();
    let sid = client.create_session(&owner, &grantee, &vec![&env, asset.clone()], &100, &1000);

    client.record_spend(&sid, &asset, &60);
    assert_eq!(client.get_session(&sid).spent, 60);

    client.record_spend(&sid, &asset, &40);
    assert_eq!(client.get_session(&sid).spent, 100);
}

#[test]
#[should_panic]
fn over_cap_rejected() {
    let (env, client, owner, grantee, asset) = setup();
    let sid = client.create_session(&owner, &grantee, &vec![&env, asset.clone()], &100, &1000);
    client.record_spend(&sid, &asset, &101);
}

#[test]
#[should_panic]
fn disallowed_asset_rejected() {
    let (env, client, owner, grantee, asset) = setup();
    let sid = client.create_session(&owner, &grantee, &vec![&env, asset], &100, &1000);
    client.record_spend(&sid, &Address::generate(&env), &10);
}

#[test]
#[should_panic]
fn revoked_session_rejected() {
    let (env, client, owner, grantee, asset) = setup();
    let sid = client.create_session(&owner, &grantee, &vec![&env, asset.clone()], &100, &1000);
    client.revoke(&sid);
    client.record_spend(&sid, &asset, &10);
}

#[test]
#[should_panic]
fn expired_session_rejected() {
    let (env, client, owner, grantee, asset) = setup();
    let sid = client.create_session(&owner, &grantee, &vec![&env, asset.clone()], &100, &1000);
    env.ledger().with_mut(|l| l.timestamp = 2000);
    client.record_spend(&sid, &asset, &10);
}
