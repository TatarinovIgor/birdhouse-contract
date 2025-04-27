use crate::commission::Commission;
use crate::error::Error;
use crate::minting::MintClient;
use crate::payer::Payer;
use crate::store::StorageKey::{Payouts, Transfers};
use crate::store::{
    get_asset_info, get_order_info, get_pay_asset_info, get_stored_transfers, OrderInfo,
    StorageKey, TransferInfo, ADMIN,
};
use soroban_sdk::{vec, Address, Env, String, Vec};

pub struct Transfer;

impl Transfer {
    fn get_admin(env: &Env) -> Result<Address, Error> {
        Ok(env.storage().persistent().get(&ADMIN).unwrap())
    }

    fn validate_admin(env: &Env) -> Result<(), Error> {
        let admin = Self::get_admin(env)?;
        admin.require_auth();
        Ok(())
    }

    fn update_transfer_records(
        env: &Env,
        code: &String,
        issuer: &Address,
        transfer_info: TransferInfo,
    ) -> Result<(), Error> {
        let storage_key = Transfers(code.clone(), issuer.clone());
        let transfers = if env.storage().persistent().has(&storage_key) {
            let mut existing_transfers: Vec<TransferInfo> =
                env.storage().persistent().get(&storage_key).unwrap();
            existing_transfers.push_back(transfer_info);
            existing_transfers
        } else {
            vec![env, transfer_info]
        };
        env.storage().persistent().set(&storage_key, &transfers);
        Ok(())
    }

    fn find_and_remove_transfer(
        transfers: &mut Vec<TransferInfo>,
        transfer_id: &String,
    ) -> Result<TransferInfo, Error> {
        for (index, recorded_transfer) in transfers.iter().enumerate() {
            if recorded_transfer.transfer == *transfer_id {
                let transfer = recorded_transfer.clone();
                transfers.remove(index as u32);
                return Ok(transfer);
            }
        }
        Err(Error::IncorrectTransfer)
    }

    /// Calls the 'transfer' function of the 'contract' with 'to' and 'amount'.
    pub fn transfer(
        env: Env,
        order: String,
        transfer: String,
        payer: String,
        beneficiary: String,
        amount: i128,
        fee: i128,
    ) -> Result<(), Error> {
        Self::validate_admin(&env)?;

        // Verify the amount is positive after commission deduction
        if amount - fee < 0 {
            return Err(Error::NegativeAmount);
        }

        let order_info = get_order_info(&env, &order)?;
        let asset_info = get_asset_info(&env, &order_info.code, &order_info.issuer)?;

        let transfer_info = TransferInfo {
            transfer,
            payer: payer.clone(),
            beneficiary: beneficiary.clone(),
            amount,
            fee,
            date: Option::from(env.ledger().timestamp()),
        };
        Self::update_transfer_records(&env, &order_info.code, &order_info.issuer, transfer_info)?;

        let client = MintClient::new(&env, &order_info.contract);
        let from = Payer::payer(env.clone(), payer);

        client.clawback(&from, &amount);

        env.storage().persistent().set(
            &StorageKey::Asset(order_info.code, order_info.issuer),
            &asset_info,
        );

        Ok(())
    }

    fn update_payout_records(
        env: &Env,
        code: &String,
        issuer: &Address,
        payout_info: TransferInfo,
    ) -> Result<(), Error> {
        let storage_key = Payouts(code.clone(), issuer.clone());
        let payouts = if env.storage().persistent().has(&storage_key) {
            let mut existing_payouts: Vec<TransferInfo> =
                env.storage().persistent().get(&storage_key).unwrap();
            existing_payouts.push_back(payout_info);
            existing_payouts
        } else {
            vec![env, payout_info]
        };
        env.storage().persistent().set(&storage_key, &payouts);
        Ok(())
    }

    /// Calls the 'approve_transfer' function of the 'contract' to unfreeze assets.
    pub fn approve_transfer(env: Env, order: String, transfer: String) -> Result<(), Error> {
        Self::validate_admin(&env)?;
        let order_info = get_order_info(&env, &order)?;

        // Find and remove the transfer record
        let mut recorded_transfers =
            get_stored_transfers(&env, &order_info.code, &order_info.issuer)?;
        let approved_transfer = Self::find_and_remove_transfer(&mut recorded_transfers, &transfer)?;

        // Update transfers storage
        env.storage().persistent().set(
            &Transfers(order_info.code.clone(), order_info.issuer.clone()),
            &recorded_transfers,
        );

        // Create and store payout record
        let payout_info = TransferInfo {
            transfer,
            payer: approved_transfer.payer,
            beneficiary: approved_transfer.beneficiary.clone(),
            amount: approved_transfer.amount,
            fee: approved_transfer.fee,
            date: Option::from(env.ledger().timestamp()),
        };
        Self::update_payout_records(&env, &order_info.code, &order_info.issuer, payout_info)?;

        // Perform asset swap
        let beneficiary_address = Payer::payer(env.clone(), approved_transfer.beneficiary);

        let pay_asset: OrderInfo = get_pay_asset_info(&env)?;
        let client_payout = MintClient::new(&env, &pay_asset.contract);
        client_payout.mint(
            &beneficiary_address,
            &(approved_transfer.amount - approved_transfer.fee),
        );
        let _ = Commission::pay_commission(env.clone(), &approved_transfer.fee);
        Ok(())
    }

    /// Calls the 'reject_transfer' function of the 'contract' to recall assets to a payer account.
    pub fn reject_transfer(env: Env, order: String, transfer: String) -> Result<(), Error> {
        Self::validate_admin(&env)?;
        let order_info = get_order_info(&env, &order)?;

        // Find and remove the transfer record
        let mut recorded_transfers =
            get_stored_transfers(&env, &order_info.code, &order_info.issuer)?;
        let rejected_transfer = Self::find_and_remove_transfer(&mut recorded_transfers, &transfer)?;

        // Update transfers storage
        env.storage().persistent().set(
            &Transfers(order_info.code.clone(), order_info.issuer.clone()),
            &recorded_transfers,
        );

        // Perform asset reallocation
        let client = MintClient::new(&env, &order_info.contract);
        let payer = Payer::payer(env.clone(), rejected_transfer.payer);

        client.mint(&payer, &rejected_transfer.amount);

        Ok(())
    }
}
