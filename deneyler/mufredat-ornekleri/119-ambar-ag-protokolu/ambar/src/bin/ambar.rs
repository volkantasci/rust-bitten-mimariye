//! Komut satırı (119. adım). Yerel komutlar her çağrıda motoru açar (günlüğü doğrulayarak
//! yeniden oynatır); `sunucu` motoru bir kez açıp TCP üzerinden hizmet verir.
use ambar::motor::{self, Motor};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::exit;

const KULLANIM: &str = "kullanım: ambar yaz DİZİN ANAHTAR DEĞER | oku DİZİN ANAHTAR | sil DİZİN ANAHTAR \
| listele DİZİN | durum DİZİN | doldur DİZİN N | kirp DİZİN | crc METİN \
| sunucu DİZİN ADRES | istemci ADRES İSTEK...";

fn hata(h: impl std::fmt::Display) -> ! {
    eprintln!("hata: {h}");
    exit(1)
}

fn ac(dizin: &str) -> Motor {
    Motor::ac(Path::new(dizin)).unwrap_or_else(|h| hata(h)).0
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = a.iter().map(String::as_str).collect();
    match a[..] {
        ["yaz", dizin, anahtar, deger] => ac(dizin).yaz(anahtar, deger).unwrap_or_else(|h| hata(h)),
        ["oku", dizin, anahtar] => match ac(dizin).oku(anahtar) {
            Some(d) => println!("{d}"),
            None => {
                println!("yok");
                exit(1)
            }
        },
        ["sil", dizin, anahtar] => {
            if !ac(dizin).sil(anahtar).unwrap_or_else(|h| hata(h)) {
                println!("yok");
                exit(1)
            }
        }
        ["listele", dizin] => {
            let m = ac(dizin);
            let v = m.anahtarlar();
            println!("günlük: {} kayıt; canlı anahtar: {}", m.kayit_sayisi(), v.len());
            for (k, d) in v {
                println!("  {k} = {d}");
            }
        }
        ["durum", dizin] => {
            let (m, r) = Motor::ac(Path::new(dizin)).unwrap_or_else(|h| hata(h));
            println!("geçerli kayıt: {} ({} bayt); kırpılan kuyruk: {} bayt", r.kayit, r.gecerli_bayt, r.atilan_bayt);
            if let Some(n) = r.neden {
                println!("kurtarma notu: {n}");
            }
            println!("canlı anahtar: {}", m.anahtarlar().len());
        }
        ["doldur", dizin, n] => {
            let n: usize = n.parse().unwrap_or_else(|h| hata(h));
            let mut m = ac(dizin);
            for i in 0..n {
                m.yaz(&format!("anahtar{i:05}"), &format!("değer-{i}")).unwrap_or_else(|h| hata(h));
            }
            println!("{n} kayıt yazıldı");
        }
        ["kirp", dizin] => {
            let (atilan, kalan) = motor::kirp(Path::new(dizin)).unwrap_or_else(|h| hata(h));
            println!("{atilan} bayt atıldı; kalan kayıt: {kalan}");
        }
        ["sunucu", dizin, adres] => {
            let (m, r) = Motor::ac(Path::new(dizin)).unwrap_or_else(|h| hata(h));
            eprintln!("günlükten {} kayıt okundu", r.kayit);
            ambar::sunucu::calistir(adres, m).unwrap_or_else(|h| hata(h));
        }
        ["istemci", adres, ref istekler @ ..] => {
            let mut akis = TcpStream::connect(adres).unwrap_or_else(|h| hata(h));
            let mut okuyucu = BufReader::new(akis.try_clone().unwrap());
            for istek in istekler {
                writeln!(akis, "{istek}").unwrap_or_else(|h| hata(h));
                let mut yanit = String::new();
                okuyucu.read_line(&mut yanit).unwrap_or_else(|h| hata(h));
                println!("> {istek}");
                println!("< {}", yanit.trim_end());
            }
        }
        ["crc", metin] => {
            let c = ambar::crc::crc32(metin.as_bytes());
            println!("{c:08x} ({c})");
        }
        _ => {
            eprintln!("{KULLANIM}");
            exit(2)
        }
    }
}
