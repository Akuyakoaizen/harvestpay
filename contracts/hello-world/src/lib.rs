#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Address, Symbol};

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum HarvestStatus {
    Pending,
    Confirmed,
    Paid,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct HarvestRecord {
    pub farmer: Address,
    pub weight_kg: u64,
    pub amount_usdc: i128,
    pub status: HarvestStatus,
}

const HARVEST: Symbol = symbol_short!("HARVEST");
const COOP: Symbol = symbol_short!("COOP");

#[contract]
pub struct HarvestPayContract;

#[contractimpl]
impl HarvestPayContract {
    pub fn initialize(env: Env, cooperative: Address) {
        env.storage().instance().set(&COOP, &cooperative);
    }

    pub fn submit_harvest(env: Env, farmer: Address, weight_kg: u64, amount_usdc: i128) {
        farmer.require_auth();
        let record = HarvestRecord {
            farmer: farmer.clone(),
            weight_kg,
            amount_usdc,
            status: HarvestStatus::Pending,
        };
        env.storage().instance().set(&HARVEST, &record);
    }

    pub fn confirm_and_pay(env: Env, cooperative: Address) {
        cooperative.require_auth();
        let stored_coop: Address = env.storage().instance().get(&COOP).unwrap();
        assert!(cooperative == stored_coop, "Unauthorized: not the cooperative");
        let mut record: HarvestRecord = env.storage().instance().get(&HARVEST).unwrap();
        assert!(record.status == HarvestStatus::Pending, "Already processed");
        record.status = HarvestStatus::Paid;
        env.storage().instance().set(&HARVEST, &record);
    }

    pub fn get_harvest(env: Env) -> HarvestRecord {
        env.storage().instance().get(&HARVEST).unwrap()
    }

    pub fn get_cooperative(env: Env) -> Address {
        env.storage().instance().get(&COOP).unwrap()
    }
}

mod test;