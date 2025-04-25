//! Transfer record structs

use ink_env::{AccountId, Balance};
use ink_primitives::Hash;
use scale::{Encode, Decode};

#[derive(Debug, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum TransferStatus {
    Initiated,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct TransferRecord {
    pub id: u64,
    pub dest_para: u32,
    pub recipient: AccountId,
    pub asset_id: Hash,
    pub amount: Balance,
    pub status: TransferStatus,
}
