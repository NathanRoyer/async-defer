use futures_lite::{future::zip, FutureExt};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

#[cfg(doc)]
use super::Caller;

type Listener = Pin<Box<dyn Future<Output = ()> + Send>>;

/// Wrapper of `Arc<Lock<T>>`
///
/// This is used to create [`Caller`]s for some methods on `T`:
///
/// - use `Dispatcher::listen_ref_N` for methods taking `&self` and N additional parameters
/// - use `Dispatcher::listen_mut_N` for methods taking `&mut self` and N additional parameters
///
/// Deferred calls to these methods can then be scheduled via the returned [`Caller`]s.
///
/// [`Dispatcher`] implements `Future`; You must await this future for calls to
/// actually be dispatched.
///
pub struct Dispatcher<L> {
    lock: Arc<L>,
    task: Option<Listener>,
}

impl<L> Dispatcher<L> {
    pub fn new(lock: Arc<L>) -> Self {
        Self { lock, task: None }
    }

    pub(crate) fn get_lock(&self) -> Arc<L> {
        self.lock.clone()
    }

    pub(crate) fn spawn(&mut self, mut new_task: Listener) {
        if let Some(prev_task) = self.task.take() {
            new_task = Box::pin(async {
                zip(prev_task, new_task).await;
            });
        }

        self.task = Some(new_task);
    }
}

impl<L> Future for Dispatcher<L> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match &mut self.task {
            Some(task) => task.poll(cx),
            None => Poll::Ready(()),
        }
    }
}
