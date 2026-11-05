//! Zero-cost pointer validation and conversion primitives.

use crate::error::AbiError;
use core::ffi::CStr;

/// Validates that a raw pointer is non-null and properly aligned, converting it into a safe immutable reference.
///
/// # Safety
/// The caller must ensure that `ptr` points to initialized memory valid for `'a`.
#[inline(always)]
pub unsafe fn ensure_non_null<'a, T>(
    ptr: *const T,
    argument_name: &'static str,
) -> Result<&'a T, AbiError> {
    if ptr.is_null() {
        return Err(AbiError::NullPointer {
            argument: argument_name,
        });
    }

    let align = core::mem::align_of::<T>();
    if (ptr as usize) % align != 0 {
        return Err(AbiError::UnalignedPointer {
            argument: argument_name,
            alignment: align,
        });
    }

    // SAFETY: Pointer is verified non-null and aligned, memory validity guaranteed by caller.
    Ok(unsafe { &*ptr })
}

/// Validates that a raw pointer is non-null and properly aligned, converting it into a safe mutable reference.
///
/// # Safety
/// The caller must ensure that `ptr` points to initialized memory valid for unique mutable access for `'a`.
#[inline(always)]
pub unsafe fn ensure_non_null_mut<'a, T>(
    ptr: *mut T,
    argument_name: &'static str,
) -> Result<&'a mut T, AbiError> {
    if ptr.is_null() {
        return Err(AbiError::NullPointer {
            argument: argument_name,
        });
    }

    let align = core::mem::align_of::<T>();
    if (ptr as usize) % align != 0 {
        return Err(AbiError::UnalignedPointer {
            argument: argument_name,
            alignment: align,
        });
    }

    // SAFETY: Pointer is verified non-null and aligned, unique mutable access guaranteed by caller.
    Ok(unsafe { &mut *ptr })
}

/// Validates that a raw C-string pointer is non-null and valid UTF-8, returning a `&'a str`.
///
/// # Safety
/// The caller must ensure `c_ptr` points to a null-terminated C string valid for `'a`.
#[inline(always)]
pub unsafe fn ensure_valid_c_str<'a>(
    c_ptr: *const core::ffi::c_char,
    argument_name: &'static str,
) -> Result<&'a str, AbiError> {
    if c_ptr.is_null() {
        return Err(AbiError::NullPointer {
            argument: argument_name,
        });
    }

    // SAFETY: c_ptr is verified non-null, null-termination guaranteed by caller.
    let c_str = unsafe { CStr::from_ptr(c_ptr) };
    c_str.to_str().map_err(|_| AbiError::InvalidUtf8 {
        argument: argument_name,
    })
}
