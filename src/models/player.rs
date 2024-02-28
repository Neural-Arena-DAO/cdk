use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use super::nft::Nft;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Player {
    pub id: Principal,
    pub nft: Nft,
}