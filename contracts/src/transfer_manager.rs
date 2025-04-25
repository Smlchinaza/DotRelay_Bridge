//! Transfer state management

use ink_env::{AccountId, Balance};
use ink_primitives::Hash;
use crate::types::transfer::{TransferRecord, TransferStatus};
use crate::types::errors::Error;
use ink_storage::collections::BTreeMap as StorageMap;
use scale::{Encode, Decode};

#[derive(Debug, Default, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct TransferManager {
    transfers: StorageMap<u64, TransferRecord>,
}

impl TransferManager {
    pub fn lock_funds(&mut self, record: TransferRecord) -> Result<(), Error> {
        // TODO: reserve tokens via PSP22 or currency interface
        self.transfers.insert(record.id, record);
        Ok(())
    }

    pub fn update_status(&mut self, transfer_id: u64, status: TransferStatus) -> Result<(), Error> {
        let mut record = self.transfers.get(&transfer_id).ok_or(Error::TransferNotFound)?;
        record.status = status.clone();
        self.transfers.insert(transfer_id, record);
        Ok(())
    }

    pub fn get_status(&self, transfer_id: u64) -> Result<TransferStatus, Error> {
        let record = self.transfers.get(&transfer_id).ok_or(Error::TransferNotFound)?;
        Ok(record.status.clone())
    }
}
