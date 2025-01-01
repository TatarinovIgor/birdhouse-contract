use crate::contract::DemandContract;
use crate::error::Error;
use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contracttype,
    crypto::Hash,
    BytesN, Env, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct AccSignature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

impl CustomAccountInterface for DemandContract {
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
        _: Vec<Context>,
    ) -> Result<(), Error> {
        // Perform authentication.
        authenticate(&env, &signature_payload, &signatures)?;
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
