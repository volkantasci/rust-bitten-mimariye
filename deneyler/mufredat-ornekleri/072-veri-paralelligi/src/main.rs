use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::thread;
use std::time::Instant;

fn agir(n: u64) -> u64 {
    (0..200).fold(n, |a, i| a.wrapping_mul(6364136223846793005).wrapping_add(i))
}

struct Bitisik {
    a: AtomicU64,
    b: AtomicU64, // a ile aynı 64 baytlık önbellek satırında
}

#[repr(align(64))]
struct Hizali(AtomicU64); // her biri ayrı önbellek satırında

fn yaris(a: &AtomicU64, b: &AtomicU64) -> f64 {
    let t = Instant::now();
    thread::scope(|s| {
        s.spawn(|| for _ in 0..50_000_000 { a.fetch_add(1, Relaxed); });
        s.spawn(|| for _ in 0..50_000_000 { b.fetch_add(1, Relaxed); });
    });
    t.elapsed().as_secs_f64() * 1e3
}

fn main() {
    let veri: Vec<u64> = (0..5_000_000).collect();
    let t = Instant::now();
    let sirali = veri.iter().map(|&x| agir(x)).fold(0, u64::wrapping_add);
    let ts = t.elapsed().as_secs_f64() * 1e3;
    let t = Instant::now();
    let paralel = veri.par_iter().map(|&x| agir(x)).reduce(|| 0, u64::wrapping_add);
    let tp = t.elapsed().as_secs_f64() * 1e3;
    println!("rayon iş parçacığı: {}", rayon::current_num_threads());
    println!("sıralı  : {ts:>7.1} ms");
    println!("paralel : {tp:>7.1} ms  (hızlanma {:.2}×, sonuç aynı: {})", ts / tp, sirali == paralel);

    let bitisik = Bitisik { a: AtomicU64::new(0), b: AtomicU64::new(0) };
    let (h1, h2) = (Hizali(AtomicU64::new(0)), Hizali(AtomicU64::new(0)));
    println!("yanlış paylaşım (aynı satır): {:>7.1} ms", yaris(&bitisik.a, &bitisik.b));
    println!("ayrı önbellek satırları     : {:>7.1} ms", yaris(&h1.0, &h2.0));
}
