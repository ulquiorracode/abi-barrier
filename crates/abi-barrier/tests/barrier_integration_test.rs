use abi_barrier::prelude::*;

#[barrier(convention = CallingConvention::C, on_panic = PanicStrategy::ReturnDefault)]
fn safe_divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero in C-ABI handler!");
    }
    a / b
}

#[barrier(convention = CallingConvention::C, on_panic = PanicStrategy::ReturnDefault)]
fn safe_strlen(c_str: *const core::ffi::c_char) -> usize {
    let s = unsafe { ensure_valid_c_str(c_str, "c_str").unwrap() };
    s.len()
}

#[barrier(convention = CallingConvention::C, on_panic = PanicStrategy::ReturnError(-101))]
fn safe_parse_int(should_panic: bool) -> i32 {
    if should_panic {
        panic!("simulated parsing failure!");
    }
    42
}

fn custom_panic_handler(_fn_name: &'static str) -> i32 {
    -999
}

#[barrier(convention = CallingConvention::C, on_panic = PanicStrategy::Custom(custom_panic_handler))]
fn safe_custom_fn(should_panic: bool) -> i32 {
    if should_panic {
        panic!("custom error!");
    }
    777
}

#[barrier(convention = CallingConvention::C, on_panic = PanicStrategy::ReturnDefault)]
fn safe_get_pointer(should_panic: bool) -> *mut u8 {
    if should_panic {
        panic!("cannot create pointer");
    }
    0x1234 as *mut u8
}

#[test]
fn test_barrier_normal_execution() {
    assert_eq!(safe_divide(10, 2), 5);
}

#[test]
fn test_barrier_catches_panic_and_returns_default() {
    // Calling safe_divide with b=0 triggers panic, which MUST be caught and return 0 (Default)
    assert_eq!(safe_divide(10, 0), 0);
}

#[test]
fn test_barrier_cstr_validation() {
    let valid_c = b"Hello, World!\0";
    let len = safe_strlen(valid_c.as_ptr() as *const _);
    assert_eq!(len, 13);

    // Null pointer should panic in inner, caught by barrier and return 0
    let null_len = safe_strlen(core::ptr::null());
    assert_eq!(null_len, 0);
}

#[test]
fn test_barrier_catches_panic_and_returns_error_code() {
    assert_eq!(safe_parse_int(false), 42);
    assert_eq!(safe_parse_int(true), -101);
}

#[test]
fn test_barrier_catches_panic_and_delegates_to_custom() {
    assert_eq!(safe_custom_fn(false), 777);
    assert_eq!(safe_custom_fn(true), -999);
}

#[test]
fn test_barrier_returns_null_pointer_on_panic() {
    assert_eq!(safe_get_pointer(false), 0x1234 as *mut u8);
    assert_eq!(safe_get_pointer(true), core::ptr::null_mut());
}
