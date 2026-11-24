use std::hint::black_box;
use std::time::Instant;

#[inline(never)]
fn toplam_basit(v: &[f32]) -> f32 {
    v.iter().sum() // kayan noktada toplama sırası sonucu değiştirir: derleyici sırayı koruyor
}

#[inline(never)]
fn toplam_8_serit(v: &[f32]) -> f32 {
    let mut ser = [0.0f32; 8]; // 8 bağımsız toplayıcı: sırayı biz değiştirdik, izin verdik
    let parcalar = v.chunks_exact(8);
    let kalan = parcalar.remainder();
    for p in parcalar {
        for i in 0..8 {
            ser[i] += p[i];
        }
    }
    ser.iter().sum::<f32>() + kalan.iter().sum::<f32>()
}

#[inline(never)]
fn toplam_basit_f64(v: &[f64]) -> f64 {
    v.iter().sum() // aynı sıra, daha geniş tip: f64'ün 53 bitlik anlamlı kısmı ara toplamları tam tutar
}

#[inline(never)]
fn toplam_tamsayi(v: &[i32]) -> i32 {
    v.iter().fold(0i32, |a, &b| a.wrapping_add(b)) // tamsayıda sıra fark etmez: serbestçe vektörleşir
}

fn main() {
    let n = 16_000_000;
    let f: Vec<f32> = (0..n).map(|i| (i % 7) as f32 * 0.5).collect();
    let d: Vec<f64> = f.iter().map(|&x| x as f64).collect();
    let t: Vec<i32> = (0..n).map(|i| (i % 7) as i32).collect();
    let olc = |ad: &str, is: &dyn Fn() -> String| {
        let t0 = Instant::now();
        let mut s = String::new();
        for _ in 0..10 { s = black_box(is()); }
        println!("{ad:<22} {:>7.2} ms  sonuç {s}", t0.elapsed().as_secs_f64() * 1e3 / 10.0);
    };
    olc("f32, basit toplam", &|| toplam_basit(black_box(&f)).to_string());
    olc("f32, 8 şeritli toplam", &|| toplam_8_serit(black_box(&f)).to_string());
    olc("f64, basit toplam", &|| toplam_basit_f64(black_box(&d)).to_string());
    olc("i32, toplam", &|| toplam_tamsayi(black_box(&t)).to_string());
    // Gerçek toplam 23999997,5: f32'nin 24 bitlik anlamlı kısmı bu büyüklükte yalnız çift sayıları tutar
    println!(
        "anlamlı bit: f32 {}, f64 {}; 23999997.5 sayısı f32'de {} olur",
        f32::MANTISSA_DIGITS,
        f64::MANTISSA_DIGITS,
        23_999_997.5_f32
    );
}
