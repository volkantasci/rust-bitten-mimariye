//! Diskteki günlük biçimi (117. adım). Her kayıt:
//!   [anahtar_uz: u32][deger_uz: u32][anahtar baytları][değer baytları]
//! Sayılar little-endian (küçüğü başta) yazılır. deger_uz == SILME ise kayıt bir silme kaydıdır.
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Silme kaydı (tombstone) işareti: değer uzunluğu alanında u32::MAX.
pub const SILME: u32 = u32::MAX;
/// Başlık: iki u32 uzunluk alanı.
pub const BASLIK: usize = 8;

#[derive(Debug, Clone, PartialEq)]
pub struct Kayit {
    pub anahtar: String,
    pub deger: Option<String>, // None: silme kaydı
}

/// Kaydı diske yazılacak baytlara çevirir.
pub fn kodla(k: &Kayit) -> Vec<u8> {
    let deger = k.deger.as_deref().unwrap_or("");
    let deger_uz = if k.deger.is_some() { deger.len() as u32 } else { SILME };
    let mut v = Vec::with_capacity(BASLIK + k.anahtar.len() + deger.len());
    v.extend_from_slice(&(k.anahtar.len() as u32).to_le_bytes()); // uzunluk bayt sayısıdır, karakter değil
    v.extend_from_slice(&deger_uz.to_le_bytes());
    v.extend_from_slice(k.anahtar.as_bytes());
    v.extend_from_slice(deger.as_bytes());
    v
}

fn metin(baytlar: &[u8]) -> io::Result<String> {
    String::from_utf8(baytlar.to_vec()).map_err(|h| io::Error::new(io::ErrorKind::InvalidData, h))
}

/// Günlüğü baştan sona okur; kayıtları yazıldıkları sırayla döndürür (yeniden oynatma için).
/// Bu adımda dosyanın sağlam olduğu varsayılıyor: yarım kalmış bir kayıtta dilim sınırı
/// denetimi panik verir. Bozulmaya dayanıklı okuma 118. adımda.
pub fn oku(yol: &Path) -> io::Result<Vec<Kayit>> {
    let mut veri = Vec::new();
    if yol.exists() {
        File::open(yol)?.read_to_end(&mut veri)?;
    }
    let mut kayitlar = Vec::new();
    let mut i = 0;
    while i < veri.len() {
        let sayi = |o: usize| u32::from_le_bytes(veri[i + o..i + o + 4].try_into().unwrap());
        let (a_uz, d_uz) = (sayi(0) as usize, sayi(4));
        let d_gercek = if d_uz == SILME { 0 } else { d_uz as usize };
        let anahtar = metin(&veri[i + BASLIK..i + BASLIK + a_uz])?;
        let deger = if d_uz == SILME {
            None
        } else {
            Some(metin(&veri[i + BASLIK + a_uz..i + BASLIK + a_uz + d_gercek])?)
        };
        kayitlar.push(Kayit { anahtar, deger });
        i += BASLIK + a_uz + d_gercek;
    }
    Ok(kayitlar)
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn silme_kaydi_uzunluk_alaninda_isaretlenir() {
        let b = kodla(&Kayit { anahtar: "yil".into(), deger: None });
        assert_eq!(b, [3, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, b'y', b'i', b'l']);
    }

    #[test]
    fn uzunluk_bayt_sayar_karakter_degil() {
        let b = kodla(&Kayit { anahtar: "şehir".into(), deger: Some("İzmir".into()) });
        assert_eq!(&b[..8], &[6, 0, 0, 0, 6, 0, 0, 0]); // 5 karakter, 6 bayt (UTF-8)
    }
}
