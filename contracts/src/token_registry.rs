//! Token mapping and validation

use ink_env::Hash;
use ink_env::AccountId;
use crate::types::errors::Error;
use ink_storage::Mapping;
use ink_storage::traits::{SpreadLayout, PackedLayout, StorageLayout};

#[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, StorageLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct AssetInfo {
    pub metadata: Vec<u8>,
}

#[derive(Debug, Default, scale::Encode, scale::Decode, SpreadLayout, PackedLayout, StorageLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct TokenRegistry {
    assets: Mapping<(u32, Hash), AssetInfo>,
}

impl TokenRegistry {
    pub fn register_token(
        &mut self,
        chain_id: u32,
        asset_id: Hash,
        metadata: Vec<u8>,
    ) -> Result<(), Error> {
        let info = AssetInfo { metadata };
        self.assets.insert((chain_id, asset_id), &info);
        Ok(())
    }

    pub fn validate_token(
        &self,
        chain_id: u32,
        asset_id: Hash,
    ) -> Result<(), Error> {
        self.assets.get((chain_id, asset_id)).ok_or(Error::TokenNotRegistered)?;
        Ok(())
    }
}
