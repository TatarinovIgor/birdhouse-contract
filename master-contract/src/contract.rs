use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, EnvBase, String, Val};
use crate::admin::Admin;
use crate::burn::Burn;
use crate::deployer::Deployer;
use crate::error::{Error};
use crate::minting::Minter;
use crate::payer::Payer;
use crate::serialize_xdr::{CPAsset, CPWriteXdr};
use crate::upgrade::UpgradeableContract;
use crate::store::{OrderInfo, ADMIN, PAY_ASSET};
use crate::transfer::Transfer;

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    pub fn __constructor(e: Env, admin: Address, pay_asset: String) {
        let _ = Self::init(e, admin, pay_asset).expect("can't initialize smart contract");
    }
    pub fn init(e: Env, admin: Address, pay_asset: String) -> Result<(), Error> {
        if e.storage().persistent().has(&ADMIN) {
            return Err(Error::AlreadyInitialized);
        }
        e.storage().persistent().set(&ADMIN, &admin);
        let asset_code: &mut [u8; 12] = &mut [0u8; 12];
        e.string_copy_to_slice(pay_asset.to_object(), Val::U32_ZERO,
                               asset_code[..(pay_asset.len() as usize)].as_mut()).unwrap();
        // Convert Symbol to String using the function
        let asset = CPAsset { code: *asset_code, issuer: admin.to_string() };
        let asset_serialized = asset.to_xdr(&e).unwrap();

        // Deploy the contract using the uploaded Wasm with given hash.
        let deployed_address = e
            .deployer()
            .with_stellar_asset(asset_serialized.clone())
            .deploy();
        // store cash out information
        let order_key = &OrderInfo {
            contract: deployed_address.clone(),
            code: pay_asset.clone(),
            issuer: admin,
        };
        e.storage().persistent().set(&PAY_ASSET, order_key);

        Ok(())
    }
    pub fn admin(env: Env) -> Address {
        Admin::admin(env)
    }
    pub fn set_admin(env: Env, new_admin: Address) {
        Admin::set_admin(env, new_admin)
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
    pub fn approve_transfer(
        env: Env,
        order: String,
        beneficiary: String,
    ) -> Result<(), Error> {
        Transfer::approve_transfer(env, order, beneficiary)
    }
    pub fn reject_transfer(
        env: Env,
        order: String,
        beneficiary: String,
    ) -> Result<(), Error> {
        Transfer::reject_transfer(env, order, beneficiary)
    }
    pub fn burn(
        env: Env,
        code: String,
        issuer: Address,
        payout: String,
        amount: i128,
    ) -> Result<(), Error> {
        Burn::burn(env, code, issuer, payout, amount)
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