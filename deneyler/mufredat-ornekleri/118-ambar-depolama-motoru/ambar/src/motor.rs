//! Depolama motoru (118. adım): açılışta günlüğü doğrulayarak yeniden oynatır.
//! Yalnız boyu dosyanın sonunu aştığı kanıtlanan son kaydı kendiliğinden kırpar;
//! CRC'nin yakaladığı her bozulmada (başlıkta ya da veride) açılmayı reddeder, dosyaya dokunmaz.
use crate::gunluk::{self, Kayit};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub struct Motor {
    indeks: HashMap<String, String>, // anahtar → son değer
    gunluk: File,                    // yalnızca sona ekleme (O_APPEND) kipinde açık
    kayit_sayisi: usize,             // günlükteki bütün kayıtlar: canlı + eskimiş
}

/// Açılışta ne bulunduğu.
#[derive(Debug)]
pub struct Rapor {
    pub kayit: usize,
    pub gecerli_bayt: u64,
    pub atilan_bayt: u64, // kırpılan yarım kuyruk
    pub neden: Option<String>,
}

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

fn gunluk_yolu(dizin: &Path) -> PathBuf {
    dizin.join("gunluk.ambar")
}

impl Motor {
    /// Günlüğü doğrulayarak yeniden oynatır (replay) ve indeksi kurar.
    pub fn ac(dizin: &Path) -> io::Result<(Motor, Rapor)> {
        std::fs::create_dir_all(dizin)?;
        let yol = gunluk_yolu(dizin);
        let okuma = gunluk::oku(&yol)?;
        let mut atilan_bayt = 0;
        if let Some(d) = &okuma.durma {
            if !d.kuyrukta_mi() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("günlük bozuk: {d}; dosyaya dokunulmadı"),
                ));
            }
            // Başlığı sağlam ve boyu dosyanın sonunu aşan kayıt yarım yazılmıştır; arkasında başka
            // bayt olamaz, bu yüzden kırpmak sağlam bir kaydı silmez.
            atilan_bayt = okuma.toplam_bayt - okuma.gecerli_bayt;
            OpenOptions::new().write(true).open(&yol)?.set_len(okuma.gecerli_bayt)?;
        }
        let rapor = Rapor {
            kayit: okuma.kayitlar.len(),
            gecerli_bayt: okuma.gecerli_bayt,
            atilan_bayt,
            neden: okuma.durma.map(|d| d.to_string()),
        };
        let mut indeks = HashMap::new();
        for k in okuma.kayitlar {
            uygula(&mut indeks, k);
        }
        let gunluk = OpenOptions::new().create(true).append(true).open(&yol)?;
        Ok((Motor { indeks, gunluk, kayit_sayisi: rapor.kayit }, rapor))
    }

    fn ekle(&mut self, k: Kayit) -> io::Result<()> {
        self.gunluk.write_all(&gunluk::kodla(&k))?; // önce günlük...
        self.kayit_sayisi += 1;
        uygula(&mut self.indeks, k); // ...sonra indeks
        Ok(())
    }

    /// Üst sınır: değerler bellekte tutulur; sınırı aşan bir kayıt yazılmaz.
    pub fn yaz(&mut self, anahtar: &str, deger: &str) -> io::Result<()> {
        if anahtar.len() > gunluk::AZAMI_ANAHTAR || deger.len() > gunluk::AZAMI_DEGER {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("anahtar en çok {} bayt, değer en çok {} bayt olabilir", gunluk::AZAMI_ANAHTAR, gunluk::AZAMI_DEGER),
            ));
        }
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

    pub fn anahtar_sayisi(&self) -> usize {
        self.indeks.len()
    }
}

/// Operatörün kararı: günlüğü sağlam önekine kırpar. Bozulmanın arkasındaki her şey
/// (sağlam kayıtlar dahil) atılır. Dönüş: atılan bayt ve kalan kayıt sayısı.
pub fn kirp(dizin: &Path) -> io::Result<(u64, usize)> {
    let yol = gunluk_yolu(dizin);
    let okuma = gunluk::oku(&yol)?;
    OpenOptions::new().write(true).open(&yol)?.set_len(okuma.gecerli_bayt)?;
    Ok((okuma.toplam_bayt - okuma.gecerli_bayt, okuma.kayitlar.len()))
}

#[cfg(test)]
mod testler {
    use super::*;

    fn dizin(ad: &str) -> PathBuf {
        let k = std::env::temp_dir().join(format!("ambar-{ad}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&k);
        k
    }

    #[test]
    fn yeniden_acinca_ayni_durum_kurulur() {
        let k = dizin("durum");
        {
            let (mut m, _) = Motor::ac(&k).unwrap();
            m.yaz("dil", "rust").unwrap();
            m.yaz("dil", "Rust").unwrap();
            m.yaz("yil", "2015").unwrap();
            assert!(m.sil("yil").unwrap());
        }
        let (mut m, r) = Motor::ac(&k).unwrap();
        assert_eq!((m.anahtarlar(), r.kayit), (vec![("dil", "Rust")], 4));
        assert!(m.yaz("buyuk", &"x".repeat(gunluk::AZAMI_DEGER + 1)).is_err());
    }

    #[test]
    fn ortadaki_bozulmada_acilmaz_ve_dosyaya_dokunmaz() {
        let k = dizin("bozuk");
        {
            let (mut m, _) = Motor::ac(&k).unwrap();
            for i in 0..3 {
                m.yaz(&format!("k{i}"), "v").unwrap();
            }
        }
        let yol = gunluk_yolu(&k);
        let mut b = std::fs::read(&yol).unwrap();
        b[19 + 16] ^= 1; // ikinci kaydın anahtarında tek bit (kayıt başına 16 + 2 + 1 = 19 bayt)
        std::fs::write(&yol, &b).unwrap();
        assert!(Motor::ac(&k).is_err());
        assert_eq!(std::fs::read(&yol).unwrap(), b);
    }
}
