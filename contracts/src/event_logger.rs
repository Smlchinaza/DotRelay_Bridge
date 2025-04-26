//! Standardized event emission

use ink_env::{AccountId, Balance};
use ink_primitives::Hash;

pub struct EventLogger;

impl EventLogger {
    pub fn log_initiated(_transfer_id: u64, _dest_para: u32, _recipient: AccountId, _asset_id: Hash, _amount: Balance) {
        // stub: no-op
    }

    pub fn log_completed(_transfer_id: u64) {
        // stub: no-op
    }
}
