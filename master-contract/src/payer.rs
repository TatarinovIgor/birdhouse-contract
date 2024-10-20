use soroban_sdk::{Address, Env, String};
use crate::store::{StorageKey, ADMIN};

pub struct Payer;

impl Payer {
    /// Return the payer address.
    pub fn payer(env: Env, id: String) -> Address {
        env.storage().persistent().get(&StorageKey::Payer(id.clone())).unwrap()
    }

    /// Add payer.
    pub fn add_payer(env: Env, id: String, address: Address) {
        if let Some(admin) = env
            .storage()
            .persistent()
            .get::<_, Address>(&ADMIN)
        {
            admin.require_auth();
        };
        env.storage().persistent().set(&StorageKey::Payer(id.clone()), &address);
    }

    /// Remove payer.
    pub fn remove_payer(env: Env, id: String) {
        if let Some(admin) = env
            .storage()
            .persistent()
            .get::<_, Address>(&ADMIN)
        {
            admin.require_auth();
        };
        let is_exist = env.storage()
            .persistent()
            .has(&StorageKey::Payer(id.clone()));

        if is_exist {
            env.storage()
                .persistent()
                .remove(&StorageKey::Payer(id.clone()));
        }
    }
}