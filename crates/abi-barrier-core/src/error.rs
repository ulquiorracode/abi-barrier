//! Error definitions for C-ABI boundaries.

/// Domain errors encountered at the C-ABI boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbiError {
    /// A required pointer argument was null.
    NullPointer { argument: &'static str },
    /// A raw C-string was not valid UTF-8.
    InvalidUtf8 { argument: &'static str },
    /// A panic was caught at the FFI boundary.
    Panicked {
        function: &'static str,
        reason: &'static str,
    },
    /// Explicit error code returned by a panic strategy.
    ErrorCode(i32),
    /// Custom error code returned by a user-defined handler.
    CustomHandled(i32),
}

#[cfg(feature = "std")]
impl std::fmt::Display for AbiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullPointer { argument } => {
                write!(f, "null pointer passed for argument `{argument}`")
            }
            Self::InvalidUtf8 { argument } => {
                write!(f, "invalid UTF-8 in C-string argument `{argument}`")
            }
            Self::Panicked { function, reason } => {
                write!(f, "panic in function `{function}`: {reason}")
            }
            Self::ErrorCode(code) => write!(f, "FFI error code {code}"),
            Self::CustomHandled(code) => write!(f, "custom FFI handler error code {code}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AbiError {}
