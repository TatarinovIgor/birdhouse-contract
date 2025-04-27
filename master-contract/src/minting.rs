use crate::deployer::Deployer;
use crate::error::Error;
use crate::payer::Payer;
use crate::store::StorageKey::Payments;
use crate::store::{OrderInfo, PaymentInfo, StorageKey, ADMIN};
use soroban_sdk::{contractclient, vec, Address, Env, String, Vec};
use crate::commission::Commission;

#[contractclient(name = "MintClient")]
trait MintInterface {
    fn mint(env: Env, to: Address, amount: i128);
    fn set_admin(env: Env, new_admin: Address);
    fn clawback(env: Env, from: Address, amount: i128);
    fn set_authorized(env: Env, id: &Address, authorize: &bool);
}

pub struct Minter;

impl Minter {
    /// Calls the 'mint' function of the 'contract' with 'order', 'payment', 'payer' and 'amount'.
    /// If the order wasn't registered before by the 'deploy' function, it will be created, and
    /// the admin address will be assigned as an issuer
    /// the function will issue the 'amount' assets associated with the 'order'
    pub fn mint(
        env: Env,
        order: String,
        payment: String,
        payer: String,
        amount: i128,
        fee: i128,
    ) -> Result<(), Error> {
        // Verify the amount is positive after commission deduction
        if amount - fee <= 0 {
            return Err(Error::NegativeAmount);
        }
        
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        admin.require_auth();

        // check if the order doesn't exist
        if env
            .storage()
            .persistent()
            .has(&StorageKey::Order(order.clone()))
            == false
        {
            // create order
            Deployer::deploy(env.clone(), order.clone(), admin.clone());
        }
        // Get order info
        let order_info: OrderInfo = env
            .storage()
            .persistent()
            .get(&StorageKey::Order(order.clone()))
            .unwrap();

        let date = Option::from(env.ledger().timestamp());

        // Update information about payment operations
        if env.storage().persistent().has(&Payments(
            order_info.code.clone(),
            order_info.issuer.clone(),
        )) {
            let mut recorded_payments: Vec<PaymentInfo> = env
                .storage()
                .persistent()
                .get(&Payments(
                    order_info.code.clone(),
                    order_info.issuer.clone(),
                ))
                .unwrap();
            recorded_payments.push_back(PaymentInfo {
                payer: payer.clone(),
                payment,
                amount,
                fee,
                date,
            });
            env.storage().persistent().set(
                &Payments(order_info.code.clone(), order_info.issuer.clone()),
                &recorded_payments,
            );
        } else {
            let create_payment = vec![
                &env,
                PaymentInfo {
                    payer: payer.clone(),
                    payment,
                    amount,
                    fee,
                    date,
                },
            ];
            env.storage().persistent().set(
                &Payments(order_info.code.clone(), order_info.issuer.clone()),
                &create_payment,
            );
        }

        // Get address for payer
        let to = Payer::payer(env.clone(), payer);
        // Perform the mint.
        let client = MintClient::new(&env, &order_info.contract);
        client.mint(&to, &(amount - fee));
        let _ = Commission::pay_commission(env.clone(), &fee);
        Ok(())
    }
}
