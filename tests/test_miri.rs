#[test]
fn test_mut_borrows() {
    let mut num: isize = 1;
    let a = &raw mut num;
    let b = &raw mut num;
    unsafe {
        *a = 2;
        *b = 2;
    }
}
