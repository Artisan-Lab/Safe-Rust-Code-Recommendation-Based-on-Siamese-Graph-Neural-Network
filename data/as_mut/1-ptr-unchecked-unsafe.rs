#![allow(unused)]
fn main() {
    let mut s = [1, 2, 3];
    let ptr: *mut u32 = s.as_mut_ptr();
    let first_value = unsafe { &mut *ptr };
    *first_value = 4;
}