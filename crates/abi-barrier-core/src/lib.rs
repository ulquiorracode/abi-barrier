//! Core `#![no_std]` runtime primitives, error types, and panic barrier logic.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

pub mod convention;
pub mod error;
pub mod panic_strategy;
pub mod ptr;
pub mod return_value;

#[cfg(feature = "stitch")]
pub mod stitch_layer;

pub use convention::CallingConvention;
pub use error::AbiError;
pub use panic_strategy::PanicStrategy;
pub use ptr::{ensure_non_null, ensure_valid_c_str};
pub use return_value::AbiReturn;

#[cfg(feature = "stitch")]
pub use stitch_layer::AbiBarrierLayer;

#[cfg(feature = "std")]
use std::io::Write;

#[cfg(feature = "std")]
/// Catches unhandled Rust panics at the C-ABI boundary, preventing undefined behavior (UB).
///
/// If a panic occurs, this function logs the panic diagnostic and executes the specified
/// [`PanicStrategy`].
#[inline(always)]
pub fn catch_panic_barrier<R, F>(
    fn_name: &'static str,
    strategy: PanicStrategy,
    f: F,
) -> Result<R, AbiError>
where
    F: FnOnce() -> R,
{
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(ret) => Ok(ret),
        Err(panic_payload) => {
            // Anti-silent suppression: log panic details explicitly without panicking on closed stderr
            let mut stderr = std::io::stderr().lock();
            if let Some(s) = panic_payload.downcast_ref::<&str>() {
                let _ = writeln!(
                    stderr,
                    "[ABI_BARRIER_PANIC] Caught panic in `{}`: {}",
                    fn_name, s
                );
            } else if let Some(s) = panic_payload.downcast_ref::<std::string::String>() {
                let _ = writeln!(
                    stderr,
                    "[ABI_BARRIER_PANIC] Caught panic in `{}`: {}",
                    fn_name, s
                );
            } else {
                let _ = writeln!(
                    stderr,
                    "[ABI_BARRIER_PANIC] Caught non-string panic in `{}`",
                    fn_name
                );
            };

            match strategy {
                PanicStrategy::Abort => {
                    std::process::abort();
                }
                PanicStrategy::ReturnDefault => Err(AbiError::Panicked {
                    function: fn_name,
                    reason: "rust panic caught at abi barrier",
                }),
                PanicStrategy::ReturnError(code) => Err(AbiError::ErrorCode(code)),
                PanicStrategy::Custom(handler) => {
                    let custom_code =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handler(fn_name)))
                            .unwrap_or(-1);
                    Err(AbiError::CustomHandled(custom_code))
                }
            }
        }
    }
}
