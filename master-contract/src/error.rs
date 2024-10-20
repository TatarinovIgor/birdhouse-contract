use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    NotAuthorizedMinter = 1,
    DailyLimitInsufficient = 2,
    NegativeAmount = 3,
    BadSignatureOrder = 4,
    NotEnoughSigners = 5,
    InvalidContext = 6,
    BadArgs = 7,
    NotInitialized = 8,
    UnknownSigner = 9,
}