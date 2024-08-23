use std::{ops::Deref, sync::Mutex};

fn main() {
    let r = Rc::new(456);

    let r2 = r.clone();

    drop(r);

    println!("{}", *r2);
}
struct Rc<T> {
    ptr: *mut RcInner<T>,
}
impl<T> Rc<T> {
    pub fn new(data: T) -> Self {
        let b = Box::new( RcInner { data, count: Mutex::new(1) } );

        let ptr = Box::into_raw(b);

        Rc { ptr }
    }
}
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let mut l = (*self.ptr).count.lock().unwrap();
            *l += 1;
        } 
        Self { ptr: self.ptr.clone() }
    }
}
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            let mut l = (*self.ptr).count.lock().unwrap();
            *l -= 1;

            if *l == 0 {
                let _ = Box::from_raw(self.ptr);
            }
        } 
    }
}
impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            &(*self.ptr).data
        }
    }
}

unsafe impl<T: Send> Send for Rc<T> {}
unsafe impl<T: Sync> Sync for Rc<T> {}

struct RcInner<T> {
    data: T,
    count: Mutex<usize>,
}