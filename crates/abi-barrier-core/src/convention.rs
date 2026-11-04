//! Closed-World calling conventions for hardware C-ABI interoperability.

/// Closed-world hardware/compiler calling conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallingConvention {
    /// Standard C calling convention (`extern "C"`).
    C,
    /// x86 32-bit cdecl (`extern "cdecl"`).
    Cdecl,
    /// Windows x86 32-bit stdcall (`extern "stdcall"`).
    Stdcall,
    /// Fastcall register-based convention (`extern "fastcall"`).
    Fastcall,
    /// Operating system platform default convention (`extern "system"`).
    System,
}
