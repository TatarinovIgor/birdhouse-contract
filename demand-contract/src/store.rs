use soroban_sdk::{contracttype, symbol_short, String, Symbol};

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MediaType {
    ImageBase64 = 1,
    ImageLink = 2,
    VideoLink = 4,
}

/// ID is a unique id of the demand. Value is a String
pub(crate) const ID: Symbol = symbol_short!("id");

/// Admin is an admin address to manage the smart contract of the demand. Value is an Address
pub(crate) const ADMIN: Symbol = symbol_short!("Admin");

/// Name is a name of a demand. Value is a String
pub(crate) const NAME: Symbol = symbol_short!("Name");

/// Description is a description of a demand. Value is a String
pub(crate) const DESCRIPTION: Symbol = symbol_short!("Desc");

/// Budget is a current budget of a demand. Value is a f64
pub(crate) const BUDGET: Symbol = symbol_short!("Budget");

/// MediaList is a list of media_id added to the smart contract. Value is an array of string
pub(crate) const MEDIA_LIST: Symbol = symbol_short!("MediaList");

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
