use soroban_sdk::{vec, Address, Env, String};
use crate::error::Error;
use crate::minting::MintClient;
use crate::payer::Payer;
use crate::store::{AssetInfo, OrderInfo, StorageKey, TransferInfo, ADMIN};

pub struct Transfer;

impl Transfer {
    /// Calls the 'transfer' function of the 'contract' with 'to' and 'amount'.
    pub fn transfer(
        env: Env,
        order: String,
        transfer: String,
        beneficiary: String,
        amount: i128,
    ) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // Verify amount is positive.
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }
        // Get order info
        let order_info: OrderInfo = env.storage().persistent()
            .get(&StorageKey::Order(order.clone())).unwrap();

        // Get info about asset generated for order
        let mut asset_info: AssetInfo = env.storage().persistent()
            .get(&StorageKey::Asset(order_info.code.clone(), order_info.issuer.clone()))
            .unwrap();

        let date = Option::from(env.ledger().timestamp());
        let b = beneficiary.clone();

        // Update information about payment operations
        if asset_info.transfers == None {
            let create_transfer = vec!(&env,
                                       TransferInfo { transfer, beneficiary, amount, date });
            asset_info.transfers = Option::from(create_transfer);
        } else {
            let mut recorded_transfers = asset_info.transfers.unwrap();
            recorded_transfers.push_back(TransferInfo { transfer, beneficiary, amount, date });
            asset_info.transfers = Option::from(recorded_transfers);
        }

        let client = MintClient::new(&env, &order_info.contract);
        // Burn asset
        let from = Payer::payer(env.clone(), asset_info.clone().payer.unwrap());
        client.clawback(&from, &amount);

        // Perform the mint.
        // Get address for payer
        let to = Payer::payer(env.clone(), b);
        client.mint(&to, &amount);
        // freeze asset
        client.set_authorized(&to, &false);

        // Store information about payment operations
        env.storage().persistent().set(&StorageKey::Asset(
            order_info.code.clone(), order_info.issuer.clone()), &asset_info);

        Ok(())
    }
    /// Calls the 'approve_transfer' function of the 'contract' to unfreeze assets.
    pub fn approve_transfer(
        env: Env,
        order: String,
        beneficiary: String
    ) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // Get order info
        let order_info: OrderInfo = env.storage().persistent()
            .get(&StorageKey::Order(order.clone())).unwrap();

        let client = MintClient::new(&env, &order_info.contract);
        // Get address for payer
        let to = Payer::payer(env.clone(), beneficiary);
        // freeze asset
        client.set_authorized(&to, &true);
        Ok(())
    }
}
