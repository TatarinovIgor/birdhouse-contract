use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    BadSignatureOrder = 2,
    NotEnoughSigners = 3,
    InvalidContext = 4,
    BadArgs = 5,
    NotInitialized = 6,
    UnknownSigner = 7,
}