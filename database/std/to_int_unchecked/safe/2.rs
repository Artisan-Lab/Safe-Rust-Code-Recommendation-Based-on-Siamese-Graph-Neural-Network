#![allow(unused)]
fn main() {
    let value = 4.6_f64;
    let rounded = value as u16;
    assert_eq!(rounded, 4);

    let value = -128.9_f64;
    let rounded = value as i8;
    assert_eq!(rounded, i8::MIN);
}