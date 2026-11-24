// Düzeltilmiş: fonksiyon vektörü ödünç alır, en uzun kelimeye bir dilim döndürür.
fn en_uzun(kelimeler: &[String]) -> &str {
    let mut en = "";
    for k in kelimeler {
        if k.len() > en.len() {
            en = k.as_str();
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
        toplam += en_uzun(&kelimeler).len(); // kopya yok
    }
    println!("toplam uzunluk: {toplam}");
}
