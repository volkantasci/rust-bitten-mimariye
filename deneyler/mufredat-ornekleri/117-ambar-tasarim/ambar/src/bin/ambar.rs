//! Komut satırı (117. adım). Her çağrı yeni bir süreçtir: motor açılırken günlüğü
//! baştan okuyup indeksi yeniden kurar, işini yapar ve çıkar.
use ambar::motor::Motor;
use std::path::Path;
use std::process::exit;

const KULLANIM: &str = "kullanım: ambar yaz DİZİN ANAHTAR DEĞER | oku DİZİN ANAHTAR | sil DİZİN ANAHTAR | listele DİZİN";

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = a.iter().map(String::as_str).collect();
    let ac = |dizin: &str| Motor::ac(Path::new(dizin)).unwrap_or_else(|h| {
        eprintln!("hata: {h}");
        exit(1)
    });
    match a[..] {
        ["yaz", dizin, anahtar, deger] => ac(dizin).yaz(anahtar, deger).unwrap(),
        ["oku", dizin, anahtar] => match ac(dizin).oku(anahtar) {
            Some(d) => println!("{d}"),
            None => {
                println!("yok");
                exit(1)
            }
        },
        ["sil", dizin, anahtar] => {
            if !ac(dizin).sil(anahtar).unwrap() {
                println!("yok");
                exit(1)
            }
        }
        ["listele", dizin] => {
            let m = ac(dizin);
            let bayt = std::fs::metadata(Path::new(dizin).join("gunluk.ambar")).unwrap().len();
            let v = m.anahtarlar();
            println!("günlük: {} kayıt, {bayt} bayt; canlı anahtar: {}", m.kayit_sayisi(), v.len());
            for (k, d) in v {
                println!("  {k} = {d}");
            }
        }
        _ => {
            eprintln!("{KULLANIM}");
            exit(2)
        }
    }
}
