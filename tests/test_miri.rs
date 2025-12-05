use std::alloc::{alloc, dealloc, Layout};
use std::slice;

#[test]
fn test_mut_borrows() {
    let mut num: isize = 1;
    let n = &mut num;

    let a = n as *mut isize;
    let b = n as *mut isize;

    unsafe {
        {
            *a = 2;
        }
        {
            *b = 2;
        }
    }
}
