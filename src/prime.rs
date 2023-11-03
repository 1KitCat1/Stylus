// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;


/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use stylus_sdk::{alloy_primitives::U256, prelude::*};   

sol_storage! {
    #[entrypoint]
    pub struct Prime {
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Prime {
    pub fn isPrime(&self, number:U256) -> Result<bool, Vec<u8>> {
        let mut divisor = U256::from(1);
        let mut _isPrime = true;
        while divisor*divisor <= number {
            divisor += U256::from(1);
            if number % divisor == U256::from(0) {
                _isPrime = false;
            }
        }
        Ok(_isPrime)
    }
 
}
