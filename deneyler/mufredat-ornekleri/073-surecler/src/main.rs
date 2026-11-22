use std::process::Command;

fn main() {
    println!("ebeveyn: pid {}", std::process::id());
    let durum = Command::new("sh")
        .arg("-c")
        .arg("echo \"çocuk: pid $$, ebeveyni $PPID\"; exit 3")
        .status()
        .unwrap();
    println!("çocuğun çıkış durumu: {durum} (code = {:?})", durum.code());

    let cikti = Command::new("uname").arg("-sr").output().unwrap();
    println!("uname -sr çıktısı yakalandı: {:?}", String::from_utf8_lossy(&cikti.stdout).trim());

    match Command::new("olmayan-program").status() {
        Ok(d) => println!("beklenmedik: {d}"),
        Err(e) => println!("başlatılamadı: {e}"),
    }
}
