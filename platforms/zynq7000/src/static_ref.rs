use core::ops::Deref;

#[derive(Debug)]
pub struct StaticRef<T> {
    ptr: *const T,
}

impl<T> StaticRef<T> {
    pub const unsafe fn new(ptr: *const T) -> StaticRef<T> {
        StaticRef { ptr }
    }
}

unsafe impl<T> Send for StaticRef<T> {}

impl<T> Deref for StaticRef<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
