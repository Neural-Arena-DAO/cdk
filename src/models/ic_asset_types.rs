use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, CandidType)]
pub struct GetRequest {
    pub key: String,
    pub accept_encodings: Vec<String>,
}

#[derive(Deserialize, CandidType)]
pub struct GetResponse {
    pub content: Vec<u8>,
    pub sha256: Option<Vec<u8>>,
    pub content_type: String,
    pub content_encoding: String,
    pub total_length: u128,
}