use crate::error::Error;
use crate::minting::MintClient;
use crate::payer::Payer;
use crate::store::StorageKey::{Withdraw};
use crate::store::{get_pay_asset_info, TransferInfo, ADMIN, LAST_BURN};
use soroban_sdk::{vec, Address, Env, String, Vec};
use crate::commission::Commission;
use crate::store;

pub struct Burn;

impl Burn {
    fn update_withdraw_records(env: &Env, transfer_info: TransferInfo) -> Result<(), Error> {
        let storage_key = Withdraw(transfer_info.clone().transfer);
        let withdraw = if env.storage().persistent().has(&storage_key) {
            let mut existing_transfers: Vec<TransferInfo> =
                env.storage().persistent().get(&storage_key).unwrap();
            existing_transfers.push_back(transfer_info);
            existing_transfers
        } else {
            vec![env, transfer_info]
        };
        env.storage().persistent().set(&storage_key, &withdraw);
        Ok(())
    }
    /// Calls the 'burn' function of the 'contract' with 'amount' to burn payer assets.
    pub fn burn(
        env: Env,
        from: String,
        payout: String,
        amount: i128,
        fee: i128,
    ) -> Result<(), Error> {
        // Verify the amount is positive after commission deduction.
        if amount - fee < 0 {
            return Err(Error::NegativeAmount);
        }
        
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        if env.storage().persistent().has(&Withdraw(payout.clone())) {
            return Err(Error::AlreadyInitialized);
        }
        let transfer_info = TransferInfo {
            transfer: payout,
            payer: from.clone(),
            beneficiary: from.clone(),
            amount,
            fee,
            date: Option::from(env.ledger().timestamp()),
        };
        Self::update_withdraw_records(&env, transfer_info)?;
        let pay_asset = get_pay_asset_info(&env)?;
        let client = MintClient::new(&env, &pay_asset.contract);
        let from_acc = Payer::payer(env.clone(), from);

        client.clawback(&from_acc, &(amount));
        Ok(())
    }
    
    fn get_withdraw_records(env: &Env, payout: String) -> Result<TransferInfo, Error> {
        let storage_key = Withdraw(payout);
        let withdraw = env.storage().persistent().get(&storage_key).unwrap();
        Ok(withdraw)
    }
    
    fn delete_withdraw_records(env: &Env, payout: String) -> Result<(), Error> {
        let storage_key = Withdraw(payout);
        if env.storage().persistent().has(&storage_key) {
            env.storage().persistent().remove(&storage_key);
        }
        Ok(())
    }
    
    pub fn approve_burn(env: Env, payout: String) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();


        let withdraw_record = Self::get_withdraw_records(&env, payout.clone())?;
        let _ = Commission::pay_commission(env.clone(), &withdraw_record.fee);
     
        let last_burn = env.storage().persistent().get(&LAST_BURN).unwrap_or(0u64);
        env.storage().persistent().set(&store::StorageKey::Burn(last_burn), &withdraw_record);
        env.storage().persistent().set(&LAST_BURN, &(last_burn + 1));
        
        Self::delete_withdraw_records(&env, payout)
    }
    
    pub fn reject_burn(env: Env, payout: String) -> Result<(), Error> {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        let pay_asset = get_pay_asset_info(&env)?;
        let client = MintClient::new(&env, &pay_asset.contract);
        let withdraw_record = Self::get_withdraw_records(&env, payout.clone())?;
        let payer_account = Payer::payer(env.clone(), withdraw_record.payer);
        
        client.mint(&payer_account, &(withdraw_record.amount));
     
        Self::delete_withdraw_records(&env, payout)
    }
}
