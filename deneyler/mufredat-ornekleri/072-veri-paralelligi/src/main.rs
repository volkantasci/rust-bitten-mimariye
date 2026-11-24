use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::thread;
use std::time::Instant;

fn agir(n: u64) -> u64 {
    (0..200).fold(n, |a, i| a.wrapping_mul(6364136223846793005).wrapping_add(i))
}

struct Bitisik {
    a: AtomicU64,
    b: AtomicU64, // a'nın hemen ardında: aynı 64 baytlık önbellek satırında
}

#[repr(align(64))]
struct Hizali(AtomicU64); // 64 bayta hizalı: her biri ayrı önbellek satırında

// Kendin dene: iki sayaç tek bir 64 bayta hizalı yapıda, aralarında N baytlık dolgu
#[repr(C, align(64))]
struct Cift<const N: usize> {
    a: AtomicU64,
    _dolgu: [u8; N],
    b: AtomicU64,
}

fn dolgu_dene<const N: usize>() {
    let c = Cift::<N> { a: AtomicU64::new(0), _dolgu: [0; N], b: AtomicU64::new(0) };
    println!("dolgu {N:>2} bayt: a @ {}, b @ {} → {:>6.1} ms", satir(&c.a), satir(&c.b), yaris(&c.a, &c.b));
}

fn yaris(a: &AtomicU64, b: &AtomicU64) -> f64 {
    let t = Instant::now();
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..50_000_000 {
                a.fetch_add(1, Relaxed);
            }
        });
        s.spawn(|| {
            for _ in 0..50_000_000 {
                b.fetch_add(1, Relaxed);
            }
        });
    });
    t.elapsed().as_secs_f64() * 1e3
}

fn satir(p: &AtomicU64) -> String {
    let adres = p as *const AtomicU64 as usize;
    format!("{adres:#x} (satır {:#x})", adres / 64)
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("dolgu") {
        dolgu_dene::<48>(); // b, a ile aynı 64 baytlık satırda (konum 56)
        dolgu_dene::<56>(); // b bir sonraki satırda (konum 64)
        return;
    }
    let veri: Vec<u64> = (0..5_000_000).collect();
    let t = Instant::now();
    let sirali = veri.iter().map(|&x| agir(x)).fold(0, u64::wrapping_add);
    let ts = t.elapsed().as_secs_f64() * 1e3;
    // Süresi ölçülen paralel hesap; her öğeyi hangi iş parçacığının işlediği de sayılır.
    // Sayaçlar Hizali (64 bayta hizalı): her biri kendi önbellek satırında, yanlış paylaşım yok.
    let pay: Vec<Hizali> = (0..rayon::current_num_threads()).map(|_| Hizali(AtomicU64::new(0))).collect();
    let t = Instant::now();
    let paralel = veri
        .par_iter()
        .map(|&x| {
            pay[rayon::current_thread_index().unwrap()].0.fetch_add(1, Relaxed);
            agir(x)
        })
        .reduce(|| 0, u64::wrapping_add);
    let tp = t.elapsed().as_secs_f64() * 1e3;
    println!("rayon iş parçacığı: {}", rayon::current_num_threads());
    // Hangi iş parçacığı kaç öğe işledi? (iş çalan havuzda paylar eşit değildir)
    let paylar: Vec<u64> = pay.iter().map(|x| x.0.load(Relaxed)).collect();
    println!("iş parçacığı başına işlenen öğe: {paylar:?} (toplam {})", paylar.iter().sum::<u64>());
    println!("sıralı  : {ts:>7.1} ms");
    println!("paralel : {tp:>7.1} ms  (hızlanma {:.2}×, sonuç aynı: {})", ts / tp, sirali == paralel);

    let bitisik = Bitisik { a: AtomicU64::new(0), b: AtomicU64::new(0) };
    let (h1, h2) = (Hizali(AtomicU64::new(0)), Hizali(AtomicU64::new(0)));
    println!("bitişik sayaçlar: a @ {}, b @ {}", satir(&bitisik.a), satir(&bitisik.b));
    println!("hizalı sayaçlar : a @ {}, b @ {}", satir(&h1.0), satir(&h2.0));
    println!("yanlış paylaşım (aynı satır): {:>7.1} ms", yaris(&bitisik.a, &bitisik.b));
    println!("ayrı önbellek satırları     : {:>7.1} ms", yaris(&h1.0, &h2.0));
}
