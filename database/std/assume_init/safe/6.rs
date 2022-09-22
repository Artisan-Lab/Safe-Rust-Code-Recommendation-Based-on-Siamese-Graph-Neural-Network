#![allow(unused)]
#![feature(allocator_api, new_uninit)]

fn main() -> Result<(), impl core::fmt::Debug>{
    use std::alloc::System;

    let mut five = Box::<u32, _>::new_in(5,System);


    assert_eq!(*five, 5);
    Ok::<(), std::alloc::AllocError>(())
}