use crate::store::StorageKey::{Media, MediaBlock};
use crate::store::{MediaInfo, MediaType, ADMIN, AMOUNT, DESCRIPTION, ID, NAME, PRICE, TOML};
use soroban_sdk::{contract, contractimpl, Address, Env, String};

/// Admin
/// Partner
/// Unique ID
/// Name
/// Description
/// Price
/// Amount
/// Media
/// Toml file link

#[contract]
pub struct GoodsContract;

#[contractimpl]
impl GoodsContract {
    /// Constructor requires Admin address
    pub fn __constructor(
        e: Env,
        admin: Address,
        id: String,
        name: String,
        description: String,
        price: u64,
        amount: u64,
        toml_file_link: String,
    ) {
        // Set ID for smart contract
        e.storage().persistent().set(&ID, &id);

        Self::set_admin(e.clone(), admin);
        Self::set_name(e.clone(), name);
        Self::set_description(e.clone(), description);
        Self::set_price(e.clone(), price);
        Self::set_amount(e.clone(), amount);
        Self::set_toml_file(e, toml_file_link);
    }
    fn authorize_admin(env: &Env) {
        if let Some(admin) = env.storage().persistent().get::<_, Address>(&ADMIN) {
            admin.require_auth();
        }
    }
    pub fn set_admin(env: Env, admin: Address) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&ADMIN, &admin);
    }

    pub fn set_name(env: Env, name: String) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&NAME, &name);
    }

    pub fn set_description(env: Env, description: String) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&DESCRIPTION, &description);
    }

    pub fn set_price(env: Env, price: u64) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&PRICE, &price);
    }

    pub fn set_amount(env: Env, amount: u64) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&AMOUNT, &amount);
    }

    pub fn media_max_block_size() -> u64 {
        2 << 16
    }

    pub fn add_media(
        env: Env,
        media_id: String,
        media_type: MediaType,
        media: String,
        total_blocks: u64,
    ) {
        Self::authorize_admin(&env);
        let media_info = MediaInfo {
            media_id: media_id.clone(),
            media_type,
            total_blocks,
            media,
        };
        env.storage()
            .persistent()
            .set(&Media(media_id), &media_info);
    }
    pub fn upload_media_block(env: Env, media_id: String, media: String, block_number: u64) {
        Self::authorize_admin(&env);
        env.storage()
            .persistent()
            .set(&MediaBlock(media_id, block_number), &media)
    }
    fn remove_media_block(env: Env, media_id: String, block_number: u64) {
        env.storage()
            .persistent()
            .remove(&MediaBlock(media_id, block_number))
    }
    pub fn remove_media(env: Env, media_id: String) {
        Self::authorize_admin(&env);
        if env.storage().persistent().has(&Media(media_id.clone())) {
            let media_info: MediaInfo = env
                .storage()
                .persistent()
                .get(&Media(media_id.clone()))
                .unwrap();
            for block in 1..=media_info.total_blocks {
                Self::remove_media_block(env.clone(), media_id.clone(), block);
            }
        }
    }

    pub fn set_toml_file(env: Env, toml_file_link: String) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&TOML, &toml_file_link)
    }

}
