use std::sync::{Arc, Mutex, MutexGuard};

pub type Synced<T> = Arc<Mutex<T>>;

pub fn wrap<T: Sized>(unwrapped: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(unwrapped))
}

pub fn unlock<'a, T: ?Sized>(wrapped: &'a Arc<Mutex<T>>) -> MutexGuard<'a, T> {
    wrapped.lock().unwrap()
}

//pub fn unlock_mut<'a, T: ?Sized>(wrapped: &'a Arc<Mutex<T>>) -> &mut T {
//    wrapped.get_mut().unwrap()
//}
