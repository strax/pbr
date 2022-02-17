use std::alloc::{Allocator, Global, Layout};
use std::{mem, ptr};
use std::ptr::NonNull;
use libc::{size_t, c_void};

const DEFAULT_ALIGN: usize = 16;

pub(crate) unsafe extern "C" fn exr_alloc(size: size_t) -> *mut c_void {
    let (layout, offset) = Layout::new::<Layout>()
        .extend(Layout::from_size_align(size, DEFAULT_ALIGN).unwrap()).unwrap();
    debug_assert_eq!(offset, Layout::new::<Layout>().pad_to_align().size());
    match Global.allocate(layout) {
        Ok(ptr) => {
            let ptr = ptr.as_ptr() as *mut u8;
            ptr.cast::<Layout>().write(layout);
            ptr.add(offset).cast()
        },
        Err(_) => std::alloc::handle_alloc_error(layout)
    }
}

pub(crate) unsafe extern "C" fn exr_free(ptr: *mut c_void) {
    let ptr = ptr.cast::<u8>().sub(Layout::new::<Layout>().pad_to_align().size());
    if ptr.is_null() {
        return
    }
    let layout: Layout = ptr.cast::<Layout>().read();
    Global.deallocate(NonNull::new_unchecked(ptr), layout)
}

#[cfg(test)]
mod test {
    use super::*;

    fn is_aligned<const ALIGN: usize>(ptr: *const c_void) -> bool {
        ptr.to_bits() % ALIGN == 0
    }

    #[test]
    fn test_alloc_dealloc() {
        unsafe {
            let ptr = exr_alloc(123);
            assert!(is_aligned::<DEFAULT_ALIGN>(ptr), "ptr is not aligned to {DEFAULT_ALIGN} bytes");
            let layout = ptr.sub(Layout::new::<Layout>().pad_to_align().size()).cast::<Layout>().read();
            assert_eq!(layout.size(), 123 + 16);
            exr_free(ptr);
        }
    }
}