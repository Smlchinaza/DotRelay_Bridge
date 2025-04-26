//! XCM message processing

use ink_env::{AccountId, Balance};
use ink_primitives::Hash;
use ink_prelude::vec::Vec;
use crate::types::errors::Error;
use scale::{Encode, Decode};

pub struct XcmHandler;

impl XcmHandler {
    /// Build XCM message payload
    pub fn construct_message(
        dest_para: u32,
        recipient: AccountId,
        asset_id: Hash,
        amount: Balance,
    ) -> Vec<u8> {
        (dest_para, recipient, asset_id, amount).encode()
    }

    /// Parse incoming XCM message payload
    pub fn parse_message(
        payload: Vec<u8>,
    ) -> Result<(u32, AccountId, Balance, Hash), Error> {
        Decode::decode(&mut &payload[..]).map_err(|_| Error::XcmParseError)
    }
}
