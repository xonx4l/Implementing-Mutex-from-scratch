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

unsafe impl<T: Send> Send for Mutex<T> {]
unsafe impl<T: Send> Sync for Mutex<T> {}


