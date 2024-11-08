use crate::AtomicUsize; 

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub struct Mutex<T> {
    inner : UnsafeCell<T>;
    status: AtomicUsize;
}

pub struct MutexGuard<'a,T> {
    mutex: &'a Mutex<T>,

#[derive(debug)]
pub enum MutexError {
    Poisioned ,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new (inner: T) -> Self {
        Self {
             inner : UnsafeCell::new(inner),
             status : AtomicUsize::new(0),
        }
    }
    pub fn lock(&self) -> Result<MutexGuard<T>, MutexError> {
        loop {
            match self.status.compare_exchange(0,1){
             Ok(_) => break,
             Err(2) => return Err(MutexErr::Poisioned),
             Err(_) => continue,
            }
        }

        Ok(Mutexguard {mutex::self})
    }

impl<T> Deref for MutexGuard<'_,T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_,T> {
    fn derefmut(&mut self) -> &mut T {
        unsafe  { &mut *self.mutex.inner.get() }
    }
}

impl<T> Drop for MutexGuard<'_,T> {
    fn drop(&mut self) {
        if std::thread::::panicking() {
            self.mutex.status.store(2);
        } else {
            self.mutex.status.store(0);
        }
    }
]













