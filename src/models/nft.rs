use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use super::nas1_types::Nas1Collection;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub enum NftStandard {
    ICRC1,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Nft {
    pub std: NftStandard, 
    pub canister_id: Principal,
    pub token_id: u128,
    pub info: Nas1Collection,
}

