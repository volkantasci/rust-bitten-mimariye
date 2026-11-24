//! İş parçacığı havuzu sunucusu (124. adım): sabit sayıda işçi; kabul edilen bağlantılar
//! bir kanaldan işçilere dağıtılır. Bir işçi, bağlantı kapanana kadar o bağlantıya aittir
//! (HTTP/1.1 kalıcı bağlantı). İsteğe bağlı son süre: bir istek (önceki yanıttan ya da
//! bağlantının işçiye geçişinden itibaren) bu süre içinde tamamlanmazsa 408 ile kapatılır.
use crate::ayristirici::Ayristirici;
use crate::dosya;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

pub fn calistir(adres: &str, kok: PathBuf, isci_sayisi: usize, son_sure: Option<Duration>) -> std::io::Result<()> {
    let dinleyici = TcpListener::bind(adres)?;
    eprintln!("havuz sunucusu {isci_sayisi} işçiyle dinliyor: {}; son süre: {son_sure:?}", dinleyici.local_addr()?);
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
            let _ = oturum(baglanti, &kok, son_sure); // bu işçi, bağlantı kapanana kadar meşgul
        });
    }
    for b in dinleyici.incoming() {
        gonderen.send(b?).unwrap();
    }
    Ok(())
}

fn oturum(mut b: TcpStream, kok: &Path, son_sure: Option<Duration>) -> std::io::Result<()> {
    b.set_nodelay(true)?;
    let mut a = Ayristirici::default();
    let mut tampon = [0u8; 4096];
    let mut veri: &[u8] = &[];
    let mut bitis = son_sure.map(|s| Instant::now() + s);
    loop {
        match a.besle(veri) {
            Ok(Some(istek)) => {
                b.write_all(&dosya::yanit(kok, &istek))?;
                if istek.baglanti_kapansin_mi() {
                    return Ok(());
                }
                veri = &[];
                bitis = son_sure.map(|s| Instant::now() + s); // sonraki istek için yeni süre
                continue; // tamponda başka istek olabilir
            }
            Ok(None) => {}
            Err(h) => {
                b.write_all(&dosya::hata_yaniti(&h))?;
                return Ok(());
            }
        }
        if let Some(t) = bitis {
            // Okuma başına değil toplam süre: her okumadan önce kalan süreyi yeniden kur.
            // Yalnız okuma başına bir zaman aşımı, baytları damla damla gönderen istemciye yetmez.
            let kalan = t.saturating_duration_since(Instant::now());
            if kalan.is_zero() {
                b.write_all(&dosya::zaman_asimi_yaniti())?;
                return Ok(());
            }
            b.set_read_timeout(Some(kalan))?;
        }
        let n = match b.read(&mut tampon) {
            Ok(n) => n,
            // Unix'te okuma zaman aşımı WouldBlock olarak döner (std belgesi: platforma bağlı)
            Err(h) if matches!(h.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                b.write_all(&dosya::zaman_asimi_yaniti())?;
                return Ok(());
            }
            Err(h) => return Err(h),
        };
        if n == 0 {
            return Ok(()); // istemci kapattı
        }
        veri = &tampon[..n];
    }
}
