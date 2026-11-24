use std::process::Command;
use std::thread;
use std::time::Duration;

// /proc/<pid>/stat'ın üçüncü alanı sürecin durumudur (R çalışıyor, S uyuyor, Z zombi ...)
fn durum(pid: u32) -> String {
    match std::fs::read_to_string(format!("/proc/{pid}/stat")) {
        Ok(s) => s.rsplit(')').next().unwrap().split_whitespace().next().unwrap().to_string(),
        Err(_) => "yok (/proc'ta kaydı kalmadı)".to_string(),
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("zombi-beklemesiz") {
        // Kendin dene: wait() hiç çağrılmaz; alt süreç üst süreç bitene kadar zombi kalır
        let alt = Command::new("true").spawn().unwrap();
        let pid = alt.id();
        for (bekle, toplam) in [(200, 200), (1000, 1200)] {
            thread::sleep(Duration::from_millis(bekle));
            println!("beklenmeyen alt süreç {pid}, başlatıldıktan {toplam} ms sonra: durum = {}", durum(pid));
        }
        return; // üst süreç biter; zombiyi init (PID 1) toplar
    }
    if std::env::args().nth(1).as_deref() == Some("zombi") {
        // Alt süreç biter ama üst süreç henüz beklemez: çekirdek çıkış durumunu saklar
        let mut alt = Command::new("true").spawn().unwrap();
        let pid = alt.id();
        thread::sleep(Duration::from_millis(200));
        println!("alt süreç {pid} bitti, 200 ms sonra henüz beklenmedi: durum = {}", durum(pid));
        let d = alt.wait().unwrap(); // wait4: çıkış durumu alınır, kayıt silinir
        println!("wait() → {d}; şimdi durum = {}", durum(pid));
        return;
    }
    println!("üst süreç: pid {}", std::process::id());
    let durum = Command::new("sh")
        .arg("-c")
        .arg("echo \"alt süreç: pid $$, üst süreci $PPID\"; exit 3")
        .status()
        .unwrap();
    println!("alt sürecin çıkış durumu: {durum} (code = {:?})", durum.code());

    let cikti = Command::new("uname").arg("-sr").output().unwrap();
    println!("uname -sr çıktısı yakalandı: {:?}", String::from_utf8_lossy(&cikti.stdout).trim());

    match Command::new("olmayan-program").status() {
        Ok(d) => println!("beklenmedik: {d}"),
        Err(e) => println!("başlatılamadı: {e}"),
    }
}
