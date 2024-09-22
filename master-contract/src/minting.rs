use soroban_sdk::{contract, contractclient, contractimpl, contracttype, Address, Env, IntoVal};
use crate::error::{Error};
use crate::store::{ADMIN};


#[contractclient(name = "MintClient")]
trait MintInterface {
    fn mint(env: Env, to: Address, amount: i128);
}

#[contract]
pub struct Minter;

#[contracttype]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinterConfig {
    limit: i128,
    epoch_length: u32,
}

#[contracttype]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct MinterStats {
    consumed_limit: i128,
}

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

    /// Calls the 'mint' function of the 'contract' with 'to' and 'amount'.
    /// Authorized by the 'minter'. Uses some of the authorized 'minter's
    /// current epoch's limit.
    pub fn mint(
        env: Env,
        contract: Address,
        minter: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {
        // Verify minter is authenticated, and authorizing args.
        minter.require_auth_for_args((&contract, &to, amount).into_val(&env));

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
