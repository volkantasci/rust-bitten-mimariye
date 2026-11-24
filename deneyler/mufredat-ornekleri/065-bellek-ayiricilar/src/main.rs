use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

// Mikroskop: bütün ayırmaları sayan, işi sistemin ayırıcısına (System) devreden bir ayırıcı
struct Sayan;
static AYIRMA: AtomicUsize = AtomicUsize::new(0);
static BUYUTME: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Sayan {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        AYIRMA.fetch_add(1, Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, yeni: usize) -> *mut u8 {
        BUYUTME.fetch_add(1, Relaxed);
        unsafe { System.realloc(p, l, yeni) }
    }
}

#[global_allocator]
static AYIRICI: Sayan = Sayan;

fn olc(ad: &str, f: impl FnOnce()) {
    let (a0, b0) = (AYIRMA.load(Relaxed), BUYUTME.load(Relaxed));
    f();
    println!("{ad:<45} ayırma {:>5}  büyütme {:>3}", AYIRMA.load(Relaxed) - a0, BUYUTME.load(Relaxed) - b0);
}

// Art arda istenen aynı boyuttaki bloklar arasındaki adres farkı: ayırıcının gerçekte ayırdığı blok boyu.
// İlk birkaç kutu daha önce iade edilmiş bir bloğa düşebilir; bu yüzden en sık görülen fark alınır.
fn blok_adimi<const N: usize>() -> isize {
    let kutular: Vec<Box<[u8; N]>> = (0..8).map(|_| Box::new([0u8; N])).collect();
    let a: Vec<isize> = kutular.iter().map(|k| &**k as *const [u8; N] as isize).collect();
    let mut farklar: Vec<isize> = a.windows(2).map(|w| w[1] - w[0]).collect();
    farklar.sort();
    let mut en_sik = (0, 0);
    for f in farklar.chunk_by(|x, y| x == y) {
        if f.len() > en_sik.1 { en_sik = (f[0], f.len()); }
    }
    black_box(kutular);
    en_sik.0
}

fn main() {
    // 1) Ayırıcı isteği yukarı yuvarlar: boyut sınıfları
    println!("istenen   8 bayt → bloklar {} bayt arayla", blok_adimi::<8>());
    println!("istenen  25 bayt → bloklar {} bayt arayla", blok_adimi::<25>());
    println!("istenen 100 bayt → bloklar {} bayt arayla", blok_adimi::<100>());

    // 2) Aynı boyut sınıfı ve "son iade edilen ilk verilir": 8 baytlık kutuları iade et, 24 baytlık iste
    let kucukler: Vec<Box<[u8; 8]>> = (0..3).map(|_| Box::new([0u8; 8])).collect();
    let eski: Vec<usize> = kucukler.iter().map(|k| &**k as *const [u8; 8] as usize).collect();
    drop(kucukler);
    let yeniler: Vec<Box<[u8; 24]>> = (0..3).map(|_| Box::new([0u8; 24])).collect();
    let yeni: Vec<usize> = yeniler.iter().map(|k| &**k as *const [u8; 24] as usize).collect();
    let ters: Vec<usize> = eski.iter().rev().copied().collect();
    println!("24 baytlık yeni kutular, iade edilen 8 baytlıkların adreslerini ters sırayla aldı mı: {}", yeni == ters);
    black_box(yeniler);

    // 3) Kaç ayırma? Sayan ayırıcının sayaçları
    olc("String::new + 1000 × push_str", || {
        let mut s = String::new();
        for _ in 0..1000 { s.push_str("ab"); }
        black_box(s);
    });
    olc("String::with_capacity(2000) + 1000 × push_str", || {
        let mut s = String::with_capacity(2000);
        for _ in 0..1000 { s.push_str("ab"); }
        black_box(s);
    });
    olc("1000 × format!(\"{i}\") → Vec<String>", || {
        let v: Vec<String> = (0..1000).map(|i| format!("{i}")).collect();
        black_box(v);
    });
    olc("1000 × Box::new(i)", || {
        let v: Vec<Box<u64>> = (0..1000).map(Box::new).collect();
        black_box(v);
    });
}
