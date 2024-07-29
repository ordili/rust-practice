use std::alloc::Layout;
use std::ptr::NonNull;
use std::{alloc, ptr};

const DEAFULT_SIZE: usize = 4;
pub struct Vec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> Vec<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        let layout = Layout::array::<T>(DEAFULT_SIZE).unwrap();
        let ptr = unsafe { alloc::alloc(layout) as *mut T };
        let ptr = NonNull::new(ptr).unwrap();
        Self {
            ptr: ptr,
            len: 0,
            cap: DEAFULT_SIZE,
        }
    }

    // 在尾部新增元素
    pub fn push(&mut self, t: T) {
        self.grow();
        unsafe {
            let ptr = self.ptr.as_ptr();
            let ptr = ptr.add(self.len);
            ptr::write(ptr, t);
            self.len += 1;
        }

    }

    // 删除尾部的元素
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            self.len -= 1;
            let p = self.ptr.as_ptr();
            let ret = ptr::read(p.add(self.len));
            Some(ret)
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }

    // 内存扩张
    pub fn grow(&mut self){

        if self.len < self.cap {
            return;
        }

        let old_layout = Layout::array::<T>(self.cap).expect("Get old layout error");
        let old_ptr = self.ptr.as_ptr() as *mut u8;

        let new_cap = self.cap * 2;
        let new_layout = Layout::array::<T>(new_cap).unwrap();

        unsafe {
            let new_ptr = alloc::realloc(old_ptr,old_layout,new_layout.size());
            let new_ptr = new_ptr as *mut T;
            self.ptr = match NonNull::new(new_ptr) {
                Some(ptr) => ptr,
                None => {
                    alloc::handle_alloc_error(new_layout);
                }
            };
            self.cap = new_cap;
        }

    }
}

#[cfg(test)]
pub mod test {

    use super::Vec;
    #[test]
    fn test_new() {
        let _vec = Vec::<i32>::new();
    }

    #[test]
    fn test_push_pop() {
        let size = 200;
        let mut vec = Vec::new();
        assert_eq!(vec.len, 0);
        for x in 1..=size {
            vec.push(x);
            assert_eq!(vec.len, x);
        }
        assert_eq!(vec.len(), size);

        for x in (1..=size).rev() {
            assert_eq!(vec.pop(), Some(x));
            assert_eq!(vec.len(), x - 1);
        }
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.len(), 0);
    }
}
