#![cfg_attr(not(feature = "std"), no_std)]
//! Fee computation logic

use ink_env::Balance;
use crate::types::errors::Error;
use scale::{Encode, Decode};

#[derive(Debug, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct FeeCalculator {
    pub base_fee: Balance,
    pub per_unit_fee: Balance,
}

impl FeeCalculator {
    pub fn new(base_fee: Balance, per_unit_fee: Balance) -> Self {
        Self { base_fee, per_unit_fee }
    }

    pub fn calculate_fee(&self, _dest_para: u32, amount: Balance) -> Result<Balance, Error> {
        // simple linear fee: base + per_unit * amount
        Ok(self.base_fee + self.per_unit_fee.saturating_mul(amount))
    }
}
