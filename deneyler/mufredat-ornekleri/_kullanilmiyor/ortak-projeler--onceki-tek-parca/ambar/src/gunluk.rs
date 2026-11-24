//! Diskteki günlük biçimi ve kurtarma.
//! Kayıt: [crc32: u32][anahtar_uz: u32][deger_uz: u32, MEZAR = silme][anahtar][değer]  (küçük uçlu)
use crate::crc::crc32;
use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use std::path::Path;

pub const MEZAR: u32 = u32::MAX; // silme işareti (tombstone)
const BASLIK: usize = 12;

#[derive(Debug, Clone, PartialEq)]
pub struct Kayit {
    pub anahtar: String,
    pub deger: Option<String>, // None: silme
}

pub fn kodla(k: &Kayit) -> Vec<u8> {
    let deger = k.deger.as_deref().unwrap_or("");
    let mut govde = Vec::with_capacity(8 + k.anahtar.len() + deger.len());
    govde.extend_from_slice(&(k.anahtar.len() as u32).to_le_bytes());
    let uz = if k.deger.is_some() { deger.len() as u32 } else { MEZAR };
    govde.extend_from_slice(&uz.to_le_bytes());
    govde.extend_from_slice(k.anahtar.as_bytes());
    govde.extend_from_slice(deger.as_bytes());
    let mut kayit = crc32(&govde).to_le_bytes().to_vec();
    kayit.extend_from_slice(&govde);
    kayit
}

#[derive(Debug, Default)]
pub struct KurtarmaRaporu {
    pub gecerli_kayit: usize,
    pub gecerli_bayt: u64,
    pub atilan_bayt: u64,
    pub neden: Option<String>,
}

/// Günlüğü baştan okur; ilk yarım ya da bozuk kayıtta durur, dosyayı geçerli öneke kırpar.
pub fn kurtar(yol: &Path) -> io::Result<(Vec<Kayit>, KurtarmaRaporu)> {
    let mut veri = Vec::new();
    if yol.exists() {
        File::open(yol)?.read_to_end(&mut veri)?;
    }
    let mut kayitlar = Vec::new();
    let mut rapor = KurtarmaRaporu::default();
    let mut i = 0usize;
    while i < veri.len() {
        let kalan = veri.len() - i;
        if kalan < BASLIK {
            rapor.neden = Some(format!("{i}. baytta yarım başlık ({kalan} bayt)"));
            break;
        }
        let sayi = |o: usize| u32::from_le_bytes(veri[i + o..i + o + 4].try_into().unwrap());
        let (crc, a_uz, d_uz) = (sayi(0), sayi(4) as usize, sayi(8));
        let d_gercek = if d_uz == MEZAR { 0 } else { d_uz as usize };
        let toplam = BASLIK + a_uz + d_gercek;
        if kalan < toplam {
            rapor.neden = Some(format!("{i}. baytta yarım kayıt ({kalan}/{toplam} bayt)"));
            break;
        }
        if crc32(&veri[i + 4..i + toplam]) != crc {
            rapor.neden = Some(format!("{i}. baytta CRC uyuşmazlığı (bozuk kayıt)"));
            break;
        }
        let anahtar = String::from_utf8_lossy(&veri[i + BASLIK..i + BASLIK + a_uz]).into_owned();
        let deger = (d_uz != MEZAR)
            .then(|| String::from_utf8_lossy(&veri[i + BASLIK + a_uz..i + toplam]).into_owned());
        kayitlar.push(Kayit { anahtar, deger });
        i += toplam;
    }
    rapor.gecerli_kayit = kayitlar.len();
    rapor.gecerli_bayt = i as u64;
    rapor.atilan_bayt = (veri.len() - i) as u64;
    if rapor.atilan_bayt > 0 {
        OpenOptions::new().write(true).open(yol)?.set_len(i as u64)?; // bozuk kuyruğu at
    }
    Ok((kayitlar, rapor))
}
