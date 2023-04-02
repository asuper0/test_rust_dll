use std::slice;

#[no_mangle]
pub extern "C" fn array_sum(array: *const u32, len: usize) -> u32 {
    let array = unsafe {
        assert!(!array.is_null());
        slice::from_raw_parts(array, len)
    };

    array.iter().sum::<u32>() 
}

#[no_mangle]
pub extern "C" fn array_sort(array: *mut u32, len: usize) {
    let array = unsafe {
        assert!(!array.is_null());
        slice::from_raw_parts_mut(array, len)
    };

    array.sort()
}
