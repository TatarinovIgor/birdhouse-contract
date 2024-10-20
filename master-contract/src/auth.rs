use soroban_sdk::{auth::{Context, CustomAccountInterface}, contracttype, crypto::Hash,
                  symbol_short, Address, BytesN, Env, Symbol, Vec};
use crate::error::Error;
use crate::contract::PaymentContract;
use crate::store::{ADMIN};

#[contracttype]
#[derive(Clone)]
pub struct AccSignature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

const SET_ADMIN_FN: Symbol = symbol_short!("set_admin");


impl CustomAccountInterface for PaymentContract {
    type Signature = Vec<AccSignature>;
    type Error = Error;

    // This is the 'entry point' of the account contract and every account
    // contract has to implement it. `require_auth` calls for the Address of
    // this contract will result in calling this `__check_auth` function with
    // the appropriate arguments.
    //
    // This should return `()` if authentication and authorization checks have
    // been passed and return an error (or panic) otherwise.
    //
    // `__check_auth` takes the payload that needed to be signed, arbitrarily
    // typed signatures (`Vec<AccSignature>` contract type here) and authorization
    // context that contains all the invocations that this call tries to verify.
    //
    // `__check_auth` has to authenticate the signatures. It also may use
    // `auth_context` to implement additional authorization policies (like token
    // spend limits here).
    //
    // Soroban host guarantees that `__check_auth` is only being called during
    // `require_auth` verification and hence this may mutate its own state
    // without the need for additional authorization (for example, this could
    // store per-time-period token spend limits instead of just enforcing the
    // limit per contract call).
    //
    // Note, that `__check_auth` function shouldn't call `require_auth` on the
    // contract's own address in order to avoid infinite recursion.
    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        signature_payload: Hash<32>,
        signatures: Vec<AccSignature>,
        auth_context: Vec<Context>,
    ) -> Result<(), Error> {

        // Perform authentication.
        authenticate(&env, &signature_payload, &signatures)?;

        let curr_contract = env.current_contract_address();
        let all_signed = true;
        // This is a map for tracking the token spend limits per token. This
        // makes sure that if e.g. multiple `transfer` calls are being authorized
        // for the same token we still respect the limit for the total
        // transferred amount (and not the 'per-call' limits).
        //let mut spend_left_per_token = Map::<Address, i128>::new(&env);

        // Verify the authorization policy.
        for context in auth_context.iter() {
            verify_authorization_policy(
                &env,
                &context,
                &curr_contract,
                all_signed,
                &signatures,
            )?;
            return Err(Error::BadSignatureOrder);
        }
        Ok(())
    }
}

fn authenticate(
    env: &Env,
    signature_payload: &Hash<32>,
    signatures: &Vec<AccSignature>,
) -> Result<(), Error> {
    for i in 0..signatures.len() {
        let signature = signatures.get_unchecked(i);
        if i > 0 {
            let prev_signature = signatures.get_unchecked(i - 1);
            if prev_signature.public_key >= signature.public_key {
                return Err(Error::BadSignatureOrder);
            }
        }
        env.crypto().ed25519_verify(
            &signature.public_key,
            &signature_payload.clone().into(),
            &signature.signature,
        );
    }
    Ok(())
}

fn verify_authorization_policy(
    env: &Env,
    context: &Context,
    curr_contract: &Address,
    all_signed: bool,
    signatures: &Vec<AccSignature>,
) -> Result<(), Error> {
    let contract_context = match context {
        Context::Contract(c) => {
            if &c.contract == curr_contract {
                if !all_signed {
                    return Err(Error::NotEnoughSigners);
                }
            }
            c
        }
        Context::CreateContractHostFn(_) => return Err(Error::InvalidContext),
    };
    // For the account control every signer must sign the invocation.

    // Otherwise, we're only interested in functions that spend tokens.
    if contract_context.fn_name == SET_ADMIN_FN {
        return is_admin(env, signatures);
    }
    Err(Error::BadArgs)
}

fn is_admin(env: &Env, signatures: &Vec<AccSignature>) -> Result<(), Error> {
    if signatures.len() != 1 {
        return Err(Error::BadSignatureOrder);
    }
    if !env
        .storage()
        .persistent()
        .has(&ADMIN)
    {
        return Err(Error::NotInitialized);
    }
    let admin = env.storage().persistent().get::<_, Address>(&ADMIN).unwrap();
    let address = Address::from_string_bytes(
        signatures.get_unchecked(0).public_key.as_ref());
    if admin != address {
        return Err(Error::UnknownSigner);
    }
    Ok(())
}
