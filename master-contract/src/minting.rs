use soroban_sdk::{contract, contractclient, contractimpl, Address, Env};
use crate::error::{Error};
use crate::store::{ADMIN};


#[contractclient(name = "MintClient")]
trait MintInterface {
    fn mint(env: Env, to: Address, amount: i128);
}

#[contract]
pub struct Minter;

#[contractimpl]
impl Minter {

    pub fn init(e: Env, admin: Address) -> Result<(), crate::store::Error> {
        if e.storage().persistent().has(&ADMIN) {
            return Err(crate::store::Error::AlreadyInitialized);
        }
        e.storage().persistent().set(&ADMIN, &admin);
        Ok(())
    }

    /// Return the admin address.
    pub fn admin(env: Env) -> Address {
        env.storage().persistent().get(&ADMIN).unwrap()
    }

    /// Set the admin.
    pub fn set_admin(env: Env, new_admin: Address) {
        if let Some(admin) = env
            .storage()
            .instance()
            .get::<_, Address>(&ADMIN)
        {
            admin.require_auth();
        };
        env.storage().instance().set(&ADMIN, &new_admin);
    }

    /// Calls the 'mint' function of the 'contract' with 'to' and 'amount'.
    /// Authorized by the 'minter'. Uses some of the authorized 'minter's
    /// current epoch's limit.
    pub fn mint(
        env: Env,
        contract: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {

        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // Verify amount is positive.
        if amount < 0 {
            return Err(Error::NegativeAmount);
        }

        // Perform the mint.
        let client = MintClient::new(&env, &contract);
        client.mint(&to, &amount);
        Ok(())
    }
}
