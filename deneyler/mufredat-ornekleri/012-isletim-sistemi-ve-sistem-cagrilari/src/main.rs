use std::hint::black_box;
use std::time::Instant;

#[inline(never)]
fn sradan(x: u32) -> u32 {
    black_box(x)
}

fn main() {
    let n: u32 = std::env::args().nth(1).map_or(1_000_000, |s| s.parse().unwrap());
    let mut toplam = 0u64;

    let t = Instant::now();
    for i in 0..n {
        toplam += sradan(i) as u64; // sıradan fonksiyon çağrısı
    }
    let fonk = t.elapsed();

    let t = Instant::now();
    for _ in 0..n {
        toplam += std::process::id() as u64; // getpid: çekirdeğe gidiş-dönüş
    }
    let cagri = t.elapsed();

    println!("{n} fonksiyon çağrısı : {:>8.2} ns/çağrı", fonk.as_nanos() as f64 / n as f64);
    println!("{n} getpid() çağrısı  : {:>8.2} ns/çağrı", cagri.as_nanos() as f64 / n as f64);
    black_box(toplam);
}
