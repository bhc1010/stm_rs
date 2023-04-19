use crossbeam_channel::Receiver;
use jlrs::prelude::*;
use jlrs::error::JlrsError;
use std::num::NonZeroUsize;
use std::thread::JoinHandle;

pub struct JuliaContext
{
    pub julia: AsyncJulia<Tokio>,
    pub handle: JoinHandle<Result<(), Box<JlrsError>>>,
    pub receiver: Option<Receiver<Result<jlrs::prelude::Bool, Box<JlrsError>>>>
}

impl Default for JuliaContext {
    fn default() -> Self {
        let (julia, handle) = unsafe {
            RuntimeBuilder::new()
                .async_runtime::<Tokio>()
                .channel_capacity(NonZeroUsize::new(2).unwrap())
                .start::<1>()
                .expect("Could not init Julia")
        };

        let receiver: Option<Receiver<Result<jlrs::prelude::Bool, Box<JlrsError>>>> = None;

        Self {
            julia, 
            handle,
            receiver
        }
    }
}

impl JuliaContext {
    pub fn load<Task>(&self) 
    where 
        Task : AsyncTask
    {
        // Include the custom code MyTask needs by registering it.
        let (sender, receiver) = crossbeam_channel::bounded(1);
        self.julia.try_register_task::<Task, _>(sender).unwrap();
        receiver.recv().unwrap().unwrap();
    }
}