use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotAuthorizedMinter = 2,
    DailyLimitInsufficient = 3,
    NegativeAmount = 4,
    BadSignatureOrder = 5,
    NotEnoughSigners = 6,
    InvalidContext = 7,
    BadArgs = 8,
    NotInitialized = 9,
    UnknownSigner = 10,
    IncorrectTransfer = 11,
}