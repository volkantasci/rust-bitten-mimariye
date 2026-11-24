//! Komut satırı (121. adım, bitmiş hâl). Yerel komutlar her çağrıda motoru açar (günlüğü doğrulayarak
//! yeniden oynatır); `sunucu` motoru bir kez açıp TCP üzerinden hizmet verir.
//! `yuk` ve `yazici` sunucuyu sınamak için istemcilerdir.
use ambar::motor::{self, FsyncKipi, Motor};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::exit;
use std::sync::Arc;
use std::time::Instant;

const KULLANIM: &str = "kullanım: ambar yaz DİZİN ANAHTAR DEĞER | oku DİZİN ANAHTAR | sil DİZİN ANAHTAR \
| listele DİZİN | durum DİZİN | doldur DİZİN N | kirp DİZİN | crc METİN \
| sunucu DİZİN ADRES her|toplu|yok | istemci ADRES İSTEK... | yuk ADRES İSTEMCİ ADET | yazici ADRES N";

fn hata(h: impl std::fmt::Display) -> ! {
    eprintln!("hata: {h}");
    exit(1)
}

fn ac(dizin: &str, kip: FsyncKipi) -> (Arc<Motor>, motor::Rapor) {
    Motor::ac(Path::new(dizin), kip).unwrap_or_else(|h| hata(h))
}

/// Bir bağlantı üzerinden istek-yanıt: isteği yaz, tek satırlık yanıtı oku.
struct Baglanti {
    akis: TcpStream,
    okuyucu: BufReader<TcpStream>,
}

impl Baglanti {
    fn kur(adres: &str) -> std::io::Result<Baglanti> {
        let akis = TcpStream::connect(adres)?;
        akis.set_nodelay(true)?;
        let okuyucu = BufReader::new(akis.try_clone()?);
        Ok(Baglanti { akis, okuyucu })
    }

    fn sor(&mut self, istek: &str) -> std::io::Result<String> {
        writeln!(self.akis, "{istek}")?;
        let mut yanit = String::new();
        if self.okuyucu.read_line(&mut yanit)? == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "bağlantı kapandı"));
        }
        Ok(yanit.trim_end().to_string())
    }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = a.iter().map(String::as_str).collect();
    match a[..] {
        ["yaz", dizin, anahtar, deger] => ac(dizin, FsyncKipi::Her).0.yaz(anahtar, deger).unwrap_or_else(|h| hata(h)),
        ["oku", dizin, anahtar] => match ac(dizin, FsyncKipi::Her).0.oku(anahtar) {
            Some(d) => println!("{d}"),
            None => {
                println!("yok");
                exit(1)
            }
        },
        ["sil", dizin, anahtar] => {
            if !ac(dizin, FsyncKipi::Her).0.sil(anahtar).unwrap_or_else(|h| hata(h)) {
                println!("yok");
                exit(1)
            }
        }
        ["listele", dizin] => {
            let (m, _) = ac(dizin, FsyncKipi::Her);
            let v = m.anahtarlar();
            println!("günlük: {} kayıt; canlı anahtar: {}", m.kayit_sayisi(), v.len());
            for (k, d) in v {
                println!("  {k} = {d}");
            }
        }
        ["durum", dizin] => {
            let (m, r) = ac(dizin, FsyncKipi::Her);
            println!("geçerli kayıt: {} ({} bayt); kırpılan kuyruk: {} bayt", r.kayit, r.gecerli_bayt, r.atilan_bayt);
            if let Some(n) = r.neden {
                println!("kurtarma notu: {n}");
            }
            println!("canlı anahtar: {}", m.anahtar_sayisi());
        }
        ["doldur", dizin, n] => {
            let n: usize = n.parse().unwrap_or_else(|h| hata(h));
            let (m, _) = ac(dizin, FsyncKipi::Yok); // toplu doldurma: fsync yok
            for i in 0..n {
                m.yaz(&format!("anahtar{i:05}"), &format!("değer-{i}")).unwrap_or_else(|h| hata(h));
            }
            println!("{n} kayıt yazıldı");
        }
        ["kirp", dizin] => {
            let (atilan, kalan) = motor::kirp(Path::new(dizin)).unwrap_or_else(|h| hata(h));
            println!("{atilan} bayt atıldı; kalan kayıt: {kalan}");
        }
        ["crc", metin] => {
            let c = ambar::crc::crc32(metin.as_bytes());
            println!("{c:08x} ({c})");
        }
        ["sunucu", dizin, adres, kip] => {
            let kip = match kip {
                "her" => FsyncKipi::Her,
                "toplu" => FsyncKipi::Toplu,
                "yok" => FsyncKipi::Yok,
                _ => hata("fsync kipi: her | toplu | yok"),
            };
            let (m, r) = ac(dizin, kip);
            eprintln!("günlükten {} kayıt okundu; fsync kipi: {kip:?}", r.kayit);
            ambar::sunucu::calistir(adres, m).unwrap_or_else(|h| hata(h));
        }
        ["istemci", adres, ref istekler @ ..] => {
            let mut b = Baglanti::kur(adres).unwrap_or_else(|h| hata(h));
            for istek in istekler {
                let yanit = b.sor(istek).unwrap_or_else(|h| hata(h));
                println!("> {istek}");
                println!("< {yanit}");
            }
        }
        ["yuk", adres, istemci, adet] => {
            // İSTEMCİ ayrı bağlantı aynı anda, her biri ADET farklı anahtar yazar;
            // her isteğin gidiş-dönüş süresi (gecikme) ölçülür.
            let istemci: usize = istemci.parse().unwrap_or_else(|h| hata(h));
            let adet: usize = adet.parse().unwrap_or_else(|h| hata(h));
            let t = Instant::now();
            let mut gecikmeler: Vec<f64> = std::thread::scope(|s| {
                let isler: Vec<_> = (0..istemci)
                    .map(|c| {
                        s.spawn(move || {
                            let mut b = Baglanti::kur(adres).unwrap();
                            let mut v = Vec::with_capacity(adet);
                            for i in 0..adet {
                                let t0 = Instant::now();
                                assert_eq!(b.sor(&format!("yaz i{c}-{i} değer")).unwrap(), "tamam");
                                v.push(t0.elapsed().as_secs_f64() * 1e3);
                            }
                            v
                        })
                    })
                    .collect();
                isler.into_iter().flat_map(|i| i.join().unwrap()).collect()
            });
            let sn = t.elapsed().as_secs_f64();
            gecikmeler.sort_by(f64::total_cmp);
            let yuzdelik = |p: f64| gecikmeler[((gecikmeler.len() - 1) as f64 * p).round() as usize];
            let toplam = istemci * adet;
            let mut b = Baglanti::kur(adres).unwrap_or_else(|h| hata(h));
            let say = b.sor("say").unwrap_or_else(|h| hata(h));
            let ist = b.sor("istatistik").unwrap_or_else(|h| hata(h));
            println!("{istemci} istemci × {adet} yazma = {toplam} yazma, {sn:.2} s, {:.0} yazma/s", toplam as f64 / sn);
            let ortalama = gecikmeler.iter().sum::<f64>() / gecikmeler.len() as f64;
            println!(
                "  gecikme: ortalama {ortalama:.2} ms, ortanca {:.2} ms, %99 {:.2} ms; sunucu: {say}, {ist}",
                yuzdelik(0.5),
                yuzdelik(0.99)
            );
        }
        ["yazici", adres, n] => {
            // Çökme deneyi: "tamam" yanıtı alınan (onaylanan) yazmaları say; bağlantı kopunca dur.
            let n: usize = n.parse().unwrap_or_else(|h| hata(h));
            let mut b = Baglanti::kur(adres).unwrap_or_else(|h| hata(h));
            let mut onaylanan = 0;
            for i in 0..n {
                match b.sor(&format!("yaz k{i:07} v{i}")) {
                    Ok(y) if y == "tamam" => onaylanan += 1,
                    _ => break,
                }
            }
            println!("onaylanan yazma: {onaylanan}");
        }
        _ => {
            eprintln!("{KULLANIM}");
            exit(2)
        }
    }
}
