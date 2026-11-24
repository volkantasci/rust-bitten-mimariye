//! Depolama motoru (117. adım): her değişiklik günlüğün sonuna bir kayıt olarak eklenir,
//! her anahtarın son değeri bellekteki indekste (HashMap) tutulur. Tek iş parçacıklı.
use crate::gunluk::{self, Kayit};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub struct Motor {
    indeks: HashMap<String, String>, // anahtar → son değer
    gunluk: File,                    // yalnızca sona ekleme (O_APPEND) kipinde açık
    kayit_sayisi: usize,             // günlükteki bütün kayıtlar: canlı + eskimiş
}

/// Bir kaydı indekse uygular: değer varsa yazar, silme kaydıysa anahtarı kaldırır.
fn uygula(indeks: &mut HashMap<String, String>, k: Kayit) {
    match k.deger {
        Some(d) => {
            indeks.insert(k.anahtar, d);
        }
        None => {
            indeks.remove(&k.anahtar);
        }
    }
}

impl Motor {
    /// Günlüğü baştan yeniden oynatarak (replay) indeksi kurar.
    pub fn ac(dizin: &Path) -> io::Result<Motor> {
        std::fs::create_dir_all(dizin)?;
        let yol = dizin.join("gunluk.ambar");
        let kayitlar = gunluk::oku(&yol)?;
        let kayit_sayisi = kayitlar.len();
        let mut indeks = HashMap::new();
        for k in kayitlar {
            uygula(&mut indeks, k);
        }
        let gunluk = OpenOptions::new().create(true).append(true).open(&yol)?;
        Ok(Motor { indeks, gunluk, kayit_sayisi })
    }

    fn ekle(&mut self, k: Kayit) -> io::Result<()> {
        self.gunluk.write_all(&gunluk::kodla(&k))?; // önce günlük...
        self.kayit_sayisi += 1;
        uygula(&mut self.indeks, k); // ...sonra indeks
        Ok(())
    }

    pub fn yaz(&mut self, anahtar: &str, deger: &str) -> io::Result<()> {
        self.ekle(Kayit { anahtar: anahtar.into(), deger: Some(deger.into()) })
    }

    /// Anahtar varsa silme kaydı ekler; yoksa günlüğe bir şey yazmaz.
    pub fn sil(&mut self, anahtar: &str) -> io::Result<bool> {
        if !self.indeks.contains_key(anahtar) {
            return Ok(false);
        }
        self.ekle(Kayit { anahtar: anahtar.into(), deger: None })?;
        Ok(true)
    }

    pub fn oku(&self, anahtar: &str) -> Option<&str> {
        self.indeks.get(anahtar).map(String::as_str)
    }

    /// Canlı anahtarlar, sıralı.
    pub fn anahtarlar(&self) -> Vec<(&str, &str)> {
        let mut v: Vec<_> = self.indeks.iter().map(|(a, d)| (a.as_str(), d.as_str())).collect();
        v.sort();
        v
    }

    pub fn kayit_sayisi(&self) -> usize {
        self.kayit_sayisi
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn yeniden_acinca_ayni_durum_kurulur() {
        let dizin = std::env::temp_dir().join(format!("ambar-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dizin);
        {
            let mut m = Motor::ac(&dizin).unwrap();
            m.yaz("dil", "rust").unwrap();
            m.yaz("yil", "2015").unwrap();
            m.yaz("dil", "Rust").unwrap();
            assert!(m.sil("yil").unwrap());
            assert!(!m.sil("yok").unwrap());
        } // Motor düşürülür (drop): bellekteki indeks gider, günlük kalır
        let m = Motor::ac(&dizin).unwrap();
        assert_eq!(m.anahtarlar(), [("dil", "Rust")]);
        assert_eq!(m.kayit_sayisi(), 4);
        std::fs::remove_dir_all(&dizin).unwrap();
    }
}
