// unsafe Rust: 8 baytlık bir öbek bloğunun sonundan öteye okumak (tanımsız davranış).
fn main() {
    let ad: Box<[u8; 8]> = Box::new(*b"Ferris\0\0");
    let parola: Box<[u8; 8]> = Box::new(*b"gizli123"); // hemen ardından ayrılan komşu blok
    let p = ad.as_ptr(); // ad bloğunun başı
    for satir in 0..5 {
        print!("+{:<2}:", satir * 8);
        for i in 0..8 {
            let bayt = unsafe { *p.add(satir * 8 + i) }; // 8 ve ötesi: ayırmanın dışı
            print!(" {bayt:02x}");
        }
        println!();
    }
    drop(parola);
}
