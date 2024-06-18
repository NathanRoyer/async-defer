use async_channel::{Send, Sender};
use std::time::Instant;

/// Deferred Caller
#[derive(Clone, Default, Debug)]
pub struct Caller<T> {
    pub(crate) inner: Option<Sender<T>>,
}

macro_rules! caller_impl {
    ($($name:ident: $p:ident),*) => {
        impl<$($p),*> Caller<(Instant, $($p),*)> {
            /// Schedule a call to the callback
            pub fn call(&self, $($name: $p),*) -> Send<(Instant, $($p),*)> {
                self.call_later(Instant::now(), $($name),*)
            }

            /// Schedule a delayed call to the callback
            pub fn call_later(&self, when: Instant, $($name: $p),*) -> Send<(Instant, $($p),*)> {
                match &self.inner {
                    Some(inner) => inner.send((when, $($name),*)),
                    None => panic!("Uninitialized Caller!"),
                }
            }
        }
    }
}

caller_impl! {}
caller_impl! {a: P1}
caller_impl! {a: P1, b: P2}
caller_impl! {a: P1, b: P2, c: P3}
caller_impl! {a: P1, b: P2, c: P3, d: P4}
