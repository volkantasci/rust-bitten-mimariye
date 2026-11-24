// İade sonrası kullanım (001'deki program). AddressSanitizer ile derleyince her erişim,
// adresin gölge baytına bakılarak denetlenir (her 8 bayta 1 gölge bayt).
fn main() {
    let isim = String::from("Ferris");
    let p: *const u8 = isim.as_ptr();
    drop(isim); // blok iade edildi
    let sifre = String::from("gizli-123");
    let ilk = unsafe { *p }; // tanımsız davranış: iade edilmiş belleği okumak
    println!("ilk bayt: {ilk:#04x}");
    drop(sifre);
}
