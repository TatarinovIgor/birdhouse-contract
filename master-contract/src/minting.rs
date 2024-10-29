use soroban_sdk::{contractclient, vec, Address, Env, String};
use crate::error::{Error};
use crate::payer::Payer;
use crate::store::{AssetInfo, OrderInfo, PaymentInfo, StorageKey, ADMIN};


#[contractclient(name = "MintClient")]
trait MintInterface {
    fn mint(env: Env, to: Address, amount: i128);
    fn set_admin(env: Env, new_admin: Address);
    fn clawback(env: Env, from: Address, amount: i128);
    fn set_authorized(env: Env, id: &Address, authorize: &bool);
}

pub struct Minter;

impl Minter {

    /// Calls the 'mint' function of the 'contract' with 'to' and 'amount'.
    pub fn mint(
        env: Env,
        order: String,
        payment: String,
        payer: String,
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
        let mut asset_info : AssetInfo = env.storage().persistent()
            .get(&StorageKey::Asset(order_info.code.clone(), order_info.issuer.clone()))
            .unwrap();

        let date = Option::from(env.ledger().timestamp());
        // Update information about payment operations

        if asset_info.payments == None {
            let create_payment = vec!(&env, PaymentInfo{payment, amount, date});
            asset_info.payments = Option::from(create_payment);
        } else {
            let mut recorded_payments = asset_info.payments.unwrap();
            recorded_payments.push_back(PaymentInfo{payment, amount, date });
            asset_info.payments = Option::from(recorded_payments);
        }


        if asset_info.payer == None {
            asset_info.payer = Option::from(payer.clone());
        }

        // Get address for payer
        let to = Payer::payer(env.clone(), payer);
        // Perform the mint.
        let client = MintClient::new(&env, &order_info.contract);
        client.mint(&to, &amount);

        // Store information about payment operations
        env.storage().persistent().set(&StorageKey::Asset(
            order_info.code.clone(), order_info.issuer.clone()), &asset_info);

        Ok(())
    }
}
