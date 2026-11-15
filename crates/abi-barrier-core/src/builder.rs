//! # Tier 2: Fluent Builder for C-ABI Barriers
//!
//! Provides [`BarrierBuilder`] for executing or wrapping functions with configurable
//! calling conventions, panic strategies, and error mapping with zero heap allocation.

use crate::convention::CallingConvention;
use crate::error::AbiError;
use crate::panic_strategy::PanicStrategy;
use crate::return_value::AbiReturn;

/// Fluent builder for constructing and invoking C-ABI barriers.
#[derive(Debug, Clone, Copy)]
pub struct BarrierBuilder {
    fn_name: &'static str,
    convention: CallingConvention,
    panic_strategy: PanicStrategy,
}

impl BarrierBuilder {
    /// Creates a new barrier builder for the named function.
    #[inline(always)]
    pub const fn new(fn_name: &'static str) -> Self {
        Self {
            fn_name,
            convention: CallingConvention::C,
            panic_strategy: PanicStrategy::ReturnDefault,
        }
    }

    /// Sets the expected calling convention.
    #[inline(always)]
    pub const fn convention(mut self, convention: CallingConvention) -> Self {
        self.convention = convention;
        self
    }

    /// Sets the panic handling strategy.
    #[inline(always)]
    pub const fn panic_strategy(mut self, strategy: PanicStrategy) -> Self {
        self.panic_strategy = strategy;
        self
    }

    /// Catches panics within the enclosed closure using the configured strategy.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn catch_panic<R, F>(self, f: F) -> Result<R, AbiError>
    where
        F: FnOnce() -> R,
    {
        crate::catch_panic_barrier(self.fn_name, self.panic_strategy, f)
    }

    /// Executes the closure under the barrier, converting panic errors to safe C-ABI return values.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn execute<R: AbiReturn, F>(self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        match self.catch_panic(f) {
            Ok(ret) => ret,
            Err(err) => R::from_panic_error(self.panic_strategy, err),
        }
    }
}

/// Fluent builder for constructing [`crate::stitch_layer::AbiBarrierLayer`].
#[cfg(feature = "stitch")]
pub struct AbiBarrierLayerBuilder<V, F> {
    validator: V,
    error_mapper: F,
    panic_strategy: PanicStrategy,
}

#[cfg(feature = "stitch")]
impl Default for AbiBarrierLayerBuilder<(), ()> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "stitch")]
impl AbiBarrierLayerBuilder<(), ()> {
    /// Creates a new empty `AbiBarrierLayerBuilder`.
    pub const fn new() -> Self {
        Self {
            validator: (),
            error_mapper: (),
            panic_strategy: PanicStrategy::ReturnDefault,
        }
    }
}

#[cfg(feature = "stitch")]
impl<V, F> AbiBarrierLayerBuilder<V, F> {
    /// Sets the panic strategy.
    pub const fn panic_strategy(mut self, strategy: PanicStrategy) -> Self {
        self.panic_strategy = strategy;
        self
    }

    /// Sets the pre-descent validator closure.
    pub fn validator<V2>(self, validator: V2) -> AbiBarrierLayerBuilder<V2, F> {
        AbiBarrierLayerBuilder {
            validator,
            error_mapper: self.error_mapper,
            panic_strategy: self.panic_strategy,
        }
    }

    /// Sets the error mapping closure.
    pub fn error_mapper<F2>(self, error_mapper: F2) -> AbiBarrierLayerBuilder<V, F2> {
        AbiBarrierLayerBuilder {
            validator: self.validator,
            error_mapper,
            panic_strategy: self.panic_strategy,
        }
    }
}

#[cfg(feature = "stitch")]
impl<V, F> AbiBarrierLayerBuilder<V, F> {
    /// Builds the configured [`crate::stitch_layer::AbiBarrierLayer`].
    pub fn build(self) -> crate::stitch_layer::AbiBarrierLayer<V, F> {
        crate::stitch_layer::AbiBarrierLayer::new(
            self.validator,
            self.error_mapper,
            self.panic_strategy,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrier_builder_execution() {
        let val = BarrierBuilder::new("test_fn")
            .panic_strategy(PanicStrategy::ReturnDefault)
            .execute(|| 42i32);
        assert_eq!(val, 42);

        let panicked_val = BarrierBuilder::new("test_panic")
            .panic_strategy(PanicStrategy::ReturnError(-99))
            .execute(|| -> i32 { panic!("simulated panic") });
        assert_eq!(panicked_val, -99);
    }
}
