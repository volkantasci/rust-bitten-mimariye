// unsafe Rust: iade edilmiş bloğu eski adresinden okumak. Bu bir tanımsız davranıştır.
fn main() {
    let isim = String::from("Ferris");
    let p: *const u8 = isim.as_ptr(); // ham işaretçi: yalnızca bir adres
    let n = isim.len(); // 6 bayt
    drop(isim); // blok iade edildi; p artık sarkan bir işaretçi
    let sifre = String::from("gizli-123");
    let mut okunan = Vec::new();
    for i in 0..n {
        okunan.push(unsafe { *p.add(i) }); // tanımsız davranış: iade edilmiş belleği okumak
    }
    print!("eski adresteki {n} bayt:");
    for b in &okunan {
        print!(" {b:02x}");
    }
    println!("  -> {:?}", String::from_utf8_lossy(&okunan));
    drop(sifre);
}
