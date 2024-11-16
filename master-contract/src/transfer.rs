use soroban_sdk::{vec, Address, Env, String, Vec};
use crate::error::Error;
use crate::minting::MintClient;
use crate::payer::Payer;
use crate::store::{AssetInfo, OrderInfo, StorageKey, TransferInfo, ADMIN, PAY_ASSET};
use crate::store::StorageKey::{Payouts, Transfers};

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
        let asset_info: AssetInfo = env.storage().persistent()
            .get(&StorageKey::Asset(order_info.code.clone(), order_info.issuer.clone()))
            .unwrap();

        let date = Option::from(env.ledger().timestamp());
        let b = beneficiary.clone();

        // Update information about payment operations
        if env.storage().persistent()
            .has(&Transfers(order_info.code.clone(), order_info.issuer.clone())) {
            let mut recorded_transfers: Vec<TransferInfo> = env.storage().persistent()
                .get(&Transfers(order_info.code.clone(), order_info.issuer.clone())).unwrap();
            recorded_transfers.push_back(TransferInfo { transfer, beneficiary, amount, date });
            env.storage()
                .persistent()
                .set(&Transfers(order_info.code.clone(), order_info.issuer.clone()),
                     &recorded_transfers);
        } else {
            let create_transfer = vec!(&env,
                                       TransferInfo { transfer, beneficiary, amount, date });
            env.storage()
                .persistent()
                .set(&Transfers(order_info.code.clone(), order_info.issuer.clone()),
                     &create_transfer);
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
        transfer: String,
    ) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // Get order info
        let order_info: OrderInfo = env.storage().persistent()
            .get(&StorageKey::Order(order.clone())).unwrap();
        if !env.storage().persistent()
            .has(&Transfers(order_info.code.clone(), order_info.issuer.clone())) {}
        let mut approved_transfer: TransferInfo = TransferInfo {
            transfer: String::from_str(&env, ""),
            beneficiary: String::from_str(&env, ""),
            amount: 0,
            date: None,
        };
        let mut recorded_transfers: Vec<TransferInfo> = env.storage().persistent()
            .get(&Transfers(order_info.code.clone(), order_info.issuer.clone())).unwrap();
        for (index, recorded_transfer) in recorded_transfers.iter().enumerate() {
            if recorded_transfer.transfer == transfer.clone() {
                approved_transfer = recorded_transfer.clone();
                recorded_transfers.remove(index as u32);
                break;
            }
        }
        if approved_transfer.transfer != transfer.clone() {
            return Err(Error::IncorrectTransfer);
        }
        // updated transfer in storage
        env.storage().persistent()
            .set(&Transfers(order_info.code.clone(), order_info.issuer.clone()),
                 &recorded_transfers);

        // update payout in storage
        let date = Option::from(env.ledger().timestamp());
        let beneficiary = approved_transfer.beneficiary;
        let amount = approved_transfer.amount;
        if env.storage().persistent()
            .has(&Payouts(order_info.code.clone(), order_info.issuer.clone())) {
            let mut recorded_payouts: Vec<TransferInfo> = env.storage().persistent()
                .get(&Payouts(order_info.code.clone(), order_info.issuer.clone())).unwrap();
            recorded_payouts.push_back(TransferInfo { transfer, beneficiary: beneficiary.clone(),
                amount, date });
            env.storage()
                .persistent()
                .set(&Payouts(order_info.code.clone(), order_info.issuer.clone()),
                     &recorded_payouts);
        } else {
            let create_payout = vec!(&env,
                                       TransferInfo { transfer, beneficiary: beneficiary.clone(),
                                           amount, date });
            env.storage()
                .persistent()
                .set(&Payouts(order_info.code.clone(), order_info.issuer.clone()),
                     &create_payout);
        }

        let client = MintClient::new(&env, &order_info.contract);
        // Get address for payer
        let beneficiary = Payer::payer(env.clone(), beneficiary);
        // swap assets
        client.clawback(&beneficiary, &amount);
        let pay_asset: OrderInfo = env.storage().persistent().get(&PAY_ASSET).unwrap();
        let client_payout = MintClient::new(&env, &pay_asset.contract);
        client_payout.mint(&beneficiary, &amount);


        Ok(())
    }

    /// Calls the 'reject_transfer' function of the 'contract' to recall assets to payer account.
    pub fn reject_transfer(
        env: Env,
        order: String,
        transfer: String,
    ) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // Get order info
        let order_info: OrderInfo = env.storage().persistent()
            .get(&StorageKey::Order(order.clone())).unwrap();
        if !env.storage().persistent()
            .has(&Transfers(order_info.code.clone(), order_info.issuer.clone())) {
            return Err(Error::IncorrectTransfer);
        }

        // Get info about asset generated for order
        let asset_info: AssetInfo = env.storage().persistent()
            .get(&StorageKey::Asset(order_info.code.clone(), order_info.issuer.clone()))
            .unwrap();

        let mut rejected_transfer: TransferInfo = TransferInfo {
            transfer: String::from_str(&env, ""),
            beneficiary: String::from_str(&env, ""),
            amount: 0,
            date: None,
        };
        let mut recorded_transfers: Vec<TransferInfo> = env.storage().persistent()
            .get(&Transfers(order_info.code.clone(), order_info.issuer.clone())).unwrap();
        for (index, recorded_transfer) in recorded_transfers.iter().enumerate() {
            if recorded_transfer.transfer == transfer.clone() {
                rejected_transfer = recorded_transfer.clone();
                recorded_transfers.remove(index as u32);
                break;
            }
        }
        if rejected_transfer.transfer != transfer.clone() {
            return Err(Error::IncorrectTransfer);
        }
        env.storage().persistent()
            .set(&Transfers(order_info.code.clone(), order_info.issuer.clone()),
                 &recorded_transfers);

        let client = MintClient::new(&env, &order_info.contract);

        // remove asset from beneficiary
        let beneficiary = Payer::payer(env.clone(), rejected_transfer.beneficiary);
        client.clawback(&beneficiary, &rejected_transfer.amount);

        // back asset to payer
        let payer = Payer::payer(env.clone(), asset_info.payer.unwrap());
        client.mint(&payer, &rejected_transfer.amount);

        Ok(())
    }
}
