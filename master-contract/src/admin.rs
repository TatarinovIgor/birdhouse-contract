use soroban_sdk::{Address, Env};
use crate::store::ADMIN;

pub struct Admin;

impl Admin {
    /// Return the admin address.
    pub fn admin(env: Env) -> Address {
        env.storage().persistent().get(&ADMIN).unwrap()
    }

    /// Set the admin.
    pub fn set_admin(env: Env, new_admin: Address) {
        if let Some(admin) = env
            .storage()
            .persistent()
            .get::<_, Address>(&ADMIN)
        {
            admin.require_auth();
        };
        env.storage().persistent().set(&ADMIN, &new_admin);
    }
}