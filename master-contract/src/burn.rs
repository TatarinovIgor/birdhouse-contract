use soroban_sdk::{vec, Address, Env, String};
use crate::error::Error;
use crate::minting::MintClient;
use crate::payer::Payer;
use crate::store::{AssetInfo, OrderInfo, StorageKey, TransferInfo, ADMIN};

pub struct Burn;

impl Burn {
    /// Calls the 'burn' function of the 'contract' with 'from' and 'amount'.
    pub fn burn(
        env: Env,
        code: String,
        issuer: Address,
        payout: String,
        from: String,
        amount: i128,
    ) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // Verify amount is positive.
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }

        // Get info about asset generated for order
        let mut asset_info: AssetInfo = env.storage().persistent()
            .get(&StorageKey::Asset(code, issuer))
            .unwrap();

        // Get order info
        let order_info: OrderInfo = env.storage().persistent()
            .get(&StorageKey::Order(asset_info.clone().order)).unwrap();

        let date = Option::from(env.ledger().timestamp());

        // Update information about payment operations
        if asset_info.cash_out == None {
            let create_cash_out = vec!(&env,
                                       TransferInfo { transfer: payout,
                                           beneficiary: from.clone(), amount, date });
            asset_info.cash_out = Option::from(create_cash_out);
        } else {
            let mut recorded_create_cash_out = asset_info.transfers.unwrap();
            recorded_create_cash_out.push_back(TransferInfo { transfer: payout,
                beneficiary: from.clone(), amount, date });
            asset_info.transfers = Option::from(recorded_create_cash_out);
        }

        let client = MintClient::new(&env, &order_info.contract);
        // Burn asset
        let from_address = Payer::payer(env.clone(), from);
        client.clawback(&from_address, &amount);


        // Store information about payment operations
        env.storage().persistent().set(&StorageKey::Asset(
            order_info.code.clone(), order_info.issuer.clone()), &asset_info);

        Ok(())
    }
}
