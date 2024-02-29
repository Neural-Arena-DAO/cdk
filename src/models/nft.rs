use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use super::nas1_types::{Nas1Collection, Nas1Token};

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub enum NftStandard {
    ICRC1,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Nft {
    pub std: NftStandard, 
    pub canister_id: Principal,
    pub token_id: u128,
    pub collection_data: Nas1Collection,
    pub token_data: Nas1Token,
}

impl PartialEq for Nft {
    fn eq(
        &self, 
        other: &Self
    ) -> bool {
        self.canister_id == other.canister_id && 
        self.token_id == other.token_id
    }
}

