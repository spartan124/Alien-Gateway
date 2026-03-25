#![cfg(test)]

use crate::{Contract, ContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, BytesN, Env};

#[path = "test_registration.rs"]
mod test_registration;
#[path = "test_address_manager.rs"]
mod test_address_manager;

// ============================================
// Test Setup Helpers
// ============================================

pub(crate) fn setup_test(env: &Env) -> (ContractClient<'_>, BytesN<32>, Address) {
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(env, &contract_id);
    let commitment = BytesN::from_array(env, &[7u8; 32]);
    let wallet = Address::generate(env);

    (client, commitment, wallet)
}

pub(crate) fn setup_with_owner(env: &Env) -> (ContractClient<'_>, Address, BytesN<32>) {
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(env, &contract_id);
    let owner = Address::generate(env);
    let commitment = BytesN::from_array(env, &[42u8; 32]);

    // Initialize address manager with owner
    client.init_address_manager(&owner);

    (client, owner, commitment)
}

// ============================================
// Resolver Tests (existing)
// ============================================

#[test]
fn test_resolve_returns_none_when_no_memo() {
    let env = Env::default();
    let (client, commitment, wallet) = setup_test(&env);

    client.register_resolver(&commitment, &wallet, &None);

    let (resolved_wallet, memo) = client.resolve(&commitment);
    assert_eq!(resolved_wallet, wallet);
    assert_eq!(memo, None);
}

#[test]
fn test_set_memo_and_resolve_flow() {
    let env = Env::default();
    let (client, commitment, wallet) = setup_test(&env);

    client.register_resolver(&commitment, &wallet, &None);
    client.set_memo(&commitment, &4242u64);

    let (resolved_wallet, memo) = client.resolve(&commitment);
    assert_eq!(resolved_wallet, wallet);
    assert_eq!(memo, Some(4242u64));
}

