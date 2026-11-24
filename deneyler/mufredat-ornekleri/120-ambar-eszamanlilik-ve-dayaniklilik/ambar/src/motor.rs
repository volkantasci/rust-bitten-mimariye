//! Depolama motoru (120. adım): eşzamanlı kullanım için kendi kilitlerini taşır.
//! Okumalar indeksin okuma kilidini (RwLock) paylaşır; yazmalar günlük kilidini (Mutex)
//! sırayla alır. Değişmez koşul: bir kayıt indekse, günlüğe eklendiği sırayla uygulanır.
//! Kalıcılık politikası seçilebilir: her yazmada fsync ya da hiç fsync.
use crate::gunluk::{self, Kayit};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, RwLock};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsyncKipi {
    Her, // her yazmada fsync: onaylanan yazma güç kesintisinde de diskte olmalı
    Yok, // fsync yok: veri çekirdeğin sayfa önbelleğinde; diske ne zaman ineceğine çekirdek karar verir
}

struct Yazici {
    dosya: File, // yalnızca sona ekleme (O_APPEND) kipinde açık
    kayit_sayisi: usize,
}

pub struct Motor {
    indeks: RwLock<HashMap<String, String>>, // anahtar → son değer
    yazici: Mutex<Yazici>,
    kip: FsyncKipi,
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

/// Yarım kalmış bir yazmadan sonra günlüğün kuyruğu belirsizdir; ardına yeni kayıt eklemek
/// günlüğün ortasında bozulma yaratırdı. Süreç durur; açılıştaki kurtarma kuyruğu düzeltir.
fn dur(ne: &str, h: io::Error) -> ! {
    eprintln!("ambar: {ne}: {h}; süreç durduruluyor");
    std::process::abort()
}

fn gunluk_yolu(dizin: &Path) -> PathBuf {
    dizin.join("gunluk.ambar")
}

impl Motor {
    /// Günlüğü doğrulayarak yeniden oynatır (replay) ve indeksi kurar.
    pub fn ac(dizin: &Path, kip: FsyncKipi) -> io::Result<(Motor, Rapor)> {
        std::fs::create_dir_all(dizin)?;
        let yol = gunluk_yolu(dizin);
        let yeni = !yol.exists();
        let okuma = gunluk::oku(&yol)?;
        let mut atilan_bayt = 0;
        if let Some(d) = &okuma.durma {
            if !d.kuyrukta_mi() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("günlük bozuk: {d}; dosyaya dokunulmadı"),
                ));
            }
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
        let dosya = OpenOptions::new().create(true).append(true).open(&yol)?;
        if yeni && kip != FsyncKipi::Yok {
            // Yeni dosyanın dizindeki adı da kalıcı olsun: dizinin kendisi fsync edilir.
            File::open(dizin)?.sync_all()?;
        }
        let yazici = Yazici { dosya, kayit_sayisi: rapor.kayit };
        Ok((Motor { indeks: RwLock::new(indeks), yazici: Mutex::new(yazici), kip }, rapor))
    }

    /// Kaydı günlüğe ekler ve indekse uygular. Çağıran günlük kilidini tutuyor.
    fn ekle(&self, y: &mut Yazici, k: Kayit) {
        if let Err(h) = y.dosya.write_all(&gunluk::kodla(&k)) {
            dur("günlüğe yazılamadı", h);
        }
        if self.kip == FsyncKipi::Her {
            // fdatasync: veri (ve dosya boyu) diske inene kadar bekle
            if let Err(h) = y.dosya.sync_data() {
                dur("fsync başarısız", h);
            }
        }
        y.kayit_sayisi += 1;
        uygula(&mut self.indeks.write().unwrap(), k); // günlük kilidi hâlâ elimizde
    }

    /// Üst sınır: değerler bellekte tutulur; sınırı aşan bir kayıt yazılmaz.
    pub fn yaz(&self, anahtar: &str, deger: &str) -> io::Result<()> {
        if anahtar.len() > gunluk::AZAMI_ANAHTAR || deger.len() > gunluk::AZAMI_DEGER {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("anahtar en çok {} bayt, değer en çok {} bayt olabilir", gunluk::AZAMI_ANAHTAR, gunluk::AZAMI_DEGER),
            ));
        }
        let mut y = self.yazici.lock().unwrap(); // günlük sırası = indekse uygulanma sırası
        self.ekle(&mut y, Kayit { anahtar: anahtar.into(), deger: Some(deger.into()) });
        Ok(())
    }

    /// Anahtar varsa silme kaydı ekler; yoksa günlüğe bir şey yazmaz.
    pub fn sil(&self, anahtar: &str) -> io::Result<bool> {
        let mut y = self.yazici.lock().unwrap(); // denetim ve yazma aynı kilit altında
        if !self.indeks.read().unwrap().contains_key(anahtar) {
            return Ok(false);
        }
        self.ekle(&mut y, Kayit { anahtar: anahtar.into(), deger: None });
        Ok(true)
    }

    pub fn oku(&self, anahtar: &str) -> Option<String> {
        self.indeks.read().unwrap().get(anahtar).cloned()
    }

    /// Canlı anahtarlar, sıralı.
    pub fn anahtarlar(&self) -> Vec<(String, String)> {
        let mut v: Vec<_> = self.indeks.read().unwrap().iter().map(|(a, d)| (a.clone(), d.clone())).collect();
        v.sort();
        v
    }

    pub fn kayit_sayisi(&self) -> usize {
        self.yazici.lock().unwrap().kayit_sayisi
    }

    pub fn anahtar_sayisi(&self) -> usize {
        self.indeks.read().unwrap().len()
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
            let (m, _) = Motor::ac(&k, FsyncKipi::Her).unwrap();
            m.yaz("dil", "rust").unwrap();
            m.yaz("dil", "Rust").unwrap();
            m.yaz("yil", "2015").unwrap();
            assert!(m.sil("yil").unwrap());
        }
        let (m, r) = Motor::ac(&k, FsyncKipi::Yok).unwrap();
        assert_eq!(r.kayit, 4);
        assert_eq!(m.anahtarlar(), [("dil".to_string(), "Rust".to_string())]);
    }

    #[test]
    fn gunluk_sirasi_indeks_sirasiyla_ayni() {
        // 8 iş parçacığı aynı anahtara kendi numarasını 500'er kez yazar. Bellekteki son değer,
        // günlüğü yeniden oynatınca bulunan son değerle aynı olmalı.
        for tur in 0..20 {
            let k = dizin(&format!("sira{tur}"));
            let (m, _) = Motor::ac(&k, FsyncKipi::Yok).unwrap();
            std::thread::scope(|s| {
                for t in 0..8 {
                    let m = &m;
                    s.spawn(move || {
                        for _ in 0..500 {
                            m.yaz("ortak", &t.to_string()).unwrap();
                        }
                    });
                }
            });
            let bellekte = m.oku("ortak");
            drop(m);
            let (yeniden, _) = Motor::ac(&k, FsyncKipi::Yok).unwrap();
            assert_eq!(yeniden.oku("ortak"), bellekte, "{tur}. turda bellek ile günlük ayrıştı");
            std::fs::remove_dir_all(&k).unwrap();
        }
    }
}
