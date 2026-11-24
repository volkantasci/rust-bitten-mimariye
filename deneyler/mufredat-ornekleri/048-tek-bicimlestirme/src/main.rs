use std::hint::black_box;
use std::ops::Add;

#[inline(never)]
fn ikiye_katla<T: Add<Output = T> + Copy>(x: T) -> T {
    x + x
}

fn main() {
    let a = ikiye_katla(black_box(21_i32));
    let b = ikiye_katla(black_box(1.25_f64));
    let c = ikiye_katla(black_box(7_u8));
    println!("{a} {b} {c}");
}
