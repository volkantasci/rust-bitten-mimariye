//! Metin protokolü (119. adım): her istek ve her yanıt tek bir satırdır; satırı "\n" bitirir.
//!   yaz ANAHTAR DEĞER...  → tamam
//!   oku ANAHTAR           → değer DEĞER | yok
//!   sil ANAHTAR           → tamam | yok
//!   say                   → sayı N
//! Hatalı istek → "hata: açıklama". Komut adları ASCII büyük/küçük harf duyarsızdır.
//! Çerçeveyi satır sonu belirler: bu yüzden bir değer "\n" içeremez.
#[derive(Debug, PartialEq)]
pub enum Istek {
    Yaz(String, String),
    Oku(String),
    Sil(String),
    Say,
}

pub fn ayristir(satir: &str) -> Result<Istek, String> {
    let satir = satir.trim_end_matches(['\r', '\n']);
    let mut parcalar = satir.splitn(3, ' ');
    let komut = parcalar.next().unwrap_or("").to_ascii_lowercase();
    let (anahtar, geri) = (parcalar.next(), parcalar.next());
    match (komut.as_str(), anahtar, geri) {
        ("yaz", Some(a), Some(d)) if !a.is_empty() => Ok(Istek::Yaz(a.into(), d.into())),
        ("oku", Some(a), None) if !a.is_empty() => Ok(Istek::Oku(a.into())),
        ("sil", Some(a), None) if !a.is_empty() => Ok(Istek::Sil(a.into())),
        ("say", None, None) => Ok(Istek::Say),
        ("yaz" | "oku" | "sil" | "say", _, _) => Err(format!("{komut}: eksik ya da fazla argüman")),
        _ => Err(format!("bilinmeyen komut: {satir:?}")),
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn degerde_bosluk_olabilir() {
        assert_eq!(ayristir("yaz selam merhaba dünya\n"), Ok(Istek::Yaz("selam".into(), "merhaba dünya".into())));
    }

    #[test]
    fn komut_adi_ascii_harf_duyarsiz() {
        assert_eq!(ayristir("OKU dil"), Ok(Istek::Oku("dil".into())));
        // Türkçe büyük İ ASCII değildir: "SİL" komut olarak tanınmaz
        assert!(ayristir("SİL dil").is_err());
    }

    #[test]
    fn eksik_arguman_ve_bilinmeyen_komut() {
        assert!(ayristir("yaz yalniz_anahtar").is_err());
        assert!(ayristir("uçur hepsini").is_err());
    }
}
