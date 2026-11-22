fn main() {
    let (a, b) = (0.1_f64, 0.2_f64);
    println!("0.1 + 0.2          = {}", a + b);
    println!("0.1 + 0.2 == 0.3   → {}", a + b == 0.3);
    println!("0.1'in tam değeri  = {:.55}", 0.1_f64);

    let bitler = 0.1_f64.to_bits();
    let us = (bitler >> 52) & 0x7ff;
    println!("0.1 bitleri        = {bitler:064b}");
    println!("  işaret           = {}", bitler >> 63);
    println!("  üs               = {us:011b} ({us} - 1023 = {})", us as i64 - 1023);
    println!("  kesir            = {:052b}", bitler & ((1u64 << 52) - 1));

    println!("16777217 → f32     = {}", 16_777_217_i32 as f32);
    println!("NaN == NaN         → {}", f64::NAN == f64::NAN);
    println!("1.0 / 0.0          = {}", 1.0_f64 / 0.0);
    println!("(0.1+0.2-0.3).abs() < f64::EPSILON → {}", (a + b - 0.3).abs() < f64::EPSILON);
}
