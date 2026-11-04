//! # ABI Barrier (`abi-barrier`)
//!
//! Universal, Zero-Cost & Unbreakable C-ABI Safe Boundary for Rust.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate self as abi_barrier;

#[doc(hidden)]
pub use abi_barrier_core as __core;
pub use abi_barrier_core as core;

pub use abi_barrier_core::{
    AbiError, AbiReturn, CallingConvention, PanicStrategy, catch_panic_barrier, ensure_non_null,
    ensure_valid_c_str,
};

#[cfg(feature = "stitch")]
pub use abi_barrier_core::AbiBarrierLayer;

#[cfg(feature = "derive")]
pub use abi_barrier_derive::barrier;

/// Common prelude for `abi-barrier`.
pub mod prelude {
    pub use abi_barrier_core::{
        AbiError, AbiReturn, CallingConvention, PanicStrategy, catch_panic_barrier,
        ensure_non_null, ensure_valid_c_str,
    };

    #[cfg(feature = "stitch")]
    pub use abi_barrier_core::AbiBarrierLayer;

    #[cfg(feature = "derive")]
    pub use abi_barrier_derive::barrier;
}
