// Düzeltilmiş: hata bir değer olarak döner, bağlam eklenir, main çıkış koduyla biter.
use anyhow::{Context, Result};

fn port_oku(girdi: &str) -> Result<u16> {
    girdi.trim().parse::<u16>().with_context(|| format!("port numarası okunamadı: {girdi:?}"))
}

fn main() -> Result<()> {
    let girdi = std::env::args().nth(1).unwrap_or_default();
    let port = port_oku(&girdi)?;
    println!("sunucu {port} numaralı portta başlıyor");
    Ok(())
}
