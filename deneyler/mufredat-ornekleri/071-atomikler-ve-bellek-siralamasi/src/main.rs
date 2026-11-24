use std::hint::spin_loop;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering::*, fence};
use std::thread;

static X: AtomicU32 = AtomicU32::new(0);
static Y: AtomicU32 = AtomicU32::new(0);
static R1: AtomicU32 = AtomicU32::new(0);
static R2: AtomicU32 = AtomicU32::new(0);
static TUR: AtomicU64 = AtomicU64::new(0); // başlama işareti
static BITEN: AtomicU64 = AtomicU64::new(0); // tamamlanan iş sayısı

// Bir koşul gerçekleşene kadar bekle: önce kısa süre dön (spin), sonra işlemciyi bırak (yield).
// yield, iş parçacıkları mantıksal işlemciden fazlaysa da (ör. 2 işlemcili sanal makine) programın bitmesini sağlar.
fn bekle(kosul: impl Fn() -> bool) {
    let mut donus = 0u32;
    while !kosul() {
        if donus < 1000 {
            spin_loop();
            donus += 1;
        } else {
            thread::yield_now();
        }
    }
}

// Önce kendi değişkenine 1 yaz, sonra öbürünü oku. Dört sıralama sürümü.
#[inline(never)]
fn yaz_oku_gevsek(yaz: &AtomicU32, oku: &AtomicU32) -> u32 {
    yaz.store(1, Relaxed);
    oku.load(Relaxed)
}
#[inline(never)]
fn yaz_oku_edin_birak(yaz: &AtomicU32, oku: &AtomicU32) -> u32 {
    yaz.store(1, Release);
    oku.load(Acquire)
}
#[inline(never)]
fn yaz_oku_sirali(yaz: &AtomicU32, oku: &AtomicU32) -> u32 {
    yaz.store(1, SeqCst);
    oku.load(SeqCst)
}
#[inline(never)]
fn yaz_bariyer_oku(yaz: &AtomicU32, oku: &AtomicU32) -> u32 {
    yaz.store(1, Relaxed);
    fence(SeqCst); // bellek bariyeri
    oku.load(Relaxed)
}

#[inline(never)]
fn yaz_zayif_bariyer_oku(yaz: &AtomicU32, oku: &AtomicU32) -> u32 {
    yaz.store(1, Relaxed);
    fence(AcqRel); // SeqCst'ten zayıf bariyer
    oku.load(Relaxed)
}

fn dene(ad: &str, f: fn(&AtomicU32, &AtomicU32) -> u32, n: u64) {
    TUR.store(0, SeqCst);
    BITEN.store(0, SeqCst);
    let isci = |ben_x: bool| {
        move || {
            for tur in 1..=n {
                bekle(|| TUR.load(Acquire) >= tur);
                if ben_x { R1.store(f(&X, &Y), Relaxed) } else { R2.store(f(&Y, &X), Relaxed) }
                BITEN.fetch_add(1, AcqRel);
            }
        }
    };
    let a = thread::spawn(isci(true));
    let b = thread::spawn(isci(false));
    let mut ikisi_sifir = 0;
    for tur in 1..=n {
        X.store(0, Relaxed);
        Y.store(0, Relaxed);
        TUR.store(tur, Release); // ikisini aynı anda başlat
        bekle(|| BITEN.load(Acquire) >= 2 * tur);
        if R1.load(Relaxed) == 0 && R2.load(Relaxed) == 0 {
            ikisi_sifir += 1; // ikisi de öbürünün yazdığını görmedi!
        }
    }
    a.join().unwrap();
    b.join().unwrap();
    println!("{ad:<23}: {n} turda r1 = r2 = 0 sonucu {ikisi_sifir} kez görüldü");
}

fn main() {
    let n = 1_000_000;
    match std::env::args().nth(1).as_deref() {
        Some("zayif-bariyer") => return dene("Relaxed + fence(AcqRel)", yaz_zayif_bariyer_oku, n),
        Some("gevsek") => return dene("Relaxed", yaz_oku_gevsek, n),
        _ => {}
    }
    dene("Relaxed", yaz_oku_gevsek, n);
    dene("Release/Acquire", yaz_oku_edin_birak, n);
    dene("SeqCst", yaz_oku_sirali, n);
    dene("Relaxed + fence(SeqCst)", yaz_bariyer_oku, n);
}
