use alloc::string::{String};
use soroban_sdk::{Bytes, Env, Error, IntoVal, String as SdkString};
use soroban_sdk::xdr::{PublicKey, ToXdr, Uint256};
use base32;

pub struct CPAsset {
    pub(crate) code: [u8; 12],
    pub(crate) issuer: SdkString,
}

impl IntoVal<Env, Bytes> for CPAsset {
    fn into_val(&self, e: &Env) -> Bytes {
        self.to_xdr(e).unwrap()
    }
}

impl CPWriteXdr for CPAsset {
    fn to_xdr(&self, env: &Env) -> Result<Bytes, Error> {
        let mut buffer = Bytes::new(&env);
        let mut len:usize = 0;
        for c in self.code {
            if c == 0 {
                break
            }
            len += 1;
        }
        if len < 5 {
            let mut asset_code4:[u8; 4] = [0u8; 4];
            for i in 0.. 4 {
                asset_code4[i] = self.code[i]
            }
            buffer.append(&Bytes::from_array(&env, &[0, 0, 0, 1]));
            buffer.append(&Bytes::from_slice(&env, &asset_code4));
        } else {
            buffer.append(&Bytes::from_array(&env, &[0, 0, 0, 2]));
            buffer.append(&Bytes::from_slice(&env, &self.code[..12]));
        }

        let account_bytes: &mut [u8; 56] = &mut [0u8; 56];
        self.issuer.copy_into_slice(account_bytes);
        let account_str = String::from_utf8(account_bytes
            .to_vec()).map_err(|_| "Invalid UTF-8 sequence").unwrap();

        let (_, key_bytes) = decode(account_str.as_str());

        let mut key_array: [u8; 32] = [0; 32];
        if key_bytes.len() == 32 {
            key_array.copy_from_slice(&key_bytes);
            let public_key = PublicKey::PublicKeyTypeEd25519(Uint256(key_array));
            let acc_type: i32 = public_key.discriminant().into();
            buffer.append(&Bytes::from_slice(&env, acc_type.to_le_bytes().as_slice()));
            buffer.append(&Bytes::from_slice(&env, key_array.as_slice()));
        } else {
            let len: i32 = key_bytes.len().try_into().unwrap();
            buffer.append(&len.to_xdr(&env));
            buffer.append(&Bytes::from_slice(&env, key_bytes.as_slice()));
        }
        Ok(buffer)
    }
}

pub trait CPWriteXdr {
    fn to_xdr(&self, env: &Env) -> Result<Bytes, Error>;
}

pub fn decode(s: &str) -> (u8, alloc::vec::Vec<u8>) {
    let data = base32::decode(base32::Alphabet::Rfc4648 { padding: false }, s)
        .unwrap();
    let ver = data[0];
    let (data_without_crc, _) = data.split_at(data.len() - 2);
    let payload = &data_without_crc[1..];
    (ver, payload.to_vec())
}