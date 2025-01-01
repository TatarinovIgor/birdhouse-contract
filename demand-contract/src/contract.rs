use crate::store::StorageKey::{Media, MediaBlock};
use crate::store::{
    MediaInfo, MediaType, ADMIN, DESCRIPTION, ID, MEDIA_LIST, NAME, BUDGET, TOML,
};
use crate::upgrade::UpgradeableContract;
use soroban_sdk::{contract, contractimpl, vec, Address, BytesN, Env, String, Vec};

// GCZMWP4RXU2QQV4ASC3PF6K4URGMBES4UR5GS2QYDUACL5SDSQWTDX52
// SAOASHP7NFQ3YO5AMYC6QVI5HOAWCIFUXVEZYFETIC4KSLZSRVXXMZU7

// GDZ7PEAS7EFVVSBQEU6L2BHK6JOUWTL246I3LSMWN5JTXVK3BIBFGITB
// SDZTZFNVGEF3PPKZEBTQVRZHI3HYGBFTFYHO6CLWHJYMRBCYXMLLWXOP

// GD4UOXPQ4CSXPXFQTHGWL7PBLOG5OOJ7J3K3SDJW5NIGIHAVEIGYEYYA
// SCTB47BHRKXFKTUA3Y676OT2H6YVIMQLT3OABPNWV77GRNBKSDXWRUZT

// 0000000087a46c6919e3d503e1ba3f99225098f33869b9ac5d6532fd6b2277d6f2038cff
/// Admin
/// Client
/// Unique ID
/// Name
/// Description
/// Budget
/// Amount
/// Media
/// Toml file link

#[contract]
pub struct DemandContract;

#[contractimpl]
impl DemandContract {
    /// Constructor requires Admin address
    pub fn __constructor(
        e: Env,
        admin: Address,
        id: String,
        name: String,
        description: String,
        budget: u64,
        toml_file_link: String,
    ) {
        // Set ID for smart contract
        e.storage().persistent().set(&ID, &id);

        Self::set_admin(e.clone(), admin);
        Self::set_name(e.clone(), name);
        Self::set_description(e.clone(), description);
        Self::set_budget(e.clone(), budget);
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

    pub fn set_budget(env: Env, budget: u64) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&BUDGET, &budget);
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

    pub fn set_toml_file(env: Env, toml_file_link: String) {
        Self::authorize_admin(&env);
        env.storage().persistent().set(&TOML, &toml_file_link)
    }

    /// return version description
    pub fn version_build(env: Env) -> String {
        UpgradeableContract::version_build(env)
    }
    /// return timestemp of the build
    pub fn version() -> i32 {
        UpgradeableContract::version()
    }

    /// Upgrade smart contract
    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        UpgradeableContract::upgrade(env, new_wasm_hash)
    }
}
