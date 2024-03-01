use candid::CandidType;
use serde::{Deserialize, Serialize};
use super::nas1_types::Nas1Collection;

#[derive(Clone, PartialEq, Serialize, Deserialize, CandidType)]
pub enum NftStandard {
    ICRC7,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct NftCollectionMetadata {
    pub std: NftStandard,
    pub name: String,
    pub desc: String,
    pub logo: String,
    pub nas1: Nas1Collection,
}
