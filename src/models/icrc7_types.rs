use std::collections::HashMap;
use candid::CandidType;
use serde::Deserialize;
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue,
    icrc1::account::{Account, Subaccount},
};
use super::nas1_types::Nas1Token;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TransferArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TransferError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u128 },
    GenericError { error_code: u128, message: String },
    GenericBatchError { error_code: u128, message: String },
}

pub type Icrc7TokenMetadata = HashMap<String, MetadataValue>;

#[derive(CandidType, Deserialize, Clone)]
pub struct MintArg {
    pub from_subaccount: Option<Subaccount>,
    pub to: Account,
    pub memo: Option<Vec<u8>>,
    pub nas1: Nas1Token,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct BurnArg {
    pub from_subaccount: Option<Subaccount>,
    pub token_id: u128,
    pub memo: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize)]
pub struct Standard {
    pub name: String,
    pub url: String,
}
