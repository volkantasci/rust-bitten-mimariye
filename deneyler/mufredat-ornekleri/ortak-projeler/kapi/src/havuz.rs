//! İş parçacığı havuzu sunucusu: sabit sayıda işçi, bağlantılar bir kanaldan dağıtılır.
use crate::ayristirici::Ayristirici;
use crate::dosya;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub fn calistir(adres: &str, kok: PathBuf, isci_sayisi: usize) -> std::io::Result<()> {
    let dinleyici = TcpListener::bind(adres)?;
    eprintln!("havuz sunucusu {isci_sayisi} işçiyle dinliyor: {}", dinleyici.local_addr()?);
    let (tx, rx) = mpsc::channel::<TcpStream>();
    let rx = Arc::new(Mutex::new(rx));
    let kok = Arc::new(kok);
    for _ in 0..isci_sayisi {
        let rx = Arc::clone(&rx);
        let kok = Arc::clone(&kok);
        thread::spawn(move || loop {
            let baglanti = match rx.lock().unwrap().recv() { Ok(b) => b, Err(_) => return };
            let _ = oturum(baglanti, &kok); // bu işçi, bağlantı kapanana kadar meşgul
        });
    }
    for b in dinleyici.incoming() {
        tx.send(b?).unwrap();
    }
    Ok(())
}

fn oturum(mut b: TcpStream, kok: &std::path::Path) -> std::io::Result<()> {
    b.set_nodelay(true)?;
    let mut a = Ayristirici::default();
    let mut tampon = [0u8; 4096];
    let mut veri: &[u8] = &[];
    loop {
        match a.besle(veri) {
            Ok(Some(istek)) => {
                b.write_all(&dosya::yanit(kok, &istek))?;
                if istek.baglanti_kapansin_mi() { return Ok(()); }
                veri = &[];
                continue; // tamponda başka istek olabilir
            }
            Ok(None) => {}
            Err(h) => {
                b.write_all(&dosya::paketle(400, "Bad Request", format!("{h:?}\n").as_bytes()))?;
                return Ok(());
            }
        }
        let n = b.read(&mut tampon)?;
        if n == 0 { return Ok(()); }
        veri = &tampon[..n];
    }
}
