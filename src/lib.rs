#![doc = include_str!("../README.md")]

use async_io::Timer;
use std::{fmt::Display, time::Duration};

mod async_fn;
mod caller;
mod defer;
mod defer_mut;
mod dispatcher;
mod locks;
mod summoner;

pub use caller::Caller;
pub use dispatcher::Dispatcher;
pub use locks::{LockMut, LockRef};
pub use summoner::Summoner;
pub use {async_channel, async_lock};

async fn sleep(duration: Duration) {
    Timer::after(duration).await;
}

/// Supported method return types
pub trait ReturnType: Sized {
    /// Called on the return value when the method returns
    fn log(self) {}
}

impl ReturnType for () {}
impl ReturnType for Option<()> {}

impl<E: Display> ReturnType for Result<(), E> {
    fn log(self) {
        if let Err(message) = self {
            log::error!("{}", message);
        }
    }
}
