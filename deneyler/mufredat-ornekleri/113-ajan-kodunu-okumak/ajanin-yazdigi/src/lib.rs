/// Kullanıcının girdiği yüzde indirimi (ör. "15%") fiyata uygular.
/// (Bir ajanın ilk denemede yazdığı, testleri geçen sürüm.)
pub fn indirimli_fiyat(fiyat_kurus: u32, yuzde: &str) -> u32 {
    let y: u32 = yuzde.trim_end_matches('%').parse().unwrap();
    fiyat_kurus - fiyat_kurus * y / 100
}

#[cfg(test)]
mod ajanin_testleri {
    use super::*;
    #[test]
    fn yuzde_on_bes() {
        assert_eq!(indirimli_fiyat(10_000, "15%"), 8_500);
    }
    #[test]
    fn indirim_yok() {
        assert_eq!(indirimli_fiyat(999, "0%"), 999);
    }
}

#[cfg(test)]
mod inceleme_testleri {
    use super::*;
    #[test]
    fn bozuk_girdi() {
        indirimli_fiyat(10_000, "on beş"); // kullanıcı girdisi: panik mi?
    }
    #[test]
    fn yuzden_fazla() {
        indirimli_fiyat(10_000, "150%"); // fiyat eksiye mi düşer?
    }
    #[test]
    fn buyuk_fiyat() {
        assert_eq!(indirimli_fiyat(50_000_000, "50%"), 25_000_000); // 500 bin TL
    }
}
