//! Token mapping and validation

use ink_env::Hash;
use ink_env::AccountId;
use crate::types::errors::Error;

#[derive(Debug, Default, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct TokenRegistry {
    // mapping chain_id -> asset_id -> metadata
}

impl TokenRegistry {
    pub fn register_token(
        &mut self,
        chain_id: u32,
        asset_id: Hash,
        metadata: Vec<u8>,
    ) -> Result<(), Error> {
        // TODO: implement
        Ok(())
    }

    pub fn validate_token(
        &self,
        chain_id: u32,
        asset_id: Hash,
    ) -> Result<(), Error> {
        // TODO: implement
        Ok(())
    }
}
