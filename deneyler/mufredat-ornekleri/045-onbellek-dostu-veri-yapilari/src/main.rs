use std::collections::LinkedList;
use std::hint::black_box;
use std::time::Instant;

fn olc<F: FnMut() -> u64>(ad: &str, mut f: F) {
    let t = Instant::now();
    let s = f();
    println!("{ad:<38} {:>8.2} ms", t.elapsed().as_secs_f64() * 1e3);
    black_box(s);
}

#[inline(never)]
fn satir_oncelikli(m: &[u32], n: usize) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        for j in 0..n {
            s += m[i * n + j] as u64; // ardışık adresler
        }
    }
    s
}

#[inline(never)]
fn sutun_oncelikli(m: &[u32], n: usize) -> u64 {
    let mut s = 0u64;
    for j in 0..n {
        for i in 0..n {
            s += m[i * n + j] as u64; // her adımda 16 KiB ileri atla
        }
    }
    s
}

// İşaretçi kovalama: bir sonraki indis ancak bu okuma bitince bilinir
#[inline(never)]
fn kovala(sonraki: &[usize], adim: usize) -> u64 {
    let mut i = 0;
    for _ in 0..adim {
        i = sonraki[i];
    }
    i as u64
}

#[derive(Clone, Copy)]
struct Parcacik { x: f32, y: f32, z: f32, kutle: f32, hiz: [f32; 4] } // 32 bayt

fn main() {
    // 1) Satır satır mı sütun sütun mu? (4096 × 4096 u32 = 64 MiB)
    {
        let n = black_box(4096usize);
        let m: Vec<u32> = (0..n * n).map(|x| x as u32 & 7).collect();
        olc("matris satır öncelikli gezinme", || satir_oncelikli(&m, n));
        olc("matris sütun öncelikli gezinme", || sutun_oncelikli(&m, n));
    }

    // 2) Bitişik dizi mi, bağlı liste mi? (5 milyon eleman)
    {
        let k = 5_000_000u64;
        let v: Vec<u64> = (0..k).collect();
        let l: LinkedList<u64> = (0..k).collect();
        olc("Vec<u64> toplamı", || v.iter().sum());
        olc("LinkedList<u64> toplamı", || l.iter().sum());
    }

    // 3) İşaretçi kovalama: aynı zincir, iki bellek düzeni (8 milyon × 8 bayt = 64 MiB)
    {
        let k = 8_000_000usize;
        let ardisik: Vec<usize> = (0..k).map(|i| (i + 1) % k).collect();
        // Sattolo algoritması: bütün elemanları tek turda gezen rastgele bir döngü (sabit tohum)
        let mut rastgele: Vec<usize> = (0..k).collect();
        let mut x = 0x2545_F491_4F6C_DD1Du64;
        for i in (1..k).rev() {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            rastgele.swap(i, (x % i as u64) as usize);
        }
        // Doğrulama: 0'dan başlayan zincir bütün elemanları gezip 0'a dönüyor mu?
        let (mut i, mut tur) = (rastgele[0], 1usize);
        while i != 0 {
            i = rastgele[i];
            tur += 1;
        }
        println!("rastgele zincirin tur uzunluğu: {tur} (eleman sayısı {k})");
        olc("kovalama: sonraki eleman yanda", || kovala(&ardisik, k));
        olc("kovalama: sonraki eleman rastgele yerde", || kovala(&rastgele, k));
    }

    // 4) Yapılar dizisi (AoS) mi, diziler yapısı (SoA) mı? Yalnızca kütleleri topla
    {
        let p = vec![Parcacik { x: 0.0, y: 0.0, z: 0.0, kutle: 1.0, hiz: [0.0; 4] }; 4_000_000];
        let kutleler: Vec<f32> = p.iter().map(|q| q.kutle).collect();
        // Kütlelerin bit desenleri tam sayı olarak toplanır (sonucun anlamı yok, yalnız ölçüm için):
        // tam sayı toplamı vektörleşebilir, böylece süreyi toplama gecikmesi değil bellek trafiği belirler
        olc("AoS: Vec<Parcacik> içinden kütle", || p.iter().map(|q| q.kutle.to_bits() as u64).sum());
        olc("SoA: Vec<f32> kütle dizisi", || kutleler.iter().map(|q| q.to_bits() as u64).sum());
        black_box((p[0].x, p[0].y, p[0].z, p[0].hiz));
    }
}
