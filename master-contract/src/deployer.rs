use soroban_sdk::{contract, contractimpl, Env, String, Address, contracttype, Val, EnvBase, Bytes};
use soroban_sdk::unwrap::UnwrapOptimized;
use crate::serialize_xdr::{CPAsset, CPWriteXdr};
use crate::store::{StorageKey, ADMIN, LAST_ASSET};

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OrderInfo {
    contract: Address,
    code: String,
    issuer: Address,
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaymentInfo {
    payment: String,
    amount: i128,
}

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AssetInfo {
    order: String,
    payments: Option<PaymentInfo>,
}

#[contract]
pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(
        env: Env,
        order: String,
        issuer: String,
        prefix: String,
    ) -> (Address, String, Address) {

        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // check is order already exists
        let is_order_exists = env.storage().persistent()
            .has::<StorageKey>(&StorageKey::Order(order.clone()));
        if is_order_exists {
            let order_from_store = env.storage().persistent()
                .get::<_, OrderInfo>(&StorageKey::Order(order.clone())).unwrap();
            return (order_from_store.contract, order_from_store.code, order_from_store.issuer);
        }

        let symbols = Bytes::from_slice(
            &env, "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes());

        let last_code: &mut [u8; 12] = &mut [0u8; 12];
        let prefix_len = prefix.len() as usize;
        env.string_copy_to_slice(prefix.to_object(), Val::U32_ZERO,
                                 last_code[..prefix_len].as_mut()).unwrap_optimized();
        let len: usize;
        let last_asset: String = env.storage().persistent()
            .get(&LAST_ASSET)
            .unwrap_or(String::from_str(&env, "A"));

        len = last_asset.len() as usize;
        env.string_copy_to_slice(
            last_asset.to_object(), Val::U32_ZERO,
            last_code[prefix_len..len + prefix_len].as_mut())
            .unwrap_optimized();
        let mut current_string = <Bytes>::from_slice(&env,
                                                     &last_code[..len + prefix_len]);
        increment_string(&mut current_string, &symbols);

        env.bytes_copy_to_slice(
            current_string.to_object(), Val::U32_ZERO,
            last_code[..len + prefix_len].as_mut())
            .unwrap_optimized();

        // Convert Symbol to String using the function
        let asset = CPAsset { code: *last_code, issuer: issuer.clone() };
        let asset_serialized = asset.to_xdr(&env).unwrap();

        // Deploy the contract using the uploaded Wasm with given hash.
        let deployed_address = env
            .deployer()
            .with_stellar_asset(asset_serialized.clone())
            .deploy();

        let code_s = core::str::from_utf8(
            &last_code[..len + prefix_len])
            .map_err(|_| "Failed to convert &[u8] to &str")
            .unwrap();

        let code_symbol = String::from_str(&env, code_s);
        // store order information
        let order_key = &OrderInfo {
            contract: deployed_address.clone(),
            code: code_symbol.clone(),
            issuer: Address::from_string(&issuer),
        };
        env.storage().persistent().set(&StorageKey::Order(order.clone()), order_key);
        // store asset information
        let asset_key = &AssetInfo {
            order: order.clone(),
            payments: None,
        };


        // store last asset used
        let code_last = core::str::from_utf8(
            &last_code[prefix_len..prefix_len + len])
            .map_err(|_| "Failed to convert &[u8] to &str")
            .unwrap();
        let code_symbol = String::from_str(&env, code_last);
        env.storage().persistent().set(&LAST_ASSET, &code_symbol);

        env.storage().persistent().set(&StorageKey::Asset(
            order_key.code.clone(), order_key.issuer.clone()), asset_key);

        (deployed_address, order_key.code.clone(), order_key.issuer.clone())
    }
}

fn increment_string(s: &mut Bytes, symbols: &Bytes) {
    let max_index = symbols.len() - 1;
    let mut increment_needed = true;
    let mut idx = s.len();

    while increment_needed && idx > 0 {
        idx -= 1;
        if let Some(current_char) = symbols.iter()
            .position(|c| c == s.get_unchecked(idx)) {
            if (current_char as u32) == max_index {
                s.set(idx, symbols.get_unchecked(0));
            } else {
                s.set(idx, symbols.get_unchecked((current_char + 1) as u32));
                increment_needed = false;
            }
        }
    }

    if increment_needed {
        s.push_back(symbols.get_unchecked(0));
    }
}
