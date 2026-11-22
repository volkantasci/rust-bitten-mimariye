use std::fmt;

// --- Alan (domain) katmanı: iş kuralları, altyapıdan habersiz
mod alan {
    #[derive(Debug, PartialEq)]
    pub enum AlanHatasi {
        YetersizStok { urun: String, istenen: u32, mevcut: u32 },
        GecersizAdet,
    }
    pub fn siparis_ver(stok: u32, urun: &str, adet: u32) -> Result<u32, AlanHatasi> {
        if adet == 0 {
            return Err(AlanHatasi::GecersizAdet);
        }
        if adet > stok {
            return Err(AlanHatasi::YetersizStok { urun: urun.into(), istenen: adet, mevcut: stok });
        }
        Ok(stok - adet)
    }
}

// --- Altyapı katmanı: dosya, ağ, veritabanı hataları
mod altyapi {
    pub fn stok_oku(urun: &str) -> std::io::Result<u32> {
        let icerik = std::fs::read_to_string(format!("stok/{urun}.txt"))?;
        icerik.trim().parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

// --- Uygulama sınırı: her hata türü kullanıcıya uygun bir yanıta çevrilir
#[derive(Debug)]
enum UygulamaHatasi {
    Alan(alan::AlanHatasi),
    Altyapi(std::io::Error),
}
impl From<alan::AlanHatasi> for UygulamaHatasi {
    fn from(h: alan::AlanHatasi) -> Self { UygulamaHatasi::Alan(h) }
}
impl From<std::io::Error> for UygulamaHatasi {
    fn from(h: std::io::Error) -> Self { UygulamaHatasi::Altyapi(h) }
}
impl fmt::Display for UygulamaHatasi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UygulamaHatasi::Alan(alan::AlanHatasi::YetersizStok { urun, istenen, mevcut }) =>
                write!(f, "400 {urun}: {istenen} istendi, stokta {mevcut} var"),
            UygulamaHatasi::Alan(alan::AlanHatasi::GecersizAdet) => write!(f, "400 adet en az 1 olmalı"),
            UygulamaHatasi::Altyapi(_) => write!(f, "500 geçici bir sorun oluştu, lütfen tekrar deneyin"),
        }
    }
}

fn istek(urun: &str, adet: u32) -> Result<String, UygulamaHatasi> {
    let stok = altyapi::stok_oku(urun)?; // io::Error → UygulamaHatasi (From)
    let kalan = alan::siparis_ver(stok, urun, adet)?; // AlanHatasi → UygulamaHatasi (From)
    Ok(format!("200 sipariş alındı, kalan stok {kalan}"))
}

fn main() {
    for (urun, adet) in [("defter", 3), ("defter", 50), ("defter", 0), ("silgi", 1)] {
        match istek(urun, adet) {
            Ok(m) => println!("{urun} × {adet}: {m}"),
            Err(h) => {
                println!("{urun} × {adet}: {h}"); // kullanıcıya
                if let UygulamaHatasi::Altyapi(ic) = &h {
                    eprintln!("    [günlük] altyapı hatası: {ic}"); // operatöre: iç ayrıntı
                }
            }
        }
    }
}
