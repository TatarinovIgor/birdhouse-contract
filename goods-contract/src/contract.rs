use crate::store::StorageKey::{Media, MediaBlock};
use crate::store::{
    MediaInfo, MediaType, ADMIN, AMOUNT, DESCRIPTION, ID, MEDIA_LIST, NAME, PRICE, TOML,
};
use crate::upgrade::UpgradeableContract;
use soroban_sdk::{contract, contractimpl, vec, Address, BytesN, Env, String, Vec};
use soroban_sdk::token::StellarAssetInterface;

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
trait GoodsInterface : StellarAssetInterface {
    fn __constructor(
        e: Env,
        admin: Address,
        id: String,
        name: String,
        description: String,
        price: u64,
        amount: u64,
        toml_file_link: String,
    );
    fn set_name(env: Env, name: String);
    fn set_description(env: Env, description: String);
    fn set_price(env: Env, price: u64);
    fn set_amount(env: Env, amount: u64);
    fn media_max_block_size() -> u64;
    fn add_media(
        env: Env,
        media_id: String,
        media_type: MediaType,
        media: String,
        total_blocks: u64,
    );
    fn upload_media_block(env: Env, media_id: String, media: String, block_number: u64);
    fn remove_media_block(env: Env, media_id: String, block_number: u64);
    fn remove_media(env: Env, media_id: String);
    fn set_toml_file(env: Env, toml_file_link: String);
    #[doc=" return version description"]

    fn version_build(env: Env) -> String;
    #[doc=" return timestemp of the build"]

    fn version() -> i32;
    #[doc=" Upgrade smart contract"]

    fn upgrade(env: Env, new_wasm_hash: BytesN<32> );
}

#[contractimpl]
impl StellarAssetInterface for GoodsContract {
    fn set_admin(env: Env, new_admin: Address) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&ADMIN, &new_admin);
    }

    fn admin(env: Env) -> Address {
        todo!()
    }

    fn set_authorized(env: Env, id: Address, authorize: bool) {
        todo!()
    }

    fn authorized(env: Env, id: Address) -> bool {
        todo!()
    }

    fn mint(env: Env, to: Address, amount: i128) {
        todo!()
    }

    fn clawback(env: Env, from: Address, amount: i128) {
        todo!()
    }
}

#[contractimpl]
impl GoodsContract {
    fn authorize_admin(env: &Env) {
        if let Some(admin) = env.storage().persistent().get::<_, Address>(&ADMIN) {
            admin.require_auth();
        }
    }
}

#[contractimpl]
impl GoodsInterface for GoodsContract {
    /// Constructor requires Admin address
    fn __constructor(
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

        e.storage().persistent().set(&ADMIN, &admin);
        e.storage().persistent().set(&NAME, &name);
        e.storage().persistent().set(&DESCRIPTION, &description);
        e.storage().persistent().set(&PRICE, &price);
        e.storage().persistent().set(&PRICE, &price);
        e.storage().persistent().set(&TOML, &toml_file_link)
    }

    fn set_name(env: Env, name: String) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&NAME, &name);
    }

    fn set_description(env: Env, description: String) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&DESCRIPTION, &description);
    }

    fn set_price(env: Env, price: u64) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&PRICE, &price);
    }

    fn set_amount(env: Env, amount: u64) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&AMOUNT, &amount);
    }

    fn media_max_block_size() -> u64 {
        2 << 16
    }

    fn add_media(
        env: Env,
        media_id: String,
        media_type: MediaType,
        media: String,
        total_blocks: u64,
    ) {
        Self::authorize_admin(&env);
        if env.storage().persistent().has(&MEDIA_LIST) {
            let mut media_list: Vec<String> = env.storage().persistent().get(&MEDIA_LIST).unwrap();
            media_list.push_back(media_id.clone());
            env.storage().persistent().set(&MEDIA_LIST, &media_list);
        } else {
            let media_list: Vec<String> = vec![&env, media_id.clone()];
            env.storage().persistent().set(&MEDIA_LIST, &media_list);
        }
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
    fn upload_media_block(env: Env, media_id: String, media: String, block_number: u64) {
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
    fn remove_media(env: Env, media_id: String) {
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
            if env.storage().persistent().has(&MEDIA_LIST) {
                let mut media_list: Vec<String> =
                    env.storage().persistent().get(&MEDIA_LIST).unwrap();
                let index = media_list.first_index_of(&media_id);
                if !index.clone().is_none() {
                    media_list.remove(index.unwrap());
                }
                env.storage().persistent().set(&MEDIA_LIST, &media_list);
            }
            env.storage().persistent().remove(&Media(media_id.clone()));
        }
    }

    fn set_toml_file(env: Env, toml_file_link: String) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&TOML, &toml_file_link)
    }

    /// return version description
    fn version_build(env: Env) -> String {
        UpgradeableContract::version_build(env)
    }
    /// return timestemp of the build
    fn version() -> i32 {
        UpgradeableContract::version()
    }

    /// Upgrade smart contract
    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        UpgradeableContract::upgrade(env, new_wasm_hash)
    }
}
