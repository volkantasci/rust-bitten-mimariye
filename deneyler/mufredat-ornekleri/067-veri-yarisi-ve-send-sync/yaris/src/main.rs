// Bilerek yazılmış bir veri yarışı: iki iş parçacığı aynı sayaca senkronizasyonsuz yazar.
// Güvenli Rust bu kodu derlemez (aşağıdaki ortak-mut/ ve rc-gonderilemez/ deneylerine bakın);
// burada `unsafe` ile derleyicinin denetimini kendi sorumluluğumuza alıyoruz.
use std::thread;

static mut SAYAC: u64 = 0;

#[inline(never)]
fn artir(n: u64) {
    for _ in 0..n {
        unsafe {
            let p = &raw mut SAYAC;
            let eski = p.read_volatile(); // 1) oku
            p.write_volatile(eski + 1); //   2) bir ekle, 3) yaz
        }
    }
}

fn main() {
    let n: u64 = std::env::args().nth(1).map_or(10_000_000, |s| s.parse().unwrap());
    let a = thread::spawn(move || artir(n));
    let b = thread::spawn(move || artir(n));
    a.join().unwrap();
    b.join().unwrap();
    println!("beklenen {}, bulunan {}", 2 * n, unsafe { (&raw const SAYAC).read() });
}
