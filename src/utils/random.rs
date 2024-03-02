use std::{cell::RefCell, thread::LocalKey};
use rand::{rngs::StdRng, RngCore, SeedableRng};

thread_local! {
    static RNG: RefCell<Option<StdRng>> = RefCell::default();
}

pub fn init() {
    RNG.with(|r| {
        let mut rng = r.borrow_mut();
        *rng = Some(StdRng::seed_from_u64(ic_cdk::api::time()));
    });
}

pub fn next_u64(
) -> u64 {
    RNG.with(|r| {
        let mut rng = r.borrow_mut();
        let rng = rng.as_mut().unwrap();
        rng.next_u64()
    })
}

pub fn next_u64_pair(
) -> (u64, u64) {
    RNG.with(|r| {
        let mut rng = r.borrow_mut();
        let rng = rng.as_mut().unwrap();
        (rng.next_u64(), rng.next_u64())
    })
}

pub fn get(
) -> &'static LocalKey<RefCell<Option<StdRng>>> {
    &RNG
}


//
// allow random on wasm32-unknown-unknown
//
#[cfg(target = "wasm32")]
fn always_fail(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

#[cfg(target = "wasm32")]
getrandom::register_custom_getrandom!(always_fail);

