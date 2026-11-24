// Ajanın ilk sürümü: en uzun kelimeyi bulan fonksiyon vektörün sahipliğini alıyor.
fn en_uzun(kelimeler: Vec<String>) -> String {
    let mut en = String::new();
    for k in kelimeler {
        if k.len() > en.len() {
            en = k;
        }
    }
    en
}

fn main() {
    let mut kelimeler = Vec::new();
    for i in 0..1000 {
        kelimeler.push(format!("kelime{i}"));
    }
    let mut toplam = 0;
    for _ in 0..100 {
        toplam += en_uzun(kelimeler).len(); // ilk turda taşınır
    }
    println!("toplam uzunluk: {toplam}");
}
