use soroban_sdk::{contractclient, vec, Address, Env, String, Vec};
use crate::error::{Error};
use crate::payer::Payer;
use crate::store::{OrderInfo, PaymentInfo, StorageKey, ADMIN};
use crate::store::StorageKey::Payments;

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
        if amount <= 0 {
            return Err(Error::NegativeAmount);
        }
        // Get order info
        let order_info: OrderInfo = env.storage().persistent()
            .get(&StorageKey::Order(order.clone())).unwrap();

        let date = Option::from(env.ledger().timestamp());

        // Update information about payment operations
        if env.storage().persistent()
            .has(&Payments(order_info.code.clone(), order_info.issuer.clone())) {
            let mut recorded_payments : Vec<PaymentInfo> = env.storage().persistent()
                .get(&Payments(order_info.code.clone(), order_info.issuer.clone())).unwrap();
            recorded_payments.push_back(PaymentInfo { payment, amount, date });
            env.storage()
                .persistent()
                .set(&Payments(order_info.code.clone(), order_info.issuer.clone()),
                     &recorded_payments);
        } else {
            let create_payment = vec!(&env, PaymentInfo { payment, amount, date });
            env.storage()
                .persistent()
                .set(&Payments(order_info.code.clone(), order_info.issuer.clone()), &create_payment);
        }

        // Get address for payer
        let to = Payer::payer(env.clone(), payer);
        // Perform the mint.
        let client = MintClient::new(&env, &order_info.contract);
        client.mint(&to, &amount);

        Ok(())
    }
}
