use std::collections::{BTreeMap, HashMap};
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let notlar = BTreeMap::from([("Ece", 91), ("Ali", 72), ("Can", 85), ("Ada", 64), ("Naz", 78)]);
    println!("sıralı      : {:?}", notlar);
    println!("'B'..'D'    : {:?}", notlar.range("B".."D").collect::<Vec<_>>());
    println!("ilk / son   : {:?} / {:?}", notlar.first_key_value(), notlar.last_key_value());

    let n = 1_000_000u64;
    let anahtarlar: Vec<u64> = (0..n).map(|i| i.wrapping_mul(0x9E37_79B9_7F4A_7C15)).collect();
    let h: HashMap<u64, u64> = anahtarlar.iter().map(|&k| (k, k)).collect();
    let b: BTreeMap<u64, u64> = anahtarlar.iter().map(|&k| (k, k)).collect();
    let t = Instant::now();
    let mut s = 0;
    for k in &anahtarlar { s += h[black_box(k)]; }
    let th = t.elapsed();
    let t = Instant::now();
    for k in &anahtarlar { s += b[black_box(k)]; }
    let tb = t.elapsed();
    println!("1e6 arama   : HashMap {:>6.1} ms, BTreeMap {:>6.1} ms", th.as_secs_f64() * 1e3, tb.as_secs_f64() * 1e3);
    black_box(s);
}
