use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

fn main() {
    let sayac = AtomicU64::new(0);
    thread::scope(|s| {
        for _ in 0..2 {
            s.spawn(|| {
                for _ in 0..10_000_000 {
                    sayac.fetch_add(1, Ordering::Relaxed); // tek, bölünmez adım
                }
            });
        }
    });
    println!("beklenen 20000000, bulunan {}", sayac.load(Ordering::Relaxed));
}
