use super::random;

pub fn generate(
) -> String {
    let (lsu64, msu64) = random::next_u64_pair();
    
    ulid::Ulid::from_parts(
        ic_cdk::api::time() / 1000000, 
        (msu64 as u128) << 64 | (lsu64 as u128)
    ).to_string()
}

