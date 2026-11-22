/// # Safety
/// `p` geçerli, hizalı ve okunabilir bir i32'yi göstermelidir.
unsafe fn oku(p: *const i32) -> i32 {
    *p // 2024 sürümünde: unsafe fn gövdesi de unsafe blok ister (uyarı)
}

fn main() {
    let x = 7;
    // SAFETY: p, hâlâ yaşayan x'i gösteriyor.
    let y = unsafe { oku(&x) };
    println!("{y}");
}
