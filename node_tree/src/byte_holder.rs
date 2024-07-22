use std::{
    alloc::Layout,
    sync::atomic::{AtomicPtr, Ordering},
};

pub struct ByteHolder {
    pub bytes: AtomicPtr<u8>,
    pub layout: Layout,
    pub need_drop: bool,
}

impl ByteHolder {
    pub fn from_ref<T>(value: &T) -> Self {
        let bytes = unsafe {
            std::slice::from_raw_parts(value as *const T as *const u8, std::mem::size_of::<T>())
        };

        Self::from_slice(bytes, Layout::new::<T>())
    }

    pub fn from_slice(bytes: &[u8], layout: Layout) -> Self {
        let ptr = unsafe { std::alloc::alloc(layout) };
        for (idx, byte) in bytes.iter().enumerate() {
            unsafe {
                std::ptr::write(ptr.offset(idx as isize), *byte);
            }
        }

        Self {
            bytes: AtomicPtr::<u8>::new(ptr),
            layout,
            need_drop: true,
        }
    }

    pub unsafe fn downcast_ref<T>(&self) -> &T {
        let ptr = self.bytes.load(Ordering::SeqCst) as *const T;
        ptr.as_ref().unwrap()
    }

    pub unsafe fn downcast_mut<T>(&mut self) -> &mut T {
        let ptr = self.bytes.load(Ordering::SeqCst) as *mut T;
        ptr.as_mut().unwrap()
    }

    pub unsafe fn downcast_box<T>(mut self) -> Box<T> {
        let ptr = self.bytes.load(Ordering::SeqCst) as *mut T;
        self.need_drop = false;
        Box::from_raw(ptr)
    }
}

impl Drop for ByteHolder {
    fn drop(&mut self) {
        if self.need_drop {
            unsafe {
                std::alloc::dealloc(self.bytes.load(Ordering::SeqCst), self.layout);
            }
        }
    }
}

impl Clone for ByteHolder {
    fn clone(&self) -> Self {
        let ptr = self.bytes.load(Ordering::SeqCst);
        let cloned_ptr = unsafe {
            let cloned_ptr = std::alloc::alloc(self.layout);
            std::ptr::copy_nonoverlapping(ptr, cloned_ptr, self.layout.size());
            cloned_ptr
        };

        Self {
            bytes: AtomicPtr::new(cloned_ptr),
            layout: self.layout,
            need_drop: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Layout;
    use std::sync::atomic::AtomicPtr;

    #[test]
    fn test_from_slice() {
        let bytes = vec![1, 2, 3, 4, 5];
        let layout = Layout::from_size_align(bytes.len(), 1).unwrap();
        let byte_holder = ByteHolder::from_slice(&bytes, layout);

        assert_eq!(byte_holder.layout.size(), bytes.len());
        assert_eq!(byte_holder.layout.align(), 1);
        assert_eq!(byte_holder.need_drop, true);

        let ptr = byte_holder.bytes.load(Ordering::SeqCst);
        for i in 0..bytes.len() {
            unsafe {
                assert_eq!(*ptr.offset(i as isize), bytes[i]);
            }
        }
    }

    #[test]
    fn test_downcast_ref() {
        let value: u32 = 42;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &value as *const u32 as *const u8,
                std::mem::size_of::<u32>(),
            )
        };
        let layout = Layout::new::<u32>();
        let byte_holder = ByteHolder::from_slice(bytes, layout);

        let ref_value: &u32 = unsafe { byte_holder.downcast_ref() };
        assert_eq!(*ref_value, value);
    }

    #[test]
    fn test_downcast_mut() {
        let value: u32 = 42;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &value as *const u32 as *const u8,
                std::mem::size_of::<u32>(),
            )
        };
        let layout = Layout::new::<u32>();
        let mut byte_holder = ByteHolder::from_slice(bytes, layout);

        let mut_value: &mut u32 = unsafe { byte_holder.downcast_mut() };
        *mut_value = 24;
        assert_eq!(*mut_value, 24);
    }

    #[test]
    fn test_downcast_box() {
        let value: u32 = 42;
        let bytes = unsafe {
            std::slice::from_raw_parts(
                &value as *const u32 as *const u8,
                std::mem::size_of::<u32>(),
            )
        };
        let layout = Layout::new::<u32>();
        let byte_holder = ByteHolder::from_slice(bytes, layout);

        let boxed_value: Box<u32> = unsafe { byte_holder.downcast_box() };
        assert_eq!(*boxed_value, value);
    }

    #[test]
    fn test_drop() {
        let bytes = vec![1, 2, 3, 4, 5];
        let layout = Layout::from_size_align(bytes.len(), 1).unwrap();
        let byte_holder = ByteHolder::from_slice(&bytes, layout);

        drop(byte_holder);
    }
}
