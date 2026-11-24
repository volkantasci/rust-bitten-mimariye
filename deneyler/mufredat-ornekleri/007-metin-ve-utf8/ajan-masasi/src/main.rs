// Ajan Masası (K1): bir ajanın yazdığı sepet hesabı. Derleniyor ve çalışıyor; sorun nerede?
fn kurusa_cevir(tl: f64) -> i64 {
    (tl * 100.0) as i64 // ajanın kodu: TL'yi kuruşa çevir
}

fn main() {
    let fiyatlar = [0.10_f64, 0.20];
    let mut toplam = 0.0;
    for f in fiyatlar {
        toplam += f;
    }
    println!("toplam (f64)       = {toplam}");
    println!("toplam == 0.30     → {}", toplam == 0.30);
    println!("1.15 TL kaç kuruş? = {}", kurusa_cevir(1.15));
    println!("1.15 * 100.0       = {}", 1.15_f64 * 100.0);
    // Düzeltme: tutarları baştan kuruş cinsinden tamsayı olarak tut
    let kuruslar = [10_i64, 20, 115];
    let mut k = 0;
    for x in kuruslar {
        k += x;
    }
    println!("toplam (kuruş)     = {k} → {}.{:02} TL", k / 100, k % 100);
}
