use std::collections::LinkedList;
use std::hint::black_box;
use std::time::Instant;

fn olc<F: FnMut() -> u64>(ad: &str, mut f: F) {
    let t = Instant::now();
    let s = f();
    println!("{ad:<34} {:>8.2} ms", t.elapsed().as_secs_f64() * 1e3);
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

#[derive(Clone, Copy)]
struct Parcacik { x: f32, y: f32, z: f32, kutle: f32, hiz: [f32; 4] } // 32 bayt

fn main() {
    // 1) Satır satır mı sütun sütun mu? (4096 × 4096 u32 = 64 MiB)
    let n = black_box(4096usize);
    let m: Vec<u32> = (0..n * n).map(|x| x as u32 & 7).collect();
    olc("matris satır-öncelikli gezinme", || satir_oncelikli(&m, n));
    olc("matris sütun-öncelikli gezinme", || sutun_oncelikli(&m, n));

    // 2) Bitişik dizi mi, bağlı liste mi? (5 milyon eleman)
    let k = 5_000_000u64;
    let v: Vec<u64> = (0..k).collect();
    let l: LinkedList<u64> = (0..k).collect();
    olc("Vec<u64> toplamı", || v.iter().sum());
    olc("LinkedList<u64> toplamı", || l.iter().sum());

    // 3) Yapı dizisi (AoS) mi, dizi yapısı (SoA) mı? Yalnızca kütleleri topla
    let p = vec![Parcacik { x: 0.0, y: 0.0, z: 0.0, kutle: 1.0, hiz: [0.0; 4] }; 4_000_000];
    let kutleler: Vec<f32> = p.iter().map(|q| q.kutle).collect();
    olc("AoS: Vec<Parcacik> içinden kütle", || p.iter().map(|q| q.kutle as f64).sum::<f64>() as u64);
    olc("SoA: Vec<f32> kütle dizisi", || kutleler.iter().map(|&q| q as f64).sum::<f64>() as u64);
    black_box((p[0].x, p[0].y, p[0].z, p[0].hiz));
}
