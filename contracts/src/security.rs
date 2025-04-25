//! Security and validation functions

use ink_env::{AccountId, Balance};
use crate::types::errors::Error;

pub struct Security;

impl Security {
    /// Ensure contract is not paused
    pub fn ensure_not_paused(paused: bool) -> Result<(), Error> {
        if paused {
            return Err(Error::Paused);
        }
        Ok(())
    }

    /// Only allow owner to execute
    pub fn ensure_owner(caller: AccountId, owner: AccountId) -> Result<(), Error> {
        if caller != owner {
            return Err(Error::NotAuthorized);
        }
        Ok(())
    }

    /// Validate transfer amount is non-zero
    pub fn validate_amount(amount: Balance) -> Result<(), Error> {
        if amount == 0 {
            return Err(Error::InvalidAmount);
        }
        Ok(())
    }
}
