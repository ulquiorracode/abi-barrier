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
    fn from_panic_error(_strategy: PanicStrategy, _err: AbiError) -> Self {
        // In C-ABI, panics or barrier errors must strictly never indicate success (true).
        false
    }
}

macro_rules! impl_abi_return_signed_int {
    ($($t:ty),*) => {
        $(
            impl AbiReturn for $t {
                #[inline(always)]
                fn from_panic_error(strategy: PanicStrategy, err: AbiError) -> Self {
                    match err {
                        AbiError::ErrorCode(code) => {
                            let truncated = code as $t;
                            if truncated == 0 && code != 0 {
                                -1
                            } else {
                                truncated
                            }
                        }
                        AbiError::CustomHandled(code) => {
                            let truncated = code as $t;
                            if truncated == 0 && code != 0 {
                                -1
                            } else {
                                truncated
                            }
                        }
                        _ => match strategy {
                            PanicStrategy::ReturnDefault => 0,
                            PanicStrategy::ReturnError(code) => {
                                let truncated = code as $t;
                                if truncated == 0 && code != 0 {
                                    -1
                                } else {
                                    truncated
                                }
                            }
                            PanicStrategy::Custom(f) => {
                                let fn_name = match err {
                                    AbiError::Panicked { function, .. } => function,
                                    _ => "unknown",
                                };
                                f(fn_name) as $t
                            }
                            PanicStrategy::Abort => std::process::abort(),
                        },
                    }
                }
            }
        )*
    };
}

macro_rules! impl_abi_return_unsigned_int {
    ($($t:ty),*) => {
        $(
            impl AbiReturn for $t {
                #[inline(always)]
                fn from_panic_error(strategy: PanicStrategy, err: AbiError) -> Self {
                    match err {
                        AbiError::ErrorCode(code) => {
                            let truncated = code as $t;
                            if truncated == 0 && code != 0 {
                                !0
                            } else {
                                truncated
                            }
                        }
                        AbiError::CustomHandled(code) => {
                            let truncated = code as $t;
                            if truncated == 0 && code != 0 {
                                !0
                            } else {
                                truncated
                            }
                        }
                        _ => match strategy {
                            PanicStrategy::ReturnDefault => 0,
                            PanicStrategy::ReturnError(code) => {
                                let truncated = code as $t;
                                if truncated == 0 && code != 0 {
                                    !0
                                } else {
                                    truncated
                                }
                            }
                            PanicStrategy::Custom(f) => {
                                let fn_name = match err {
                                    AbiError::Panicked { function, .. } => function,
                                    _ => "unknown",
                                };
                                f(fn_name) as $t
                            }
                            PanicStrategy::Abort => std::process::abort(),
                        },
                    }
                }
            }
        )*
    };
}

impl_abi_return_signed_int!(i8, i16, i32, i64, isize);
impl_abi_return_unsigned_int!(u8, u16, u32, u64, usize);

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
