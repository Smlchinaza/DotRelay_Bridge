#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;

pub mod bridge_hub;
pub mod token_registry;
pub mod transfer_manager;
pub mod xcm_handler;
pub mod security;
pub mod fee_calculator;
pub mod event_logger;
pub mod admin;
pub mod types;
#[cfg(test)]
pub mod tests;
pub mod traits;

#[ink::contract]
pub mod dotrelay_bridge {
    use super::*;
    #[ink(storage)]
    pub struct DotRelayBridge {
        hub: bridge_hub::BridgeHub,
    }

    impl DotRelayBridge {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self { hub: bridge_hub::BridgeHub::new(caller) }
        }

        #[ink(message)]
        pub fn initiate_transfer(
            &mut self,
            dest_para: u32,
            recipient: AccountId,
            asset_id: Hash,
            amount: Balance,
        ) -> Result<u64, types::errors::Error> {
            self.hub.initiate_cross_chain_transfer(dest_para, recipient, amount, asset_id)
        }

        #[ink(message)]
        pub fn receive_assets(
            &mut self,
            transfer_id: u64,
            amount: Balance,
            asset_id: Hash,
        ) -> Result<(), types::errors::Error> {
            self.hub.receive_cross_chain_assets(transfer_id, amount, asset_id)
        }

        #[ink(message)]
        pub fn query_status(
            &self,
            transfer_id: u64,
        ) -> Result<types::transfer::TransferStatus, types::errors::Error> {
            self.hub.query_transfer_status(transfer_id)
        }

        /// Pause contract (admin only)
        #[ink(message)]
        pub fn pause(&mut self) -> Result<(), types::errors::Error> {
            let caller = Self::env().caller();
            self.hub.pause(caller)
        }

        /// Unpause contract (admin only)
        #[ink(message)]
        pub fn unpause(&mut self) -> Result<(), types::errors::Error> {
            let caller = Self::env().caller();
            self.hub.unpause(caller)
        }

        /// Set fee rates (admin only)
        #[ink(message)]
        pub fn set_fee_rates(&mut self, base_fee: Balance, per_unit_fee: Balance) -> Result<(), types::errors::Error> {
            let caller = Self::env().caller();
            self.hub.set_fee_rates(caller, base_fee, per_unit_fee)
        }

        /// Whitelist a token (admin only)
        #[ink(message)]
        pub fn whitelist_token(&mut self, chain_id: u32, asset_id: Hash, metadata: Vec<u8>) -> Result<(), types::errors::Error> {
            let caller = Self::env().caller();
            self.hub.whitelist_token(caller, chain_id, asset_id, metadata)
        }
    }
}