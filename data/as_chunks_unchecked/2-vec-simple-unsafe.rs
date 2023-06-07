/*
    This is a case from std
    Link: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_chunks_unchecked
    Purpose: Splits the slice into a slice of N-element arrays, assuming that there’s no remainder.
    Replaceable? Yes
*/

#![allow(unused)]
#![feature(slice_as_chunks)]
fn main() {
    let v = vec!['l', 'o', 'r', 'e', 'm', '!'];
    let chunks: &[[char; 1]] = unsafe { v.as_chunks_unchecked() };
    assert_eq!(chunks, &[['l'], ['o'], ['r'], ['e'], ['m'], ['!']]);
}