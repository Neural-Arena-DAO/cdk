use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Standard {
    pub name: String,
    pub url: String,
}
