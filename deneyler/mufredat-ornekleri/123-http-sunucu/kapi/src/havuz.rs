//! İş parçacığı havuzu sunucusu (123. adım): sabit sayıda işçi; kabul edilen bağlantılar
//! bir kanaldan işçilere dağıtılır. Bir işçi, bağlantı kapanana kadar o bağlantıya aittir
//! (HTTP/1.1 kalıcı bağlantı: aynı bağlantıda art arda istekler).
use crate::ayristirici::Ayristirici;
use crate::dosya;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub fn calistir(adres: &str, kok: PathBuf, isci_sayisi: usize) -> std::io::Result<()> {
    let dinleyici = TcpListener::bind(adres)?;
    eprintln!("havuz sunucusu {isci_sayisi} işçiyle dinliyor: {}", dinleyici.local_addr()?);
    let (gonderen, alan) = mpsc::channel::<TcpStream>();
    let alan = Arc::new(Mutex::new(alan)); // tek alıcı, işçiler sırayla kilitleyip bekler
    let kok = Arc::new(kok);
    for _ in 0..isci_sayisi {
        let alan = Arc::clone(&alan);
        let kok = Arc::clone(&kok);
        thread::spawn(move || loop {
            let baglanti = match alan.lock().unwrap().recv() {
                Ok(b) => b,
                Err(_) => return, // kanal kapandı
            };
            let _ = oturum(baglanti, &kok); // bu işçi, bağlantı kapanana kadar meşgul
        });
    }
    for b in dinleyici.incoming() {
        gonderen.send(b?).unwrap();
    }
    Ok(())
}

fn oturum(mut b: TcpStream, kok: &Path) -> std::io::Result<()> {
    b.set_nodelay(true)?;
    let mut a = Ayristirici::default();
    let mut tampon = [0u8; 4096];
    let mut veri: &[u8] = &[];
    loop {
        match a.besle(veri) {
            Ok(Some(istek)) => {
                b.write_all(&dosya::yanit(kok, &istek))?;
                if istek.baglanti_kapansin_mi() {
                    return Ok(());
                }
                veri = &[];
                continue; // tamponda başka istek olabilir
            }
            Ok(None) => {}
            Err(h) => {
                b.write_all(&dosya::hata_yaniti(&h))?;
                return Ok(());
            }
        }
        let n = b.read(&mut tampon)?;
        if n == 0 {
            return Ok(()); // istemci kapattı
        }
        veri = &tampon[..n];
    }
}
