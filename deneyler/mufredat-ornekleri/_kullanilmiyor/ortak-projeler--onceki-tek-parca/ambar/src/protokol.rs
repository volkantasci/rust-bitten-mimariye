//! Metin protokolü: her istek ve yanıt tek satır.
//!   SET <anahtar> <değer...>  → OK
//!   GET <anahtar>             → DEGER <değer> | YOK
//!   DEL <anahtar>             → OK | YOK
//!   SAY                       → SAYI <n>
#[derive(Debug, PartialEq)]
pub enum Istek {
    Set(String, String),
    Get(String),
    Del(String),
    Say,
}

pub fn ayristir(satir: &str) -> Result<Istek, String> {
    let satir = satir.trim_end_matches(['\r', '\n']);
    let mut parcalar = satir.splitn(3, ' ');
    let komut = parcalar.next().unwrap_or("").to_ascii_uppercase();
    let anahtar = parcalar.next();
    let geri = parcalar.next();
    match (komut.as_str(), anahtar, geri) {
        ("SET", Some(a), Some(d)) if !a.is_empty() => Ok(Istek::Set(a.into(), d.into())),
        ("GET", Some(a), None) => Ok(Istek::Get(a.into())),
        ("DEL", Some(a), None) => Ok(Istek::Del(a.into())),
        ("SAY", None, None) => Ok(Istek::Say),
        ("SET" | "GET" | "DEL", _, _) => Err(format!("{komut}: eksik ya da fazla argüman")),
        _ => Err(format!("bilinmeyen komut: {satir:?}")),
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    #[test]
    fn degerde_bosluk_olabilir() {
        assert_eq!(ayristir("SET selam merhaba dünya\n"), Ok(Istek::Set("selam".into(), "merhaba dünya".into())));
    }
    #[test]
    fn bilinmeyen_komut() {
        assert!(ayristir("FLUSHALL").is_err());
    }
}
