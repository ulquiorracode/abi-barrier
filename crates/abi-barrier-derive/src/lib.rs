//! Procedural macros for `abi-barrier`.

use proc_macro::TokenStream;

mod barrier;

/// Wraps an `extern` function with an unbreakable C-ABI safety barrier.
///
/// Ensures:
/// - Unhandled Rust panics are caught with zero stack-unwind leaks across the ABI.
/// - Closed-world typed configuration via `convention = ...` and `on_panic = ...`.
/// - Automatic diagnostic logging of failures (Anti-Silent Error Suppression).
#[proc_macro_attribute]
pub fn barrier(attr: TokenStream, item: TokenStream) -> TokenStream {
    barrier::expand_barrier(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
