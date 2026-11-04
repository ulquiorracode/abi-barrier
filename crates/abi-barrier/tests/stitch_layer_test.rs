use abi_barrier::prelude::*;
use stitch_core::middleware::Terminal;
use stitch_core::pipeline::Pipeline;

#[repr(C, align(64))]
struct MockContext {
    is_client_connected: bool,
}

// MockContext has 64-byte alignment and POD representation
impl stitch_core::Blackboard for MockContext {}

struct EngineTerminal;

impl Terminal<MockContext, i32, i32, &'static str> for EngineTerminal {
    fn execute(&mut self, ctx: &mut MockContext, intent: i32) -> Result<i32, &'static str> {
        if !ctx.is_client_connected {
            Err("client not connected")
        } else {
            Ok(intent * 2)
        }
    }
}

#[test]
fn test_abi_barrier_stitch_layer() {
    let barrier_layer = AbiBarrierLayer::new(
        |ctx: &mut MockContext, _intent: &i32| -> Result<(), AbiError> {
            if !ctx.is_client_connected {
                Err(AbiError::NullPointer {
                    argument: "client_edict",
                })
            } else {
                Ok(())
            }
        },
        |_abi_err: AbiError, _ctx: &mut MockContext| -> &'static str {
            "halted by abi barrier: invalid client"
        },
        PanicStrategy::ReturnDefault,
    );

    let mut pipeline = Pipeline::on_terminal(EngineTerminal).wrap(barrier_layer);

    // 1. Rejected on enter
    let mut invalid_ctx = MockContext {
        is_client_connected: false,
    };
    let res1 = pipeline.dispatch(&mut invalid_ctx, 42);
    assert_eq!(res1, Err("halted by abi barrier: invalid client"));

    // 2. Allowed to proceed
    let mut valid_ctx = MockContext {
        is_client_connected: true,
    };
    let res2 = pipeline.dispatch(&mut valid_ctx, 42);
    assert_eq!(res2, Ok(84));
}
