#![allow(unused)]
fn main() {
    use std::cell::UnsafeCell;
    use std::mem::MaybeUninit;

    let uc = UnsafeCell::new(5);
    assert_eq!(uc.into_inner(), 5);
}