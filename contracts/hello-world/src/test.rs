#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(HarvestPayContract, ());
    let client = HarvestPayContractClient::new(&env, &contract_id);
    let cooperative = Address::generate(&env);
    client.initialize(&cooperative);
    assert_eq!(client.get_cooperative(), cooperative);
}

#[test]
fn test_submit_harvest() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(HarvestPayContract, ());
    let client = HarvestPayContractClient::new(&env, &contract_id);
    let cooperative = Address::generate(&env);
    let farmer = Address::generate(&env);
    client.initialize(&cooperative);
    client.submit_harvest(&farmer, &500, &1000);
    let record = client.get_harvest();
    assert_eq!(record.farmer, farmer);
    assert_eq!(record.weight_kg, 500);
    assert_eq!(record.amount_usdc, 1000);
    assert_eq!(record.status, HarvestStatus::Pending);
}

#[test]
fn test_confirm_and_pay() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(HarvestPayContract, ());
    let client = HarvestPayContractClient::new(&env, &contract_id);
    let cooperative = Address::generate(&env);
    let farmer = Address::generate(&env);
    client.initialize(&cooperative);
    client.submit_harvest(&farmer, &300, &600);
    client.confirm_and_pay(&cooperative);
    let record = client.get_harvest();
    assert_eq!(record.status, HarvestStatus::Paid);
}