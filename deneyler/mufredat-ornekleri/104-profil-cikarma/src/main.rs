use std::hint::black_box;

#[inline(never)]
fn yavas_arama(v: &[u32], hedefler: &[u32]) -> usize {
    hedefler.iter().filter(|h| v.contains(h)).count() // her hedef için doğrusal tarama
}

#[inline(never)]
fn hizli_hazirlik(v: &[u32]) -> Vec<u32> {
    let mut s = v.to_vec();
    s.sort_unstable();
    s
}

#[inline(never)]
fn hizli_arama(sirali: &[u32], hedefler: &[u32]) -> usize {
    hedefler.iter().filter(|h| sirali.binary_search(h).is_ok()).count()
}

fn main() {
    let v: Vec<u32> = (0..20_000u32).map(|i| i.wrapping_mul(2_654_435_761)).collect();
    let hedefler: Vec<u32> = v.iter().step_by(10).copied().collect(); // 2000 hedef, hepsi var
    let a = yavas_arama(black_box(&v), black_box(&hedefler));
    let s = hizli_hazirlik(black_box(&v));
    let b = hizli_arama(black_box(&s), black_box(&hedefler));
    println!("bulunan: {a} / {b}");
}
