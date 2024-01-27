// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{
    alloy_sol_types::sol,
    evm,
    prelude::*,
};

sol! {
    event Event();
}

sol_storage! {
    #[entrypoint]
    pub struct Contract {}
}

#[external]
impl Contract {
    pub fn with_mutable_reference_to_self(&mut self) -> Result<(), Vec<u8>> {
        evm::log(Event {});

        Ok(())
    }

    pub fn with_reference_to_self(&self) -> Result<(), Vec<u8>> {
        evm::log(Event {});

        Ok(())
    }

    pub fn without_self() -> Result<(), Vec<u8>> {
        evm::log(Event {});

        Ok(())
    }
}
