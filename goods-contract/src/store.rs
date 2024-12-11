use soroban_sdk::{contracttype, symbol_short, String, Symbol};

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MediaType {
    ImageBase64 = 1,
    ImageLink = 2,
    VideoLink = 4,
}

/// ID is a unique id of the goods. Value is a String
pub(crate) const ID: Symbol = symbol_short!("id");

/// Admin is an admin address to manage the smart contract of the goods. Value is an Address
pub(crate) const ADMIN: Symbol = symbol_short!("Admin");

/// Name is a name of a good. Value is a String
pub(crate) const NAME: Symbol = symbol_short!("Name");

/// Description is a description of a good. Value is a String
pub(crate) const DESCRIPTION: Symbol = symbol_short!("Desc");

/// Price is a current price of a good. Value is a f64
pub(crate) const PRICE: Symbol = symbol_short!("Price");

/// Amount is a current available amount of goods. Value is an i64
pub(crate) const AMOUNT: Symbol = symbol_short!("Amount");

/// Toml is representation of toml file link. Value is String.
pub(crate) const TOML: Symbol = symbol_short!("toml");

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MediaInfo {
    pub(crate) media_id: String,
    pub(crate) media_type: MediaType,
    pub(crate) total_blocks: u64,
    pub(crate) media: String,
}

#[contracttype]
pub enum StorageKey {
    /// Media is representation of a media data. Value is MediaInfo.
    Media(String),
    /// MediaBlock is representation of a media data block as a base64 string. Value is String.
    MediaBlock(String, u64),
}
