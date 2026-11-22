use tracing::{debug, error, info, instrument, warn};

#[instrument] // fonksiyon bir "span" olur; argümanlar alan olarak kaydedilir
fn siparis_isle(id: u64, kalem_sayisi: u32) -> Result<u32, String> {
    info!("işleniyor");
    if kalem_sayisi == 0 {
        warn!("boş sipariş");
        return Err("boş".into());
    }
    let tutar = odeme_al(kalem_sayisi * 250)?;
    debug!(tutar, "ödeme tamam");
    Ok(tutar)
}

#[instrument]
fn odeme_al(tutar: u32) -> Result<u32, String> {
    if tutar > 1000 {
        error!(tutar, sinir = 1000, "limit aşıldı");
        return Err("limit".into());
    }
    info!("onaylandı");
    Ok(tutar)
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .without_time() // deterministik çıktı için zaman damgası kapalı
        .with_target(false)
        .with_ansi(false)
        .init();
    for (id, adet) in [(41, 2), (42, 0), (43, 7)] {
        let _ = siparis_isle(id, adet);
    }
}
