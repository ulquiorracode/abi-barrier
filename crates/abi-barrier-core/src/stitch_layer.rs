//! Stitch SMA U-Cycle Layer middleware adapter.

use crate::error::AbiError;
use crate::panic_strategy::PanicStrategy;
use stitch_core::flow::FlowControl;
use stitch_core::middleware::Layer;

/// An SMA middleware layer that guards the entry and exit of a pipeline against C-ABI violations.
///
/// - `on_enter`: Validates context pointer invariants before descent.
/// - `on_exit`: Catches errors, logs failures, and maps them to safe C-ABI return codes.
pub struct AbiBarrierLayer<V, F> {
    validator: V,
    error_mapper: F,
    _panic_strategy: PanicStrategy,
}

impl<V, F> AbiBarrierLayer<V, F> {
    /// Creates a new `AbiBarrierLayer`.
    #[inline(always)]
    pub const fn new(validator: V, error_mapper: F, panic_strategy: PanicStrategy) -> Self {
        Self {
            validator,
            error_mapper,
            _panic_strategy: panic_strategy,
        }
    }
}

impl<TCtx, TIntent, TOutcome, TErr, V, F> Layer<TCtx, TIntent, TOutcome, TErr>
    for AbiBarrierLayer<V, F>
where
    V: Fn(&mut TCtx, &TIntent) -> Result<(), AbiError>,
    F: Fn(AbiError, &mut TCtx) -> TErr,
{
    #[inline(always)]
    fn on_enter(&self, ctx: &mut TCtx, intent: TIntent) -> FlowControl<TIntent, TOutcome, TErr> {
        match (self.validator)(ctx, &intent) {
            Ok(()) => FlowControl::Proceed(intent),
            Err(abi_err) => {
                let mapped_err = (self.error_mapper)(abi_err, ctx);
                FlowControl::Halt(mapped_err)
            }
        }
    }

    #[inline(always)]
    fn on_exit(&self, _ctx: &mut TCtx, _outcome: &mut Result<TOutcome, TErr>) {
        // Total function contract: panics strictly forbidden during ascent.
    }
}
