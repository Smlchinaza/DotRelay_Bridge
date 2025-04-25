//! Custom error types

use scale::{Encode, Decode};
#[derive(Debug, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    InsufficientBalance,
    InvalidAmount,
    NotAuthorized,
    Paused,
    TransferNotFound,
    TokenNotRegistered,
    XcmParseError,
    FeeCalculationFailed,
    AdminError,
}
