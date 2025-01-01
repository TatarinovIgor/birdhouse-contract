use crate::store::ADMIN;
use soroban_sdk::{Address, BytesN, Env, String};

pub struct UpgradeableContract;

impl UpgradeableContract {
    pub fn version_build(env: Env) -> String {
        String::from_str(&env, "goods:0.0.1")
    }

    pub fn version() -> i32 {
        1734008630
    }

    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();
        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
