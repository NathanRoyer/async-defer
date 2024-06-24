use std::future::Future;

macro_rules! async_fn_impl {
    ($name:ident, $($p:ident),*) => {
        pub trait $name<$($p),*>: Fn($($p),*) -> Self::OutputFuture + Send + 'static {
            type OutputFuture: Future<Output = <Self as $name<$($p),*>>::Output> + Send;
            type Output;
        }

        impl<F, Fut, $($p),*> $name<$($p),*> for F where
            F: ?Sized + Fn($($p),*) -> Fut + Send + 'static,
            Fut: Future + Send,
        {
            type OutputFuture = Fut;
            type Output = Fut::Output;
        }
    }
}

async_fn_impl! {AsyncFn1, P1}
async_fn_impl! {AsyncFn2, P1, P2}
async_fn_impl! {AsyncFn3, P1, P2, P3}
async_fn_impl! {AsyncFn4, P1, P2, P3, P4}
async_fn_impl! {AsyncFn5, P1, P2, P3, P4, P5}
async_fn_impl! {AsyncFn6, P1, P2, P3, P4, P5, P6}
async_fn_impl! {AsyncFn7, P1, P2, P3, P4, P5, P6, P7}
async_fn_impl! {AsyncFn8, P1, P2, P3, P4, P5, P6, P7, P8}
