#[derive(Debug, PartialEq)]
pub enum IndirimHatasi {
    GecersizYuzde(String),
    AralikDisi(u32),
}

/// Kullanıcının girdiği yüzde indirimi (ör. "15%") fiyata uygular.
/// Hatalar çağırana döner; ara hesap u64'te yapılır (taşma yok).
pub fn indirimli_fiyat(fiyat_kurus: u32, yuzde: &str) -> Result<u32, IndirimHatasi> {
    let y: u32 = yuzde
        .trim()
        .trim_end_matches('%')
        .parse()
        .map_err(|_| IndirimHatasi::GecersizYuzde(yuzde.to_string()))?;
    if y > 100 {
        return Err(IndirimHatasi::AralikDisi(y));
    }
    let indirim = fiyat_kurus as u64 * y as u64 / 100;
    Ok(fiyat_kurus - indirim as u32) // indirim <= fiyat_kurus olduğu kanıtlandı
}

#[cfg(test)]
mod testler {
    use super::*;
    #[test]
    fn yuzde_on_bes() {
        assert_eq!(indirimli_fiyat(10_000, "15%"), Ok(8_500));
    }
    #[test]
    fn bozuk_girdi() {
        assert_eq!(indirimli_fiyat(10_000, "on beş"), Err(IndirimHatasi::GecersizYuzde("on beş".into())));
    }
    #[test]
    fn yuzden_fazla() {
        assert_eq!(indirimli_fiyat(10_000, "150%"), Err(IndirimHatasi::AralikDisi(150)));
    }
    #[test]
    fn buyuk_fiyat() {
        assert_eq!(indirimli_fiyat(50_000_000, "50%"), Ok(25_000_000));
    }
}
