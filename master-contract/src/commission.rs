use soroban_sdk::{Address, Env};
use crate::error::Error;
use crate::minting::MintClient;
use crate::store::{get_pay_asset_info, ADMIN, FEE_ACCOUNT};

pub struct Commission;

impl Commission {
    /// Return the admin address.
    pub fn commission_account(env: Env) -> Result<Address, Error> {
        if env.storage().persistent().has(&FEE_ACCOUNT) {
            return env.storage().persistent().get(&FEE_ACCOUNT).unwrap();
        }
        Err(Error::NotInitialized)
    }

    /// Set the commission address.
    pub fn set_commission_account(env: Env, commission_account: Address) {
        if let Some(admin) = env
            .storage()
            .persistent()
            .get::<_, Address>(&ADMIN)
        {
            admin.require_auth();
        };
        env.storage().persistent().set(&FEE_ACCOUNT, &commission_account);
    }
    
    pub fn pay_commission(env: Env, fee: &i128) -> Result<(), Error>  {
        let pay_asset = get_pay_asset_info(&env)?;
        let client = MintClient::new(&env, &pay_asset.contract);
        let commission_account = Commission::commission_account(env.clone());
        if !commission_account.is_err() {
            client.mint(&commission_account?, fee);
        }
        Ok(())
    }
}