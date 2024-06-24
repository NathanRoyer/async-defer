use async_lock::{Mutex, RwLock};
use std::{
    future::Future,
    ops::{Deref, DerefMut},
};

// --- MUT ---

/// Locking primitives, Mutable Access
pub trait LockMut: Send + Sync + 'static {
    type Inner;
    type Output<'a>: DerefMut<Target = Self::Inner> + Send;
    fn lock_mut<'a>(&'a self) -> impl Future<Output = Self::Output<'a>> + Send;
}

impl<T: Send + Sync + 'static> LockMut for RwLock<T> {
    type Inner = T;
    type Output<'a> = async_lock::RwLockWriteGuard<'a, T>;
    fn lock_mut<'a>(&'a self) -> impl Future<Output = Self::Output<'a>> + Send {
        async { self.write().await }
    }
}

impl<T: Send + Sync + 'static> LockMut for Mutex<T> {
    type Inner = T;
    type Output<'a> = async_lock::MutexGuard<'a, T>;
    fn lock_mut<'a>(&'a self) -> impl Future<Output = Self::Output<'a>> + Send {
        async { self.lock().await }
    }
}

// --- REF ---

/// Locking primitives, Immutable Access
pub trait LockRef: Send + Sync + 'static {
    type Inner;
    type Output<'a>: Deref<Target = Self::Inner> + Send;
    fn lock_ref<'a>(&'a self) -> impl Future<Output = Self::Output<'a>> + Send;
}

impl<T: Send + Sync + 'static> LockRef for RwLock<T> {
    type Inner = T;
    type Output<'a> = async_lock::RwLockReadGuard<'a, T>;
    fn lock_ref<'a>(&'a self) -> impl Future<Output = Self::Output<'a>> + Send {
        async { self.read().await }
    }
}

impl<T: Send + Sync + 'static> LockRef for Mutex<T> {
    type Inner = T;
    type Output<'a> = async_lock::MutexGuard<'a, T>;
    fn lock_ref<'a>(&'a self) -> impl Future<Output = Self::Output<'a>> + Send {
        async { self.lock().await }
    }
}
