use std::hint::black_box;
use std::time::Instant;

trait Sekil {
    fn alan(&self) -> f64;
}
struct Kare { a: f64 }
impl Sekil for Kare {
    fn alan(&self) -> f64 { self.a * self.a }
}

#[inline(never)]
fn toplam_statik<T: Sekil>(v: &[T]) -> f64 {
    v.iter().map(|s| s.alan()).sum() // derleme anında hangi alan() olduğu bilinir
}

#[inline(never)]
fn toplam_dinamik(v: &[&dyn Sekil]) -> f64 {
    v.iter().map(|s| s.alan()).sum() // her çağrıda adres vtable'dan okunur
}

#[inline(never)]
fn toplam_kutulu(v: &[Box<dyn Sekil>]) -> f64 {
    v.iter().map(|s| s.alan()).sum() // vtable + her nesne ayrı bir öbek bloğunda
}

fn main() {
    let n = 10_000_000;
    let kareler: Vec<Kare> = (0..n).map(|i| Kare { a: (i % 100) as f64 }).collect();
    let referanslar: Vec<&dyn Sekil> = kareler.iter().map(|k| k as &dyn Sekil).collect();
    let kutular: Vec<Box<dyn Sekil>> = (0..n).map(|i| Box::new(Kare { a: (i % 100) as f64 }) as Box<dyn Sekil>).collect();
    for _ in 0..2 {
        let t = Instant::now();
        let a = toplam_statik(black_box(&kareler));
        let ts = t.elapsed().as_secs_f64() * 1e3;
        let t = Instant::now();
        let b = toplam_dinamik(black_box(&referanslar));
        let td = t.elapsed().as_secs_f64() * 1e3;
        let t = Instant::now();
        let c = toplam_kutulu(black_box(&kutular));
        let tk = t.elapsed().as_secs_f64() * 1e3;
        println!("statik {ts:>6.2} ms | &dyn {td:>6.2} ms | Box<dyn> {tk:>6.2} ms | eşit: {}", a == b && b == c);
    }
}
