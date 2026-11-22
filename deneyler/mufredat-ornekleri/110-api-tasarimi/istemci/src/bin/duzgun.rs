use kutuphane::{Hata, Kullanici, selamla};

fn aciklama(h: Hata) -> &'static str {
    match h {
        Hata::Bulunamadi => "bulunamadı",
        Hata::YetkiYok => "yetki yok",
        _ => "bilinmeyen hata", // ileride eklenecek türler için
    }
}

fn main() {
    let sahipli = String::from("Ayşe");
    println!("{}", selamla("Ali"));
    println!("{}", selamla(&sahipli));
    println!("{}", selamla(sahipli));
    let k = Kullanici::yeni("Can");
    k.yeniden_adlandir("Cem"); // dönüş değeri atıldı
    println!("{} {}", k.ad, aciklama(Hata::Bulunamadi));
}
