use soroban_sdk::{vec, Address, Env, String, Vec};
use crate::error::Error;
use crate::minting::MintClient;
use crate::payer::Payer;
use crate::store::{AssetInfo, OrderInfo, StorageKey, TransferInfo, ADMIN};
use crate::store::StorageKey::Transfers;

pub struct Burn;

impl Burn {
    /// Calls the 'burn' function of the 'contract' with 'amount' to burn payer assets.
    pub fn burn(
        env: Env,
        code: String,
        issuer: Address,
        payout: String,
        amount: i128,
    ) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // Verify amount is positive.
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }

        // Get info about asset generated for order
        let asset_info: AssetInfo = env.storage().persistent()
            .get(&StorageKey::Asset(code, issuer))
            .unwrap();

        // Get order info
        let order_info: OrderInfo = env.storage().persistent()
            .get(&StorageKey::Order(asset_info.clone().order)).unwrap();

        let date = Option::from(env.ledger().timestamp());
        let payer =  asset_info.clone().payer.unwrap();
        // Update information about payout operations
        if env.storage().persistent()
            .has(&Transfers(order_info.code.clone(), order_info.issuer.clone())) {
            let mut recorded_pay_out : Vec<TransferInfo> = env.storage().persistent()
                .get(&Transfers(order_info.code.clone(), order_info.issuer.clone())).unwrap();
            recorded_pay_out.push_back(TransferInfo { transfer: payout,
                beneficiary: payer.clone(), amount, date });
            env.storage().persistent()
                .set(&Transfers(order_info.code.clone(), order_info.issuer.clone()),
                     &recorded_pay_out);
        } else {
            let create_pay_out = vec!(&env,
                                       TransferInfo { transfer: payout,
                                           beneficiary: payer.clone(), amount, date });
            env.storage().persistent()
                .set(&Transfers(order_info.code.clone(), order_info.issuer.clone()),
                     &create_pay_out);
        }

        let client = MintClient::new(&env, &order_info.contract);
        // Burn asset

        let from_address = Payer::payer(env.clone(), payer);
        client.clawback(&from_address, &amount);


        // Store information about payment operations
        env.storage().persistent().set(&StorageKey::Asset(
            order_info.code.clone(), order_info.issuer.clone()), &asset_info);

        Ok(())
    }
}
