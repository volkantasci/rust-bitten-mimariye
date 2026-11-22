use std::hint::black_box;

fn main() {
    let x: u8 = black_box(255); // derleyici değeri önceden bilemesin
    println!("checked_add(1)     = {:?}", x.checked_add(1));
    println!("wrapping_add(1)    = {}", x.wrapping_add(1));
    println!("saturating_add(1)  = {}", x.saturating_add(1));
    println!("overflowing_add(1) = {:?}", x.overflowing_add(1));
    let eksi_bir: i8 = -1;
    println!("-1i8 bitleri       = {eksi_bir:08b}");
    println!("i8 aralığı         = {} ..= {}", i8::MIN, i8::MAX);
    println!("x + 1              = {}", x + 1); // debug: panik, release: sarar
}
