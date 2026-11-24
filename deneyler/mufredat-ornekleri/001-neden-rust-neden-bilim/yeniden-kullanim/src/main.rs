// Güvenli Rust: iade edilen öbek bloğu bir sonraki isteğe yeniden verilebilir.
fn main() {
    let isim = String::from("Ferris");
    let adres1 = isim.as_ptr().addr(); // yalnızca bir sayı: bloğun adresi
    drop(isim); // blok ayırıcıya iade edildi
    let sifre = String::from("gizli-123");
    let adres2 = sifre.as_ptr().addr();
    println!("isim  bloğu: {adres1:#x}");
    println!("sifre bloğu: {adres2:#x}");
    println!("aynı blok mu? {}", if adres1 == adres2 { "evet" } else { "hayır" });
}
