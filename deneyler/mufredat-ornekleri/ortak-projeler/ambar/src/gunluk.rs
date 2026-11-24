//! Diskteki günlük biçimi (118. adım). Her kayıt:
//!   [başlık_crc: u32][anahtar_uz: u32][deger_uz: u32][veri_crc: u32][anahtar baytları][değer baytları]
//! Sayılar little-endian (küçüğü başta). İki ayrı CRC var:
//!   - başlık CRC'si iki uzunluk alanını korur: okuyucu bir kaydın nerede bittiğini ancak
//!     uzunluğu doğrulandıktan sonra hesaplar;
//!   - veri CRC'si anahtar ve değer baytlarını korur.
//! deger_uz == SILME ise kayıt bir silme kaydıdır.
use crate::crc::crc32;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Silme kaydı (tombstone) işareti: değer uzunluğu alanında u32::MAX.
pub const SILME: u32 = u32::MAX;
/// Başlık: başlık CRC'si + iki uzunluk alanı.
pub const BASLIK: usize = 12;
/// Başlıktan sonra gelen veri CRC'si.
pub const VERI_CRC: usize = 4;
/// Yazılabilecek en uzun anahtar ve değer (yazarken denetlenir).
pub const AZAMI_ANAHTAR: usize = 1024;
pub const AZAMI_DEGER: usize = 1024 * 1024;

#[derive(Debug, Clone, PartialEq)]
pub struct Kayit {
    pub anahtar: String,
    pub deger: Option<String>, // None: silme kaydı
}

/// Kaydı diske yazılacak baytlara çevirir.
pub fn kodla(k: &Kayit) -> Vec<u8> {
    let deger = k.deger.as_deref().unwrap_or("");
    let deger_uz = if k.deger.is_some() { deger.len() as u32 } else { SILME };
    let mut uzunluklar = Vec::with_capacity(8);
    uzunluklar.extend_from_slice(&(k.anahtar.len() as u32).to_le_bytes());
    uzunluklar.extend_from_slice(&deger_uz.to_le_bytes());
    let mut veri = Vec::with_capacity(k.anahtar.len() + deger.len());
    veri.extend_from_slice(k.anahtar.as_bytes());
    veri.extend_from_slice(deger.as_bytes());
    let mut v = Vec::with_capacity(BASLIK + VERI_CRC + veri.len());
    v.extend_from_slice(&crc32(&uzunluklar).to_le_bytes());
    v.extend_from_slice(&uzunluklar);
    v.extend_from_slice(&crc32(&veri).to_le_bytes());
    v.extend_from_slice(&veri);
    v
}

/// Okumanın neden durduğu.
#[derive(Debug, PartialEq)]
pub enum Durma {
    /// Dosyanın sonunda başlığı bile tamamlanmamış kayıt: yarım yazma.
    YarimBaslik { konum: u64, kalan: u64 },
    /// Başlık sağlam (uzunluklar doğrulandı) ama kayıt dosyanın sonunu aşıyor: yarım yazma.
    YarimKayit { konum: u64, kalan: u64, gereken: u64 },
    /// Başlık CRC'si tutmuyor: uzunluk alanları bozuk, kaydın nerede bittiği bilinmiyor.
    BozukBaslik { konum: u64, kalan: u64 },
    /// Veri CRC'si tutmuyor (kayıt tam boyda; arkasında `sonrasi` bayt daha var).
    BozukKayit { konum: u64, sonrasi: u64 },
}

impl Durma {
    /// Kendiliğinden kırpılabilecek tek durum: boyu dosyanın sonunu aştığı kanıtlanan son kayıt.
    /// Bozulma (başlıkta ya da veride, sonda ya da ortada) hiçbir zaman kendiliğinden kırpılmaz.
    pub fn kuyrukta_mi(&self) -> bool {
        matches!(self, Durma::YarimBaslik { .. } | Durma::YarimKayit { .. })
    }
}

impl fmt::Display for Durma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Durma::YarimBaslik { konum, kalan } => write!(f, "{konum}. baytta yarım başlık ({kalan}/{BASLIK} bayt)"),
            Durma::YarimKayit { konum, kalan, gereken } => {
                write!(f, "{konum}. baytta yarım kayıt ({kalan}/{gereken} bayt; başlık sağlam)")
            }
            Durma::BozukBaslik { konum, kalan } => {
                write!(f, "{konum}. baytta başlık CRC'si tutmuyor (uzunluk alanları bozuk); dosyada kalan {kalan} bayt")
            }
            Durma::BozukKayit { konum, sonrasi } => {
                write!(f, "{konum}. baytta veri CRC'si tutmuyor; arkasında {sonrasi} bayt daha var")
            }
        }
    }
}

/// Günlüğü okumanın sonucu: sağlam önekteki kayıtlar ve (varsa) durma nedeni.
#[derive(Debug)]
pub struct Okuma {
    pub kayitlar: Vec<Kayit>,
    pub gecerli_bayt: u64, // baştan itibaren sağlam önek
    pub toplam_bayt: u64,
    pub durma: Option<Durma>,
}

fn metin(baytlar: &[u8]) -> io::Result<String> {
    String::from_utf8(baytlar.to_vec()).map_err(|h| io::Error::new(io::ErrorKind::InvalidData, h))
}

/// Günlüğü baştan okur; ilk yarım ya da bozuk kayıtta durur. Dosyayı değiştirmez.
pub fn oku(yol: &Path) -> io::Result<Okuma> {
    let mut veri = Vec::new();
    if yol.exists() {
        File::open(yol)?.read_to_end(&mut veri)?;
    }
    let mut kayitlar = Vec::new();
    let mut i = 0usize;
    let durma = loop {
        let kalan = veri.len() - i;
        if kalan == 0 {
            break None;
        }
        let konum = i as u64;
        if kalan < BASLIK {
            break Some(Durma::YarimBaslik { konum, kalan: kalan as u64 });
        }
        let sayi = |o: usize| u32::from_le_bytes(veri[i + o..i + o + 4].try_into().unwrap());
        // Önce uzunlukları doğrula: bozuk bir uzunlukla kaydın sonu yanlış hesaplanırdı
        if crc32(&veri[i + 4..i + BASLIK]) != sayi(0) {
            break Some(Durma::BozukBaslik { konum, kalan: kalan as u64 });
        }
        let (a_uz, d_uz) = (sayi(4) as usize, sayi(8));
        let d_gercek = if d_uz == SILME { 0 } else { d_uz as usize };
        let toplam = BASLIK + VERI_CRC + a_uz + d_gercek;
        if kalan < toplam {
            break Some(Durma::YarimKayit { konum, kalan: kalan as u64, gereken: toplam as u64 });
        }
        let govde = &veri[i + BASLIK + VERI_CRC..i + toplam];
        if crc32(govde) != sayi(BASLIK) {
            break Some(Durma::BozukKayit { konum, sonrasi: (kalan - toplam) as u64 });
        }
        let anahtar = metin(&govde[..a_uz])?;
        let deger = if d_uz == SILME { None } else { Some(metin(&govde[a_uz..])?) };
        kayitlar.push(Kayit { anahtar, deger });
        i += toplam;
    };
    Ok(Okuma { kayitlar, gecerli_bayt: i as u64, toplam_bayt: veri.len() as u64, durma })
}

#[cfg(test)]
mod testler {
    use super::*;

    fn gecici(ad: &str, baytlar: &[u8]) -> std::path::PathBuf {
        let yol = std::env::temp_dir().join(format!("ambar-{ad}-{}", std::process::id()));
        std::fs::write(&yol, baytlar).unwrap();
        yol
    }

    // Üç kayıt; her biri 16 + 1 + 1 = 18 bayt
    fn uc_kayit() -> Vec<u8> {
        ["a", "b", "c"]
            .iter()
            .flat_map(|k| kodla(&Kayit { anahtar: k.to_string(), deger: Some("1".into()) }))
            .collect()
    }

    #[test]
    fn kuyruktaki_yarim_kayit_kirpilabilir() {
        let mut v = uc_kayit();
        v.truncate(v.len() - 2);
        let o = oku(&gecici("kuyruk", &v)).unwrap();
        assert_eq!(o.kayitlar.len(), 2);
        assert!(o.durma.unwrap().kuyrukta_mi());
    }

    #[test]
    fn ortadaki_veri_bozulmasi_kuyruk_sayilmaz() {
        let mut v = uc_kayit();
        v[18 + 16] ^= 0x20; // ikinci kaydın anahtar baytı: 'b' → 'B'
        let o = oku(&gecici("orta", &v)).unwrap();
        assert_eq!(o.kayitlar.len(), 1);
        assert_eq!(o.durma, Some(Durma::BozukKayit { konum: 18, sonrasi: 18 }));
    }

    #[test]
    fn uzunluk_bozulmasi_baslik_crcsine_takilir() {
        let mut v = uc_kayit();
        v[18 + 9] = 0x58; // ikinci kaydın değer uzunluğunun ikinci baytı: 1 → 22 529
        let o = oku(&gecici("uzunluk", &v)).unwrap();
        assert!(matches!(o.durma, Some(Durma::BozukBaslik { konum: 18, .. })));
    }

    #[test]
    fn ortadaki_kaydin_hicbir_tek_bit_bozulmasi_kirpilmaz() {
        // İkinci kaydın 18 baytının her bitini tek tek çevir: 144 bozulmanın hiçbiri
        // "kendiliğinden kırpılabilir" sayılmamalı ve ilk kayıt her seferinde okunmalı.
        let asil = uc_kayit();
        for bit in 18 * 8..36 * 8 {
            let mut v = asil.clone();
            v[bit / 8] ^= 1 << (bit % 8);
            let o = oku(&gecici("bit", &v)).unwrap();
            let d = o.durma.expect("bozulma yakalanmalı");
            assert!(!d.kuyrukta_mi(), "{bit}. bit: {d}");
            assert_eq!(o.kayitlar.len(), 1, "{bit}. bit");
        }
    }
}
