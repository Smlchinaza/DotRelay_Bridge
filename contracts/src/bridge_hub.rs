//! Core coordination contract

use ink_env::AccountId;
use ink_env::Balance;
use ink_primitives::Hash;

use crate::token_registry::TokenRegistry;
use crate::transfer_manager::TransferManager;
use crate::fee_calculator::FeeCalculator;
use crate::types::errors::Error;
use crate::types::transfer::TransferStatus;

#[derive(Debug, Default, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct BridgeHub {
    pub next_transfer_id: u64,
    pub paused: bool,
    pub token_registry: TokenRegistry,
    pub transfer_manager: TransferManager,
    pub fee_calculator: FeeCalculator,
}

impl BridgeHub {
    pub fn new() -> Self {
        Self {
            next_transfer_id: 1,
            paused: false,
            token_registry: TokenRegistry::default(),
            transfer_manager: TransferManager::default(),
            fee_calculator: FeeCalculator::default(),
        }
    }

    pub fn initiate_cross_chain_transfer(
        &mut self,
        dest_para: u32,
        recipient: AccountId,
        amount: Balance,
        asset_id: Hash,
    ) -> Result<u64, Error> {
        // TODO: implement validation, fee calc, token lock, XCM dispatch
        // increment transfer ID for uniqueness
        let id = self.next_transfer_id;
        self.next_transfer_id += 1;
        Ok(id)
    }

    pub fn receive_cross_chain_assets(
        &mut self,
        transfer_id: u64,
        amount: Balance,
        asset_id: Hash,
    ) -> Result<(), Error> {
        // TODO: implement asset reception and completion
        Ok(())
    }

    pub fn query_transfer_status(&self, transfer_id: u64) -> Result<TransferStatus, Error> {
        // TODO: query status from TransferManager
        Ok(TransferStatus::Completed)
    }
}
