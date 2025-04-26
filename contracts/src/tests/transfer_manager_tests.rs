#![cfg(test)]
use crate::transfer_manager::TransferManager;
use crate::types::transfer::{TransferRecord, TransferStatus};
use crate::types::errors::Error;
use ink_env::AccountId;
use ink_primitives::Hash;

#[test]
fn lock_and_get_status() {
    let mut tm = TransferManager::default();
    let record = TransferRecord {
        id: 1,
        dest_para: 2,
        recipient: AccountId::default(),
        asset_id: Hash::default(),
        amount: 100,
        status: TransferStatus::Initiated,
    };
    assert!(tm.lock_funds(record.clone()).is_ok());
    assert_eq!(tm.get_status(1).unwrap(), TransferStatus::Initiated);
}

#[test]
fn update_status() {
    let mut tm = TransferManager::default();
    let record = TransferRecord {
        id: 1,
        dest_para: 2,
        recipient: AccountId::default(),
        asset_id: Hash::default(),
        amount: 100,
        status: TransferStatus::Initiated,
    };
    tm.lock_funds(record).unwrap();
    assert!(tm.update_status(1, TransferStatus::Completed).is_ok());
    assert_eq!(tm.get_status(1).unwrap(), TransferStatus::Completed);
}

#[test]
fn unknown_transfer() {
    let tm = TransferManager::default();
    assert_eq!(tm.get_status(1), Err(Error::TransferNotFound));
    assert_eq!(tm.update_status(1, TransferStatus::Failed), Err(Error::TransferNotFound));
}
