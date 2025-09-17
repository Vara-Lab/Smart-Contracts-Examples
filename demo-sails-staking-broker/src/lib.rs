// #![no_std]

// #[cfg(target_arch = "wasm32")]
// pub use contract_app::wasm::*;

// #[cfg(feature = "wasm-binary")]
// #[cfg(not(target_arch = "wasm32"))]
// pub use code::WASM_BINARY_OPT as WASM_BINARY;

// #[cfg(feature = "wasm-binary")]
// #[cfg(not(target_arch = "wasm32"))]
// mod code {
//     include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
// }


#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod code {
    include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
}

#[cfg(feature = "std")]
pub use code::WASM_BINARY_OPT as WASM_BINARY;

#[cfg(not(feature = "std"))]
pub const WASM_BINARY: &[u8] = &[];

#[cfg(not(feature = "std"))]
// pub mod wasm;
pub mod contract_app::wasm;
