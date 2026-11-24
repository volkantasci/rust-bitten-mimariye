//! Aynı sunucu, tokio ile (124. adım): her bağlantıya bir görev (task). Veri bekleyen bir
//! görev iş parçacığı tutmaz; çalışma zamanı soketleri epoll ile izler ve hazır olan görevi
//! yürütür. Ayrıştırıcı ve yanıt kodu havuz sunucusuyla aynıdır. İsteğe bağlı son süre (408).
use crate::ayristirici::Ayristirici;
use crate::dosya;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{Instant, timeout_at};

pub async fn calistir(adres: &str, kok: PathBuf, son_sure: Option<Duration>) -> std::io::Result<()> {
    let dinleyici = TcpListener::bind(adres).await?;
    eprintln!("async sunucu dinliyor: {}; son süre: {son_sure:?}", dinleyici.local_addr()?);
    let kok = Arc::new(kok);
    loop {
        let (b, _) = dinleyici.accept().await?;
        let kok = Arc::clone(&kok);
        tokio::spawn(async move {
            let _ = oturum(b, &kok, son_sure).await;
        });
    }
}

async fn oturum(mut b: TcpStream, kok: &Path, son_sure: Option<Duration>) -> std::io::Result<()> {
    b.set_nodelay(true)?;
    let mut a = Ayristirici::default();
    let mut tampon = [0u8; 4096];
    let mut veri: Vec<u8> = Vec::new();
    let mut bitis = son_sure.map(|s| Instant::now() + s);
    loop {
        match a.besle(&veri) {
            Ok(Some(istek)) => {
                b.write_all(&dosya::yanit(kok, &istek)).await?;
                if istek.baglanti_kapansin_mi() {
                    return Ok(());
                }
                veri.clear();
                bitis = son_sure.map(|s| Instant::now() + s); // sonraki istek için yeni süre
                continue;
            }
            Ok(None) => {}
            Err(h) => {
                b.write_all(&dosya::hata_yaniti(&h)).await?;
                return Ok(());
            }
        }
        let n = match bitis {
            Some(t) => match timeout_at(t, b.read(&mut tampon)).await {
                Ok(okuma) => okuma?,
                Err(_) => {
                    b.write_all(&dosya::zaman_asimi_yaniti()).await?;
                    return Ok(());
                }
            },
            None => b.read(&mut tampon).await?,
        };
        if n == 0 {
            return Ok(()); // istemci kapattı
        }
        veri = tampon[..n].to_vec();
    }
}
