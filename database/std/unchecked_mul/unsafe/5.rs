#![allow(unused)]
#![feature(nonzero_ops)]
use std::num::NonZeroI8;

fn main() -> (){

    let two = NonZeroI8::new(2).unwrap();
    let four = NonZeroI8::new(4).unwrap();

    assert_eq!(four, unsafe { two.unchecked_mul(two) });


}
