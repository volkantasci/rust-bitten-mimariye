use std::hint::black_box;
use std::time::Instant;

#[inline(never)]
fn dallanan(v: &[u8]) -> u64 {
    let mut s = 0u64;
    for &x in v {
        if x >= 128 {
            s = black_box(s + x as u64); // black_box: derleyici dalı silemesin
        }
    }
    s
}

#[inline(never)]
fn dallanmasiz(v: &[u8]) -> u64 {
    v.iter().map(|&x| (x >= 128) as u64 * x as u64).sum() // koşul aritmetiğe çevrildi
}

fn olc(ad: &str, f: fn(&[u8]) -> u64, v: &[u8]) {
    let t = Instant::now();
    let mut s = 0;
    for _ in 0..10 {
        s += f(black_box(v));
    }
    println!("{ad:<26} {:>8.1} ms   (toplam {s})", t.elapsed().as_secs_f64() * 1e3);
}

fn main() {
    let mut durum = 0x1234_5678_9abc_def1u64;
    let mut veri: Vec<u8> = (0..10_000_000)
        .map(|_| {
            durum ^= durum << 13;
            durum ^= durum >> 7;
            durum ^= durum << 17;
            durum as u8
        })
        .collect();
    olc("dallanan,    karışık veri", dallanan, &veri);
    olc("dallanmasız, karışık veri", dallanmasiz, &veri);
    veri.sort_unstable();
    olc("dallanan,    sıralı veri", dallanan, &veri);
    olc("dallanmasız, sıralı veri", dallanmasiz, &veri);
}
