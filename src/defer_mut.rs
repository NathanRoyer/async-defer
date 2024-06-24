#![allow(unused_variables)]

use super::async_fn::*;
use super::{sleep, Caller, Dispatcher, LockMut, ReturnType};
use futures_lite::future::or;
use std::time::{Duration, Instant};

macro_rules! listen_impl {
    ($dispatcher:ident, $callback:ident, $($p:ident),*) => {{
        let (tx, rx) = async_channel::unbounded();
        let lock = $dispatcher.get_lock();

        let task = async move {
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
                        let mut locked = lock.lock_mut().await;
                        let ret = $callback(&mut *locked, $($p),*).await;
                        ret.log();
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

        let boxed = Box::pin(task);
        $dispatcher.spawn(boxed);

        Caller {
            inner: Some(tx),
        }
    }}
}

macro_rules! defer_impl {
    ($method:ident, $async_fn:ident, $($name:ident: $p:ident),*) => {
        impl<T: Send, L: LockMut<Inner = T>> Dispatcher<L> {
            pub fn $method<F, R, $($p: Send + 'static, )*>(&mut self, callback: F) -> Caller<(Instant, $($p, )*)>
            where
                for<'a> F: $async_fn<&'a mut T, $($p, )* Output = R>,
                R: ReturnType
            {
                listen_impl!(self, callback, $($name),*)
            }
        }
    }
}

defer_impl! {listen_mut_0, AsyncFn1, }
defer_impl! {listen_mut_1, AsyncFn2, a: P1}
defer_impl! {listen_mut_2, AsyncFn3, a: P1, b: P2}
defer_impl! {listen_mut_3, AsyncFn4, a: P1, b: P2, c: P3}
defer_impl! {listen_mut_4, AsyncFn5, a: P1, b: P2, c: P3, d: P4}
defer_impl! {listen_mut_5, AsyncFn6, a: P1, b: P2, c: P3, d: P4, e: P5}
defer_impl! {listen_mut_6, AsyncFn7, a: P1, b: P2, c: P3, d: P4, e: P5, f: P6}
defer_impl! {listen_mut_7, AsyncFn8, a: P1, b: P2, c: P3, d: P4, e: P5, f: P6, g: P7}
