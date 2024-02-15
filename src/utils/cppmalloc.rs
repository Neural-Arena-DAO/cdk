use std::mem::size_of;

//
// imports from C++
//
#[no_mangle]
pub unsafe fn malloc(
    size: usize
) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    let layout = std::alloc::Layout::from_size_align_unchecked(size + size_of::<usize>(), align);
    let ptr = std::alloc::alloc(layout);
    *ptr.cast::<usize>() = size;
    ptr.add(size_of::<usize>())
}

#[no_mangle]
pub unsafe fn free(
    ptr: *mut u8
) {
    let ptr = ptr.sub(size_of::<usize>());
    let size = *ptr.cast::<usize>();
    let align = std::mem::align_of::<usize>();
    let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
    std::alloc::dealloc(ptr, layout);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn _ZdlPv(
    _ptr: *mut u8
) {
    panic!("delete called from C++");
}

