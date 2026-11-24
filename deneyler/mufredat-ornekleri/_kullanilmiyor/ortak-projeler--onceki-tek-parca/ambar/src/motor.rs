//! Depolama motoru: günlüğe yaz, dizini güncelle; kalıcılık (fsync) politikası seçilebilir.
use crate::gunluk::{self, Kayit, KurtarmaRaporu};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsyncKipi {
    Her,   // her yazmada fsync: en güvenli, en yavaş
    Toplu, // grup kaydı (group commit): birkaç ms'de bir tek fsync, yazanlar onu bekler
    Yok,   // fsync yok: işletim sistemi ne zaman isterse (güç kesilirse kayıp olabilir)
}

struct Yazici {
    dosya: File,
    sira: u64, // günlüğe eklenmiş son kaydın sıra numarası
}

pub struct Motor {
    harita: RwLock<HashMap<String, String>>,
    yazici: Mutex<Yazici>,
    kalici: Mutex<u64>, // diske indiği kesin olan son sıra numarası
    kalici_cv: Condvar,
    kip: FsyncKipi,
}

impl Motor {
    pub fn ac(dizin: &Path, kip: FsyncKipi) -> io::Result<(Arc<Motor>, KurtarmaRaporu)> {
        std::fs::create_dir_all(dizin)?;
        let yol = dizin.join("gunluk.ambar");
        let (kayitlar, rapor) = gunluk::kurtar(&yol)?;
        let mut harita = HashMap::new();
        for k in kayitlar {
            match k.deger {
                Some(d) => { harita.insert(k.anahtar, d); }
                None => { harita.remove(&k.anahtar); }
            }
        }
        let dosya = OpenOptions::new().create(true).append(true).open(&yol)?;
        let motor = Arc::new(Motor {
            harita: RwLock::new(harita),
            yazici: Mutex::new(Yazici { dosya, sira: 0 }),
            kalici: Mutex::new(0),
            kalici_cv: Condvar::new(),
            kip,
        });
        if kip == FsyncKipi::Toplu {
            let m = Arc::clone(&motor);
            let kopya = motor.yazici.lock().unwrap().dosya.try_clone()?;
            thread::spawn(move || m.grup_kaydi_dongusu(kopya));
        }
        Ok((motor, rapor))
    }

    fn grup_kaydi_dongusu(&self, dosya: File) {
        let mut son = 0;
        loop {
            thread::sleep(Duration::from_millis(2));
            let hedef = self.yazici.lock().unwrap().sira;
            if hedef > son {
                dosya.sync_data().expect("fsync başarısız"); // hedef'e kadar yazılanların hepsini kapsar
                *self.kalici.lock().unwrap() = hedef;
                self.kalici_cv.notify_all();
                son = hedef;
            }
        }
    }

    /// SET (deger = Some) ya da DEL (deger = None). Dönüş: anahtar önceden var mıydı?
    pub fn yaz(&self, anahtar: &str, deger: Option<&str>) -> io::Result<bool> {
        let kayit = gunluk::kodla(&Kayit { anahtar: anahtar.to_string(), deger: deger.map(str::to_string) });
        let (sira, vardi) = {
            let mut y = self.yazici.lock().unwrap(); // günlük sırası = dizine uygulanma sırası
            y.dosya.write_all(&kayit)?;
            if self.kip == FsyncKipi::Her {
                y.dosya.sync_data()?;
            }
            y.sira += 1;
            let mut h = self.harita.write().unwrap();
            let vardi = match deger {
                Some(d) => h.insert(anahtar.to_string(), d.to_string()).is_some(),
                None => h.remove(anahtar).is_some(),
            };
            (y.sira, vardi)
        };
        if self.kip == FsyncKipi::Toplu {
            let mut k = self.kalici.lock().unwrap();
            while *k < sira {
                k = self.kalici_cv.wait(k).unwrap(); // onaydan önce kalıcı olmasını bekle
            }
        }
        Ok(vardi)
    }

    pub fn oku(&self, anahtar: &str) -> Option<String> {
        self.harita.read().unwrap().get(anahtar).cloned()
    }

    pub fn sayi(&self) -> usize {
        self.harita.read().unwrap().len()
    }

    pub fn anahtarlar(&self) -> Vec<(String, String)> {
        let mut v: Vec<_> = self.harita.read().unwrap().iter().map(|(a, d)| (a.clone(), d.clone())).collect();
        v.sort();
        v
    }
}
