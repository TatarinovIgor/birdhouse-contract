use soroban_sdk::{contracttype, symbol_short, Address, Env, String, Symbol, Vec};
use crate::error::Error;
use crate::store::StorageKey::Transfers;

/// Admin is an address that authorized to sign the contract. Value is an Address
pub(crate) const ADMIN: Symbol = symbol_short!("Admin");

/// FeeAcc is an address that collects fee from smart contract operations. Value is an Address
pub(crate) const FEE_ACCOUNT: Symbol = symbol_short!("FeeAcc");

/// LastAsset is a name of the last-used asset for smart contract generation. Value is a Symbol
pub(crate) const LAST_ASSET: Symbol = symbol_short!("LastAsset");

/// PayAsset is a name of the last used asset for smart contract generation. Value is an OrderInfo
pub(crate) const PAY_ASSET: Symbol = symbol_short!("PayAsset");

/// LastBurn is a counter of burns calls. Value is an u64
pub(crate) const LAST_BURN: Symbol = symbol_short!("LastBurn");

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrderInfo {
    pub(crate) contract: Address,
    pub(crate) code: String,
    pub(crate) issuer: Address,
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaymentInfo {
    pub(crate) payment: String,
    pub(crate) payer: String,
    pub(crate) amount: i128,
    pub(crate) fee: i128,
    pub(crate) date: Option<u64>,
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TransferInfo {
    pub(crate) transfer: String,
    pub(crate) payer: String,
    pub(crate) beneficiary: String,
    pub(crate) amount: i128,
    pub(crate) fee: i128,
    pub(crate) date: Option<u64>,
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AssetInfo {
    pub(crate) order: String,
}

#[contracttype]
pub enum StorageKey {
    /// Order is an order id that was used to issue an asset. Value is OrderInfo.
    Order(String),
    /// Asset is an asset issued by this smart contract. Value is AssetInfo
    Asset(String, Address),
    /// Payment is a list of payment that was made by this smart contract for the asset.
    /// Value is PaymentInfo
    Payments(String, Address),
    /// Transfer is a list of transfer that was made by this smart contract for the asset.
    ///  Value is TransferInfo
    Transfers(String, Address),
    /// Payouts is a list of payout that was made by this smart contract for the asset.
    /// Value is TransferInfo
    Payouts(String, Address),
    /// Withdraw is a withdrawal payout request made by this smart contract for the pay asset.
    /// Value is TransferInfo
    Withdraw(String),
    /// Burn is the executed burn payout approved by this smart contract for the pay asset.
    /// Value is TransferInfo
    Burn(u64),
    /// Payer is an id of user that do payment and receive confirmation as issued assets.
    /// Value is Address
    Payer(String),
}


pub fn get_order_info(env: &Env, order: &String) -> Result<OrderInfo, Error> {
    Ok(env
        .storage()
        .persistent()
        .get(&StorageKey::Order(order.clone()))
        .unwrap())
}

pub fn get_asset_info(env: &Env, code: &String, issuer: &Address) -> Result<AssetInfo, Error> {
    Ok(env
        .storage()
        .persistent()
        .get(&StorageKey::Asset(code.clone(), issuer.clone()))
        .unwrap())
}


pub fn get_pay_asset_info(env: &Env) -> Result<OrderInfo, Error> {
    Ok(env
        .storage()
        .persistent()
        .get(&PAY_ASSET)
        .unwrap())
}

pub fn get_stored_transfers(
    env: &Env,
    code: &String,
    issuer: &Address,
) -> Result<Vec<TransferInfo>, Error> {
    let storage_key = Transfers(code.clone(), issuer.clone());
    if !env.storage().persistent().has(&storage_key) {
        return Err(Error::IncorrectTransfer);
    }
    Ok(env.storage().persistent().get(&storage_key).unwrap())
}