use soroban_sdk::{contractimpl, BytesN, Env, contract, Address};
use crate::store::{ADMIN};

#[contract]
pub struct UpgradeableContract;

#[contractimpl]
impl UpgradeableContract {

    pub fn version() -> u32 {
        4
    }

    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}