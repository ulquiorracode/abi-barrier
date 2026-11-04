//! Safe FFI return value conversions on panic or failure.

use crate::error::AbiError;
use crate::panic_strategy::PanicStrategy;

/// Trait for converting ABI errors or panic reactions into safe C-ABI return values.
pub trait AbiReturn: Sized {
    /// Produces a safe return value when a panic occurs, according to the specified strategy.
    fn from_panic_error(strategy: PanicStrategy, err: AbiError) -> Self;
}

impl AbiReturn for () {
    #[inline(always)]
    fn from_panic_error(_strategy: PanicStrategy, _err: AbiError) -> Self {}
}

impl AbiReturn for bool {
    #[inline(always)]
    fn from_panic_error(_strategy: PanicStrategy, err: AbiError) -> Self {
        match err {
            AbiError::ErrorCode(code) => code != 0,
            AbiError::CustomHandled(code) => code != 0,
            _ => false,
        }
    }
}

macro_rules! impl_abi_return_int {
    ($($t:ty),*) => {
        $(
            impl AbiReturn for $t {
                #[inline(always)]
                fn from_panic_error(_strategy: PanicStrategy, err: AbiError) -> Self {
                    match err {
                        AbiError::ErrorCode(code) => code as $t,
                        AbiError::CustomHandled(code) => code as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}

impl_abi_return_int!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

impl AbiReturn for f32 {
    #[inline(always)]
    fn from_panic_error(_strategy: PanicStrategy, err: AbiError) -> Self {
        match err {
            AbiError::ErrorCode(code) => code as f32,
            AbiError::CustomHandled(code) => code as f32,
            _ => 0.0,
        }
    }
}

impl AbiReturn for f64 {
    #[inline(always)]
    fn from_panic_error(_strategy: PanicStrategy, err: AbiError) -> Self {
        match err {
            AbiError::ErrorCode(code) => code as f64,
            AbiError::CustomHandled(code) => code as f64,
            _ => 0.0,
        }
    }
}

impl<T> AbiReturn for *const T {
    #[inline(always)]
    fn from_panic_error(_strategy: PanicStrategy, _err: AbiError) -> Self {
        core::ptr::null()
    }
}

impl<T> AbiReturn for *mut T {
    #[inline(always)]
    fn from_panic_error(_strategy: PanicStrategy, _err: AbiError) -> Self {
        core::ptr::null_mut()
    }
}

impl<T> AbiReturn for Option<T> {
    #[inline(always)]
    fn from_panic_error(_strategy: PanicStrategy, _err: AbiError) -> Self {
        None
    }
}
