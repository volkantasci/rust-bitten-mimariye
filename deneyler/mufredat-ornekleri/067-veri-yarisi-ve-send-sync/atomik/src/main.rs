use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

static SAYAC: AtomicU64 = AtomicU64::new(0);

#[inline(never)]
fn artir(n: u64) {
    for _ in 0..n {
        SAYAC.fetch_add(1, Ordering::Relaxed); // okuma-değiştirme-yazma tek, bölünmez adımda
    }
}

fn main() {
    thread::scope(|s| {
        s.spawn(|| artir(10_000_000));
        s.spawn(|| artir(10_000_000));
    });
    println!("beklenen 20000000, bulunan {}", SAYAC.load(Ordering::Relaxed));
}
