#![cfg(test)]
use super::*;
use soroban_sdk::testutils::BytesN as _;
use soroban_sdk::{vec, Env};

#[test]
fn init_and_signer_registry() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Contract, ());
    let client = ContractClient::new(&env, &id);

    let k1 = BytesN::<32>::random(&env);
    client.init(&vec![&env, k1.clone()]);

    assert_eq!(client.get_signers().len(), 1);
    assert_eq!(client.nonce(), 0);

    let k2 = BytesN::<32>::random(&env);
    client.add_signer(&k2);
    assert_eq!(client.get_signers().len(), 2);

    client.remove_signer(&k1);
    assert_eq!(client.get_signers().len(), 1);
}

#[test]
#[should_panic]
fn cannot_init_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Contract, ());
    let client = ContractClient::new(&env, &id);
    let k1 = BytesN::<32>::random(&env);
    client.init(&vec![&env, k1.clone()]);
    client.init(&vec![&env, k1]);
}

#[test]
#[should_panic]
fn cannot_remove_last_signer() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Contract, ());
    let client = ContractClient::new(&env, &id);
    let k1 = BytesN::<32>::random(&env);
    client.init(&vec![&env, k1.clone()]);
    client.remove_signer(&k1);
}
