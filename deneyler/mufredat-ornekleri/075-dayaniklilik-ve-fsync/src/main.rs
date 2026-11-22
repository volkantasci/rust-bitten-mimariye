use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Instant;

fn main() {
    let kip = std::env::args().nth(1).unwrap_or_default();
    let dizin = std::env::args().nth(2).expect("dizin");
    let dizin = Path::new(&dizin);
    if kip == "atomik" {
        // Atomik değiştirme kalıbı: geçici dosyaya yaz, diske indir, adını değiştir
        let (gecici, hedef) = (dizin.join("ayar.json.gecici"), dizin.join("ayar.json"));
        let mut f = File::create(&gecici).unwrap();
        f.write_all(b"{\"surum\": 2}\n").unwrap();
        f.sync_all().unwrap(); // fsync: veri ve meta veri kalıcı depolamaya
        fs::rename(&gecici, &hedef).unwrap(); // okuyan ya eski ya yeni dosyayı görür, yarımını asla
        File::open(dizin).unwrap().sync_all().unwrap(); // dizin girdisini de kalıcı yap
        return;
    }
    let yol = dizin.join("gunluk.log");
    for (ad, her_kayitta) in [("her kayıtta fsync", true), ("sonda tek fsync", false)] {
        let _ = fs::remove_file(&yol);
        let mut f = OpenOptions::new().create(true).append(true).open(&yol).unwrap();
        let t = Instant::now();
        for i in 0..200 {
            writeln!(f, "kayıt {i}").unwrap();
            if her_kayitta {
                f.sync_data().unwrap();
            }
        }
        f.sync_data().unwrap();
        let ms = t.elapsed().as_secs_f64() * 1e3;
        println!("{ad:<18}: 200 kayıt {ms:>8.2} ms ({:.3} ms/kayıt)", ms / 200.0);
    }
}
