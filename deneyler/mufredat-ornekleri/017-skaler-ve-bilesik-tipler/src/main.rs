use std::mem::{align_of, offset_of, size_of};

macro_rules! goster {
    ($($t:ty),*) => { $(
        println!("{:<12} boyut {:>2}  hizalama {:>2}", stringify!($t), size_of::<$t>(), align_of::<$t>());
    )* };
}

fn main() {
    goster!(bool, u8, i16, u32, i64, u128, usize, f32, f64, char, (), (u8, u32), [u16; 5]);
    println!("(u8, u32) alanları: .0@{} .1@{}", offset_of!((u8, u32), 0), offset_of!((u8, u32), 1));
    println!();
    println!("300_i32 as u8     = {}", 300_i32 as u8);
    println!("-1_i32 as u32     = {}", -1_i32 as u32);
    println!("3.99_f64 as i32   = {}", 3.99_f64 as i32);
    println!("-3.99_f64 as i32  = {}", -3.99_f64 as i32);
    println!("1e20_f64 as i32   = {}", 1e20_f64 as i32);
    println!("f64::NAN as i32   = {}", f64::NAN as i32);
    println!("u8::try_from(300) = {:?}", u8::try_from(300_i32));
}
