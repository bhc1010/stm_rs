use crate::core::{stmimage::STMImage, jlcontext::JuliaContext};
use std::path::PathBuf;
use jlrs::prelude::*;

#[async_trait(?Send)]
impl AsyncTask for STMImage {
    type Output = Bool;

    // Include the custom code MyTask needs.
    async fn register<'frame>(mut frame: AsyncGcFrame<'frame>) -> JlrsResult<()> {
        unsafe {
            let path = PathBuf::from("../procedures/lockin_test.jl");
            if path.exists() {
                Value::include(frame.as_extended_target(), "../procedures/lockin_test.jl")?.into_jlrs_result()?;
            }
        }
        Ok(())
    }

     // This is the async variation of the closure you provide `Julia::scope` when using the sync
    // runtime.
    async fn run<'frame>(&mut self, mut frame: AsyncGcFrame<'frame>) -> JlrsResult<Self::Output> {
        // Convert the two arguments to values Julia can work with.
        // let dims = Value::new(&mut frame, self.dims);
        // let iters = Value::new(&mut frame, self.iters);

        // Get `read_lockin` in `Test`, call it on another thread with `call_async`, and await
        // the result before casting it to an `f64` (which that function returns). A function that
        // is called with `call_async` is executed on another thread by calling
        // `Base.threads.@spawn`.
        // The module and function don't have to be rooted because the module is never redefined,
        // so they're globally rooted.
        unsafe {
            Module::main(&frame)
                .submodule(&frame, "Test")?
                .wrapper()
                .function(&frame, "read_lockin")?
                .wrapper()
                .call_async(&mut frame, &mut [])
                .await
                .into_jlrs_result()?
                .unbox::<Bool>()
        }
    }
}