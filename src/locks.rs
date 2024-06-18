use async_lock::{Mutex, RwLock};
use std::{
    future::Future,
    ops::{Deref, DerefMut},
    sync::Arc,
};

// --- MUT ---

/// Locking primitives, Mutable Access
pub trait LockMut {
    type Inner;
    type Output<'a>: DerefMut<Target = Self::Inner>
    where
        Self: 'a;
    fn lock_mut<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        Self: 'a;
}

impl<T> LockMut for RwLock<T> {
    type Inner = T;
    type Output<'a> = async_lock::RwLockWriteGuard<'a, T> where T: 'a;
    fn lock_mut<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        T: 'a,
    {
        async { self.write().await }
    }
}

impl<T> LockMut for Mutex<T> {
    type Inner = T;
    type Output<'a> = async_lock::MutexGuard<'a, T> where T: 'a;
    fn lock_mut<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        T: 'a,
    {
        async { self.lock().await }
    }
}

impl<T, I: LockMut<Inner = T>> LockMut for Arc<I> {
    type Inner = T;
    type Output<'a> = I::Output<'a> where I: 'a;
    fn lock_mut<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        I: 'a,
    {
        async { self.deref().lock_mut().await }
    }
}

// --- REF ---

/// Locking primitives, Immutable Access
pub trait LockRef {
    type Inner;
    type Output<'a>: Deref<Target = Self::Inner>
    where
        Self: 'a;
    fn lock_ref<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        Self: 'a;
}

impl<T> LockRef for RwLock<T> {
    type Inner = T;
    type Output<'a> = async_lock::RwLockReadGuard<'a, T> where T: 'a;
    fn lock_ref<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        T: 'a,
    {
        async { self.read().await }
    }
}

impl<T> LockRef for Mutex<T> {
    type Inner = T;
    type Output<'a> = async_lock::MutexGuard<'a, T> where T: 'a;
    fn lock_ref<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        T: 'a,
    {
        async { self.lock().await }
    }
}

impl<T, I: LockRef<Inner = T>> LockRef for Arc<I> {
    type Inner = T;
    type Output<'a> = I::Output<'a> where I: 'a;
    fn lock_ref<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        I: 'a,
    {
        async { self.deref().lock_ref().await }
    }
}

impl<T> LockRef for &T {
    type Inner = T;
    type Output<'a> = &'a T where T: 'a, Self: 'a;
    fn lock_ref<'a>(&'a self) -> impl Future<Output = Self::Output<'a>>
    where
        T: 'a,
    {
        async { *self }
    }
}
