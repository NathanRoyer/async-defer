#![doc = include_str!("../README.md")]

use async_io::Timer;
use std::time::Duration;

mod caller;
mod defer;
mod defer_mut;
mod locks;
mod summoner;

pub use caller::Caller;
pub use defer::{Defer0, Defer1, Defer2, Defer3, Defer4};
pub use defer_mut::{DeferMut0, DeferMut1, DeferMut2, DeferMut3, DeferMut4};
pub use locks::{LockMut, LockRef};
pub use summoner::Summoner;
pub use {async_channel, async_lock};

type Ret = Result<(), String>;

async fn sleep(duration: Duration) {
    Timer::after(duration).await;
}
