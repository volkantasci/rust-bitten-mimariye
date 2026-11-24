//! TCP sunucusu (119. adım): her bağlantıya bir iş parçacığı; motor tek bir kilitle
//! (`Arc<Mutex<Motor>>`) paylaşılır. Kilitsiz paylaşma girişimi derlenmez; `cargo test`
//! bunu da denetler:
//!
//! ```compile_fail,E0596
//! use std::sync::Arc;
//! let (motor, _) = ambar::motor::Motor::ac("ornek".as_ref()).unwrap();
//! let motor = Arc::new(motor); // kilit yok
//! let m = Arc::clone(&motor);
//! std::thread::spawn(move || m.yaz("dil", "rust")); // yaz(&mut self) ister
//! ```
use crate::gunluk;
use crate::motor::Motor;
use crate::protokol::{self, Istek};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

/// Bir istek satırının en büyük uzunluğu: komut, anahtar, değer sınırları ve ayırıcılar.
const AZAMI_SATIR: u64 = (gunluk::AZAMI_ANAHTAR + gunluk::AZAMI_DEGER + 16) as u64;

pub fn calistir(adres: &str, motor: Motor) -> io::Result<()> {
    let dinleyici = TcpListener::bind(adres)?;
    eprintln!("ambar dinliyor: {}", dinleyici.local_addr()?);
    let motor = Arc::new(Mutex::new(motor));
    for baglanti in dinleyici.incoming() {
        let baglanti = baglanti?;
        let motor = Arc::clone(&motor);
        thread::spawn(move || {
            let _ = oturum(baglanti, &motor);
        });
    }
    Ok(())
}

/// Bir istek satırını motora uygular ve yanıt satırını döndürür.
pub fn yanitla(motor: &Mutex<Motor>, satir: &str) -> String {
    let istek = match protokol::ayristir(satir) {
        Ok(i) => i,
        Err(h) => return format!("hata: {h}"),
    };
    let mut m = motor.lock().unwrap(); // bir istek boyunca motor tek iş parçacığına ait
    match istek {
        Istek::Yaz(a, d) => match m.yaz(&a, &d) {
            Ok(()) => "tamam".into(),
            Err(h) => format!("hata: {h}"),
        },
        Istek::Oku(a) => m.oku(&a).map_or("yok".into(), |d| format!("değer {d}")),
        Istek::Sil(a) => match m.sil(&a) {
            Ok(true) => "tamam".into(),
            Ok(false) => "yok".into(),
            Err(h) => format!("hata: {h}"),
        },
        Istek::Say => format!("sayı {}", m.anahtar_sayisi()),
    }
}

fn oturum(baglanti: TcpStream, motor: &Mutex<Motor>) -> io::Result<()> {
    baglanti.set_nodelay(true)?;
    let mut okuyucu = BufReader::new(baglanti.try_clone()?);
    let mut yazici = BufWriter::new(baglanti);
    loop {
        // TCP bir bayt akışıdır: satırı "\n" görene kadar biriktir, ama sınırsız değil
        let mut satir = Vec::new();
        let n = (&mut okuyucu).take(AZAMI_SATIR).read_until(b'\n', &mut satir)?;
        if n == 0 {
            return Ok(()); // istemci bağlantıyı kapattı
        }
        if satir.last() != Some(&b'\n') && n as u64 == AZAMI_SATIR {
            writeln!(yazici, "hata: satır çok uzun")?;
            yazici.flush()?;
            return Ok(());
        }
        let yanit = match std::str::from_utf8(&satir) {
            Ok(s) => yanitla(motor, s),
            Err(_) => "hata: satır UTF-8 değil".into(),
        };
        writeln!(yazici, "{yanit}")?;
        yazici.flush()?;
    }
}
