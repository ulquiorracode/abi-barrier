//! Closed-World panic isolation strategies.

/// Closed-world reactions when a panic occurs at the C-ABI boundary.
#[derive(Debug, Clone, Copy)]
pub enum PanicStrategy {
    /// Immediately abort process (`std::process::abort()`).
    ///
    /// Essential when cross-language stack unwinding is strictly forbidden and
    /// partial state corruption is unacceptable.
    Abort,

    /// Return `Default::default()` or a safe dummy, transforming the panic into [`crate::AbiError::Panicked`].
    ReturnDefault,

    /// Return an explicit integer error code (`i32`) to the C caller.
    ReturnError(i32),

    /// Delegate panic handling to a custom function `fn(&'static str) -> i32`.
    Custom(fn(&'static str) -> i32),
}
