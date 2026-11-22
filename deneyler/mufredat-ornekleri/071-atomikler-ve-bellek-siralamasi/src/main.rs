use std::hint::spin_loop;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering::*};
use std::thread;

static X: AtomicU32 = AtomicU32::new(0);
static Y: AtomicU32 = AtomicU32::new(0);
static R1: AtomicU32 = AtomicU32::new(0);
static R2: AtomicU32 = AtomicU32::new(0);
static TUR: AtomicU64 = AtomicU64::new(0); // başlama işareti
static BITEN: AtomicU64 = AtomicU64::new(0); // tamamlanan iş sayısı

// Önce yaz, sonra öbürünü oku. Gevşek (Relaxed) ve sıralı (SeqCst) iki sürüm.
#[inline(never)]
fn yaz_oku_gevsek(yaz: &AtomicU32, oku: &AtomicU32) -> u32 {
    yaz.store(1, Relaxed);
    oku.load(Relaxed)
}
#[inline(never)]
fn yaz_oku_sirali(yaz: &AtomicU32, oku: &AtomicU32) -> u32 {
    yaz.store(1, SeqCst);
    oku.load(SeqCst)
}

fn dene(ad: &str, f: fn(&AtomicU32, &AtomicU32) -> u32, n: u64) {
    TUR.store(0, SeqCst);
    BITEN.store(0, SeqCst);
    let isci = |ben_x: bool| {
        move || {
            for tur in 1..=n {
                while TUR.load(Acquire) < tur { spin_loop(); }
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
        while BITEN.load(Acquire) < 2 * tur { spin_loop(); }
        if R1.load(Relaxed) == 0 && R2.load(Relaxed) == 0 {
            ikisi_sifir += 1; // ikisi de öbürünün yazdığını görmedi!
        }
    }
    a.join().unwrap();
    b.join().unwrap();
    println!("{ad:<8}: {n} turda r1 = r2 = 0 sonucu {ikisi_sifir} kez görüldü");
}

fn main() {
    let n = 1_000_000;
    dene("Relaxed", yaz_oku_gevsek, n);
    dene("SeqCst", yaz_oku_sirali, n);
}
