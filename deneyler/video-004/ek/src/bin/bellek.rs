// 1_000_000'un dört baytı bellekte: her baytın kendi adresi var.
// Adresler her çalıştırmada değişir (sizde farklı çıkar); baytlar değişmez.
fn main() {
    let n: u32 = 1_000_000;
    let ilk = &n as *const u32 as *const u8; // n'nin ilk baytının adresi
    // to_ne_bytes: sayının bellekteki hâli, makinenin kendi bayt sırasıyla
    for (i, bayt) in n.to_ne_bytes().iter().enumerate() {
        println!("{:p} : {bayt:02x}", ilk.wrapping_add(i));
    }
}
