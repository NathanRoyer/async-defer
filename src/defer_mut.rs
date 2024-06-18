#![allow(unused_variables)]

use super::{sleep, Caller, LockMut, Ret};
use async_fn_traits::*;
use futures_lite::future::or;
use std::time::{Duration, Instant};

macro_rules! listen_impl {
    ($this:ident, $callback:ident, $($p:ident),*) => {{
        let (tx, rx) = async_channel::unbounded();

        let _task = async move {
            let mut pending = Vec::new();
            loop {
                // find earliest scheduled task
                let next_exec_instant = pending
                    .iter()
                    .map(|(instant, $($p),*)| instant)
                    .enumerate()
                    .min_by(|(_, a), (_, b)| Instant::cmp(a, b));

                let mut sleep_time = Duration::MAX;

                if let Some((i, instant)) = next_exec_instant {
                    let now = Instant::now();

                    // is it time to execute the task?
                    if *instant < now {
                        // execute the task

                        let (_, $($p),*) = pending.remove(i);
                        let mut locked = $this.lock_mut().await;
                        let future = $callback(&mut *locked, $($p),*);

                        if let Err(message) = future.await {
                            log::error!("{}", message);
                        }

                        continue;

                    } else {
                        // sleep until the next scheduled execution
                        sleep_time = *instant - now;
                    }
                }

                let pause = async {
                    sleep(sleep_time).await;
                    None
                };

                let recv = async {
                    Some(rx.recv().await)
                };

                match or(pause, recv).await {
                    Some(Ok(task)) => pending.push(task),
                    Some(_) => break,
                    None => (/* end of pause */),
                }
            }

            log::error!("Exiting deferred task");
        };

        Caller {
            inner: Some(tx),
        }
    }}
}

macro_rules! defer_impl {
    ($trait:ident, $method:ident, $async_fn:ident, $($name:ident: $p:ident),*) => {
        /// Deferred Calls, Mutable Access
        pub trait $trait<T, $($p, )*>: Sized {
            fn listen<F>(self, callback: F) -> Caller<(Instant, $($p, )*)>
            where
                for<'a> F: $async_fn<&'a mut T, $($p, )* Output = Ret>;

            fn $method<F>(self, callback: F) -> Caller<(Instant, $($p, )*)>
            where
                for<'a> F: $async_fn<&'a mut T, $($p, )* Output = Ret>,
            {
                self.listen(callback)
            }
        }

        impl<T, L: LockMut<Inner = T>, $($p, )*> $trait<T, $($p, )*> for L {
            fn listen<F>(self, callback: F) -> Caller<(Instant, $($p, )*)>
            where
                for<'a> F: $async_fn<&'a mut T, $($p, )* Output = Ret>,
            {
                listen_impl!(self, callback, $($name),*)
            }
        }
    }
}

defer_impl! {DeferMut0, listen_mut_0, AsyncFn1, }
defer_impl! {DeferMut1, listen_mut_1, AsyncFn2, a: P1}
defer_impl! {DeferMut2, listen_mut_2, AsyncFn3, a: P1, b: P2}
defer_impl! {DeferMut3, listen_mut_3, AsyncFn4, a: P1, b: P2, c: P3}
defer_impl! {DeferMut4, listen_mut_4, AsyncFn5, a: P1, b: P2, c: P3, d: P4}