fn main() {
    let mut v: Vec<u64> = Vec::new();
    println!("başlangıç: len {}, cap {}", v.len(), v.capacity());
    let mut onceki_cap = v.capacity();
    let mut kopyalanan = 0; // yeniden ayırmada taşınan toplam eleman
    for i in 0..1000u64 {
        if v.len() == v.capacity() {
            kopyalanan += v.len();
        }
        v.push(i);
        if v.capacity() != onceki_cap {
            println!("len {:>4} → cap {:>4}", v.len(), v.capacity());
            onceki_cap = v.capacity();
        }
    }
    println!("1000 push, yeniden ayırmada en fazla {kopyalanan} eleman taşındı (≈ push başına {:.2})",
             kopyalanan as f64 / 1000.0);
    let hazir: Vec<u64> = Vec::with_capacity(1000);
    println!("Vec::with_capacity(1000): cap {}", hazir.capacity());
}
