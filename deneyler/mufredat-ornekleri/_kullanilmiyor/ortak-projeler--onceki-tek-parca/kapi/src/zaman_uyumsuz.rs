//! Aynı sunucu, tokio ile: her bağlantıya bir görev.
use crate::ayristirici::Ayristirici;
use crate::dosya;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn calistir(adres: &str, kok: PathBuf) -> std::io::Result<()> {
    let dinleyici = TcpListener::bind(adres).await?;
    eprintln!("async sunucu dinliyor: {}", dinleyici.local_addr()?);
    let kok = Arc::new(kok);
    loop {
        let (mut b, _) = dinleyici.accept().await?;
        let kok = Arc::clone(&kok);
        tokio::spawn(async move {
            let _ = b.set_nodelay(true);
            let mut a = Ayristirici::default();
            let mut tampon = [0u8; 4096];
            let mut veri: Vec<u8> = Vec::new();
            loop {
                match a.besle(&veri) {
                    Ok(Some(istek)) => {
                        if b.write_all(&dosya::yanit(&kok, &istek)).await.is_err() || istek.baglanti_kapansin_mi() { return; }
                        veri.clear();
                        continue;
                    }
                    Ok(None) => {}
                    Err(h) => {
                        let _ = b.write_all(&dosya::paketle(400, "Bad Request", format!("{h:?}\n").as_bytes())).await;
                        return;
                    }
                }
                match b.read(&mut tampon).await {
                    Ok(0) | Err(_) => return,
                    Ok(n) => veri = tampon[..n].to_vec(),
                }
            }
        });
    }
}
