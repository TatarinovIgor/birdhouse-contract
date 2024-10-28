use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};
use crate::admin::Admin;
use crate::deployer::Deployer;
use crate::error::{Error};
use crate::minting::Minter;
use crate::payer::Payer;
use crate::upgrade::UpgradeableContract;
use crate::store::ADMIN;
use crate::transfer::Transfer;

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    pub fn init(e: Env, admin: Address) -> Result<(), crate::store::Error> {
        if e.storage().persistent().has(&ADMIN) {
            return Err(crate::store::Error::AlreadyInitialized);
        }
        e.storage().persistent().set(&ADMIN, &admin);
        Ok(())
    }
    pub fn admin(env: Env) -> Address {
        Admin::admin(env)
    }
    pub fn set_admin(env: Env, new_admin: Address) {
        Admin::set_admin(env, new_admin)
    }
    pub fn mint(
        env: Env,
        order: String,
        payment: String,
        payer: String,
        amount: i128,
    ) -> Result<(), Error> {
        Minter::mint(env, order, payment, payer, amount)
    }
    pub fn transfer(
        env: Env,
        order: String,
        transfer: String,
        beneficiary: String,
        amount: i128,
    ) -> Result<(), Error> {
        Transfer::transfer(env, order, transfer, beneficiary, amount)
    }
    pub fn payer(env: Env, id: String) -> Address {
        Payer::payer(env, id)
    }
    pub fn add_payer(env: Env, id: String, address: Address) {
        Payer::add_payer(env, id, address)
    }
    pub fn remove_payer(env: Env, id: String) {
        Payer::remove_payer(env, id)
    }
    pub fn deploy(
        env: Env,
        order: String,
        payer: String,
        issuer: String,
        prefix: String,
    ) -> (Address, String, Address) {
        Deployer::deploy(env, order, payer, issuer, prefix)
    }
    pub fn version_build(env: Env) -> String {
        UpgradeableContract::version_build(env)
    }
    pub fn version() -> i32 {
        UpgradeableContract::version()
    }
    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        UpgradeableContract::upgrade(env, new_wasm_hash)
    }

}