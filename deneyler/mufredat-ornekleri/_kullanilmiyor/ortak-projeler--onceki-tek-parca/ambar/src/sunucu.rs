//! TCP sunucusu: her bağlantıya bir iş parçacığı; motor Arc ile paylaşılır.
use crate::motor::Motor;
use crate::protokol::{self, Istek};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

pub fn calistir(adres: &str, motor: Arc<Motor>) -> std::io::Result<()> {
    let dinleyici = TcpListener::bind(adres)?;
    eprintln!("ambar dinliyor: {}", dinleyici.local_addr()?);
    for baglanti in dinleyici.incoming() {
        let baglanti = baglanti?;
        let motor = Arc::clone(&motor);
        thread::spawn(move || {
            let _ = oturum(baglanti, &motor);
        });
    }
    Ok(())
}

fn oturum(baglanti: TcpStream, motor: &Motor) -> std::io::Result<()> {
    baglanti.set_nodelay(true)?;
    let okuyucu = BufReader::new(baglanti.try_clone()?);
    let mut yazici = BufWriter::new(baglanti);
    for satir in okuyucu.lines() {
        let yanit = match protokol::ayristir(&satir?) {
            Ok(Istek::Set(a, d)) => motor.yaz(&a, Some(&d)).map(|_| "OK".to_string()).unwrap_or_else(|h| format!("HATA {h}")),
            Ok(Istek::Get(a)) => motor.oku(&a).map_or("YOK".into(), |d| format!("DEGER {d}")),
            Ok(Istek::Del(a)) => match motor.yaz(&a, None) {
                Ok(true) => "OK".into(),
                Ok(false) => "YOK".into(),
                Err(h) => format!("HATA {h}"),
            },
            Ok(Istek::Say) => format!("SAYI {}", motor.sayi()),
            Err(h) => format!("HATA {h}"),
        };
        writeln!(yazici, "{yanit}")?;
        yazici.flush()?;
    }
    Ok(())
}
