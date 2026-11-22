use anyhow::{Context, Result};
use thiserror::Error;

// Kütüphane katmanı: çağıranın ayırt edebileceği, tipli hatalar (thiserror)
#[derive(Debug, Error)]
enum AyarHatasi {
    #[error("ayar dosyası okunamadı: {yol}")]
    Okuma {
        yol: String,
        #[source]
        kaynak: std::io::Error,
    },
    #[error("satır {satir}: {metin:?} anahtar=değer biçiminde değil")]
    Bicim { satir: usize, metin: String },
}

fn ayar_oku(yol: &str) -> Result<Vec<(String, String)>, AyarHatasi> {
    let icerik = std::fs::read_to_string(yol)
        .map_err(|kaynak| AyarHatasi::Okuma { yol: yol.to_string(), kaynak })?;
    icerik
        .lines()
        .enumerate()
        .map(|(i, satir)| {
            satir
                .split_once('=')
                .map(|(a, d)| (a.trim().to_string(), d.trim().to_string()))
                .ok_or(AyarHatasi::Bicim { satir: i + 1, metin: satir.to_string() })
        })
        .collect()
}

// Uygulama katmanı: bağlam ekleyip yukarı taşı (anyhow)
fn main() -> Result<()> {
    let yol = std::env::args().nth(1).unwrap_or_default();
    let ayarlar = ayar_oku(&yol).with_context(|| format!("sunucu başlatılamadı ({yol})"))?;
    for (anahtar, deger) in ayarlar {
        println!("{anahtar} → {deger}");
    }
    Ok(())
}
