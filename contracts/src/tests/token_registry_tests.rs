#![cfg(test)]

use crate::token_registry::TokenRegistry;
use crate::types::errors::Error;
use ink_env::Hash;

#[test]
fn default_registry_has_no_tokens() {
    let registry = TokenRegistry::default();
    let chain_id = 1u32;
    let asset_id = Hash::default();
    assert_eq!(
        registry.validate_token(chain_id, asset_id),
        Err(Error::TokenNotRegistered)
    );
}

#[test]
fn register_and_validate_token() {
    let mut registry = TokenRegistry::default();
    let chain_id = 1u32;
    let asset_id = Hash::default();
    let metadata = b"metadata".to_vec();

    assert!(registry.register_token(chain_id, asset_id, metadata).is_ok());
    assert!(registry.validate_token(chain_id, asset_id).is_ok());
}
