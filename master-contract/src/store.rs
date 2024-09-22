use soroban_sdk::{contracterror, contracttype, symbol_short, Address, String, Symbol, Vec};

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    DailyLimitInsufficient = 2,
    NegativeAmount = 3,
}

/// Admin is an address that authorized to sign contract. Value is an Address
pub(crate)const ADMIN: Symbol = symbol_short!("Admin");

/// LastAsset is a name of last used asset for smart contract generation. Value is a Symbol
pub(crate) const LAST_ASSET: Symbol = symbol_short!("LastAsset");


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
    pub(crate) amount: i128,
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AssetInfo {
    pub(crate) order: String,
    pub(crate) payments: Option<Vec<PaymentInfo>>,
}

#[contracttype]
pub enum StorageKey {
    /// Order is an order id that was used to issue an asset. Value is OrderInfo.
    Order(String),
    /// Asset is an asset that was issued by this smart contract. Value is AssetInfo
    Asset(String, Address),
}
