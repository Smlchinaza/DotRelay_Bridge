//! XCM message processing

use ink_env::{AccountId, Balance};
use ink_primitives::Hash;
use ink_prelude::vec::Vec;
use crate::types::errors::Error;

pub struct XcmHandler;

impl XcmHandler {
    /// Build XCM message payload
    pub fn construct_message(
        dest_para: u32,
        recipient: AccountId,
        asset_id: Hash,
        amount: Balance,
    ) -> Vec<u8> {
        // TODO: serialize into XCM format
        Vec::new()
    }

    /// Parse incoming XCM message payload
    pub fn parse_message(
        payload: Vec<u8>,
    ) -> Result<(u64, Balance, Hash), Error> {
        // TODO: parse XCM payload
        Err(Error::XcmParseError)
    }
}
