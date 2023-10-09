// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
extern crate poseidon_rs;
extern crate winterfell;
// use ff_ce::PrimeField;
mod air;
// mod prover;
mod verifier;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use stylus_sdk::{alloy_primitives::U256, prelude::*};   

use poseidon_rs::{Poseidon, Fr};

// use air::

// WINTERFELL
use winterfell::{StarkProof, verify, Air};

// ---------------
// use ff::PrimeField;

// #[derive(PrimeField)]
// #[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
// #[PrimeFieldGenerator = "7"]
// pub struct Fr(FrRepr);

sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}

/// Define an implementation of the generated Counter struct, defining a set_number
/// and increment method using the features of the Stylus SDK.
#[external]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }

    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.number.set(new_number);
        Ok(())
    }

    /// Increments number and updates it values in storage.
    pub fn increment(&mut self) -> Result<(), Vec<u8>> {
        let number = self.number.get();
        self.number.set(number + U256::from(1));
        Ok(())
    }

    pub fn poseidon_hash(&self, data: U256) -> Result<U256, Vec<u8>> {
        let bytes: [u8; 32] = data.to_be_bytes();
        
        let data_str = std::str::from_utf8(&bytes).unwrap();
        
        // let b0: Fr = Fr::from_str(
        //     "21888242871839275222246405745257275088548364400416034343698204186575808495619",
        // ).unwrap_of_default();

        // let arr: Vec<Fr> = vec![b0];
        // let poseidon = Poseidon::new();  
        // let hash = poseidon.hash(arr).unwrap();
        Ok(data)
    }

    // pub fn proof(&mut self)

    pub fn verification(&self) -> Result<U256, Vec<u8>> {
        verifier::main();

        Ok(U256::from(1))
    }
 
}
