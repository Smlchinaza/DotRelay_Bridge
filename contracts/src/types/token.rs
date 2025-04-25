//! Token-related structs

use ink_primitives::Hash;
use ink_prelude::vec::Vec;
use scale::{Encode, Decode};

#[derive(Debug, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct AssetInfo {
    pub chain_id: u32,
    pub asset_id: Hash,
    pub name: Vec<u8>,
    pub symbol: Vec<u8>,
    pub decimals: u8,
}
