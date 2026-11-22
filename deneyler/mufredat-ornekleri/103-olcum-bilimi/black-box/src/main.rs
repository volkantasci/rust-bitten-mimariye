use std::hint::black_box;
use std::time::Instant;

fn agir(n: u64) -> u64 {
    (0..n).map(|i| i.wrapping_mul(i) ^ (i >> 3)).fold(0, u64::wrapping_add)
}

fn main() {
    let t = Instant::now();
    for _ in 0..100 {
        let _ = agir(1_000_000); // sonuç kullanılmıyor: derleyici hesabı silebilir
    }
    println!("sonuç atılınca      : {:>10.3} ms", t.elapsed().as_secs_f64() * 1e3);

    let t = Instant::now();
    for _ in 0..100 {
        black_box(agir(black_box(1_000_000))); // "bu değer kullanılıyor, girdiyi bilmiyorsun"
    }
    println!("black_box ile       : {:>10.3} ms", t.elapsed().as_secs_f64() * 1e3);
}
