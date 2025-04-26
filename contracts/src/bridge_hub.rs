//! Core coordination contract

use ink_env::AccountId;
use ink_env::Balance;
use ink_primitives::Hash;

use crate::token_registry::TokenRegistry;
use crate::transfer_manager::TransferManager;
use crate::fee_calculator::FeeCalculator;
use crate::xcm_handler::XcmHandler;
use crate::event_logger::EventLogger;
use crate::security::Security;
use crate::types::errors::Error;
use crate::types::transfer::{TransferStatus, TransferRecord};

#[derive(Debug, Default, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct BridgeHub {
    pub next_transfer_id: u64,
    pub paused: bool,
    pub token_registry: TokenRegistry,
    pub transfer_manager: TransferManager,
    pub fee_calculator: FeeCalculator,
    pub owner: AccountId,
}

impl BridgeHub {
    pub fn new(owner: AccountId) -> Self {
        Self {
            next_transfer_id: 1,
            paused: false,
            token_registry: TokenRegistry::default(),
            transfer_manager: TransferManager::default(),
            fee_calculator: FeeCalculator::default(),
            owner,
        }
    }

    pub fn initiate_cross_chain_transfer(
        &mut self,
        dest_para: u32,
        recipient: AccountId,
        amount: Balance,
        asset_id: Hash,
    ) -> Result<u64, Error> {
        // Ensure contract not paused
        Security::ensure_not_paused(self.paused)?;
        // Validate token registration
        self.token_registry.validate_token(dest_para, asset_id)?;
        // Compute fee
        let fee = self.fee_calculator.calculate_fee(dest_para, amount)?;
        // Assign transfer ID
        let id = self.next_transfer_id;
        self.next_transfer_id += 1;
        // Lock funds and fees
        let record = TransferRecord {
            id,
            dest_para,
            recipient,
            asset_id,
            amount,
            status: TransferStatus::Initiated,
        };
        self.transfer_manager.lock_funds(record)?;
        // Build and dispatch XCM message
        let payload = XcmHandler::construct_message(dest_para, recipient, asset_id, amount);
        // TODO: send payload via XCM executor
        // Mark in-progress
        self.transfer_manager.update_status(id, TransferStatus::InProgress)?;
        // Log event
        EventLogger::log_initiated(id, dest_para, recipient, asset_id, amount);
        Ok(id)
    }

    pub fn receive_cross_chain_assets(
        &mut self,
        transfer_id: u64,
        amount: Balance,
        asset_id: Hash,
    ) -> Result<(), Error> {
        // Ensure contract not paused
        Security::ensure_not_paused(self.paused)?;
        // Update status to completed
        self.transfer_manager.update_status(transfer_id, TransferStatus::Completed)?;
        // TODO: release locked funds via PSP22
        // Log event
        EventLogger::log_completed(transfer_id);
        Ok(())
    }

    pub fn query_transfer_status(&self, transfer_id: u64) -> Result<TransferStatus, Error> {
        self.transfer_manager.get_status(transfer_id)
    }

    /// Admin: pause contract
    pub fn pause(&mut self, caller: AccountId) -> Result<(), Error> {
        Security::ensure_owner(caller, self.owner)?;
        if self.paused {
            return Err(Error::AlreadyPaused);
        }
        self.paused = true;
        Ok(())
    }

    /// Admin: unpause contract
    pub fn unpause(&mut self, caller: AccountId) -> Result<(), Error> {
        Security::ensure_owner(caller, self.owner)?;
        if !self.paused {
            return Err(Error::AlreadyUnpaused);
        }
        self.paused = false;
        Ok(())
    }

    /// Admin: update fee rates
    pub fn set_fee_rates(&mut self, caller: AccountId, base_fee: Balance, per_unit_fee: Balance) -> Result<(), Error> {
        Security::ensure_owner(caller, self.owner)?;
        Security::ensure_not_paused(self.paused)?;
        self.fee_calculator = FeeCalculator::new(base_fee, per_unit_fee);
        Ok(())
    }

    /// Admin: whitelist a token
    pub fn whitelist_token(&mut self, caller: AccountId, chain_id: u32, asset_id: Hash, metadata: Vec<u8>) -> Result<(), Error> {
        Security::ensure_owner(caller, self.owner)?;
        Security::ensure_not_paused(self.paused)?;
        self.token_registry.register_token(chain_id, asset_id, metadata)?;
        Ok(())
    }
}
