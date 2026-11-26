// Rust'ın işaretsiz tam sayı tipleri: kaç bayt, kaç bit; u8 ve u32'nin aralığı
fn main() {
    println!("u8   : {:>2} bayt, {:>3} bit", size_of::<u8>(), u8::BITS);
    println!("u16  : {:>2} bayt, {:>3} bit", size_of::<u16>(), u16::BITS);
    println!("u32  : {:>2} bayt, {:>3} bit", size_of::<u32>(), u32::BITS);
    println!("u64  : {:>2} bayt, {:>3} bit", size_of::<u64>(), u64::BITS);
    println!("u128 : {:>2} bayt, {:>3} bit", size_of::<u128>(), u128::BITS);
    println!("u8 aralığı   : {} ..= {}", u8::MIN, u8::MAX);
    println!("u32 aralığı  : {} ..= {}", u32::MIN, u32::MAX);
    let harf: u8 = b'A'; // bayt değişmezi: tipi u8
    println!("b'A'         : {harf} ({} bayt)", size_of_val(&harf));
}
