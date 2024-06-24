use async_channel::{Send, Sender};
use std::time::Instant;

#[cfg(doc)]
use super::Dispatcher;

/// Deferred Caller
///
/// When you create a [`Caller`] for a method of `T` using [`Dispatcher`], a background
/// task is created, waiting for you to schedule deferred calls via the [`Caller`]. When
/// a call is scheduled, the background task will lock the `RwLock` or `Mutex` and call
/// the method on the locked `T` instance.
///
/// ### Compatible `T` methods
///
/// The methods must:
/// - be asynchronous
/// - return `Result<(), String>`
///
/// They can take `self` mutably or immutably.
///
/// *Note: you can use a freestanding function instead of a method if the first parameter
/// of that function is `&T` or `&mut T`.*
///
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
caller_impl! {a: P1, b: P2, c: P3, d: P4, e: P5}
caller_impl! {a: P1, b: P2, c: P3, d: P4, e: P5, f: P6}
caller_impl! {a: P1, b: P2, c: P3, d: P4, e: P5, f: P6, g: P7}
