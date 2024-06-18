use super::Caller;
use async_channel::{bounded, Receiver, Sender};
use std::time::Instant;

/// Deferred Caller with built-in reply pipe
#[derive(Debug)]
pub struct Summoner<T, R> {
    reply_tx: Sender<R>,
    reply_rx: Receiver<R>,
    inner: Caller<T>,
}

impl<T, R> Summoner<T, R> {
    pub fn new(inner: Caller<T>) -> Self {
        let (reply_tx, reply_rx) = bounded(1);
        Summoner {
            reply_tx,
            reply_rx,
            inner,
        }
    }
}

macro_rules! summoner_impl {
    ($($p:ident),*) => {
        impl<$($p, )* R> Caller<(Instant, $($p, )* Sender<R>)> {
            /// Turn this [`Caller`] into a [`Summoner`]
            pub fn summoner(self) -> Summoner<(Instant, $($p, )* Sender<R>), R> {
                Summoner::new(self)
            }
        }
    }
}

summoner_impl! {}
summoner_impl! {P1}
summoner_impl! {P1, P2}
summoner_impl! {P1, P2, P3}

macro_rules! summon_impl {
    ($($name:ident: $p:ident),*) => {
        impl<$($p, )* R> Summoner<(Instant, $($p, )* Sender<R>), R> {
            /// Schedule a call to the callback and wait for a reply
            pub async fn summon(&self, $($name: $p, )*) -> R {
                let tx = self.reply_tx.clone();
                self.inner.call($($name, )* tx).await.unwrap();
                self.reply_rx.recv().await.unwrap()
            }
        }
    }
}

summon_impl! {}
summon_impl! {a: P1}
summon_impl! {a: P1, b: P2}
summon_impl! {a: P1, b: P2, c: P3}
