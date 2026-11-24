// Cargo bu dosyayı, yardimci crate'ini derlemeden ÖNCE derleyip çalıştırır.
// Program, derlemeyi başlatan kişinin yetkileriyle çalışır: dosya okuyabilir, program başlatabilir.
fn main() {
    let ev = std::env::var("HOME").unwrap_or_default();
    let girdi = std::fs::read_dir(&ev).map(|d| d.count()).unwrap_or(0);
    println!("cargo::warning=build.rs çalıştı: HOME = {ev}, orada {girdi} girdi görebiliyorum");
}
