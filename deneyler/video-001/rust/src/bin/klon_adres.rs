// klon.rs ile aynı; ek olarak referansın hâlâ isim'in belleğine mi baktığını yazdırır
fn main() {
    let isim = String::from("Ferris");
    let referans = &isim.clone();
    println!("aynı bellek mi? {}", isim.as_ptr() == referans.as_ptr());
    drop(isim);
    println!("isim = {referans}");
}
