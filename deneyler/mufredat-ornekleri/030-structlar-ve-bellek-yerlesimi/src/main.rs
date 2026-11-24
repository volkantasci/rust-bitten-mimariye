use std::mem::{offset_of, size_of};

#[allow(dead_code)]
struct Karisik {
    a: u8,
    b: u32,
    c: u16,
}

#[allow(dead_code)]
#[repr(C)] // C ABI'sinin kuralı: alanlar yazıldığı sırada
struct KarisikC {
    a: u8,
    b: u32,
    c: u16,
}

#[allow(dead_code)]
#[repr(C)] // aynı alanlar, büyükten küçüğe yazıldı
struct SiraliC {
    b: u32,
    c: u16,
    a: u8,
}

fn main() {
    println!("repr(Rust): boyut {:>2}  a@{} b@{} c@{}", size_of::<Karisik>(),
             offset_of!(Karisik, a), offset_of!(Karisik, b), offset_of!(Karisik, c));
    println!("repr(C)   : boyut {:>2}  a@{} b@{} c@{}", size_of::<KarisikC>(),
             offset_of!(KarisikC, a), offset_of!(KarisikC, b), offset_of!(KarisikC, c));
    println!("repr(C) büyükten küçüğe: boyut {:>2}  a@{} b@{} c@{}", size_of::<SiraliC>(),
             offset_of!(SiraliC, a), offset_of!(SiraliC, b), offset_of!(SiraliC, c));
}
