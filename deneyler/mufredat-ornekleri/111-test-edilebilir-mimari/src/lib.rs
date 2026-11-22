use std::collections::HashMap;

// "Bağlantı noktaları" (ports): iş mantığı dış dünyayı yalnızca bu trait'lerle görür
pub trait Saat {
    fn bugun(&self) -> u32; // gün numarası
}
pub trait UyeDeposu {
    fn bitis_gunu(&self, uye: &str) -> Option<u32>;
}

#[derive(Debug, PartialEq)]
pub enum Durum {
    Aktif { kalan_gun: u32 },
    SuresiDoldu,
    Bilinmiyor,
}

// İş mantığı: saf, deterministik, jenerik (statik dağıtım)
pub fn uyelik_durumu(saat: &impl Saat, depo: &impl UyeDeposu, uye: &str) -> Durum {
    match depo.bitis_gunu(uye) {
        None => Durum::Bilinmiyor,
        Some(bitis) if bitis >= saat.bugun() => Durum::Aktif { kalan_gun: bitis - saat.bugun() },
        Some(_) => Durum::SuresiDoldu,
    }
}

// Gerçek bağdaştırıcılar (adapters) uygulamada; testlerde sahteleri
pub struct SabitSaat(pub u32);
impl Saat for SabitSaat {
    fn bugun(&self) -> u32 { self.0 }
}
pub struct BellekDepo(pub HashMap<String, u32>);
impl UyeDeposu for BellekDepo {
    fn bitis_gunu(&self, uye: &str) -> Option<u32> { self.0.get(uye).copied() }
}

#[cfg(test)]
mod testler {
    use super::*;

    fn depo() -> BellekDepo {
        BellekDepo(HashMap::from([("ayse".to_string(), 100), ("can".to_string(), 90)]))
    }

    #[test]
    fn suresi_dolmamis_uye() {
        assert_eq!(uyelik_durumu(&SabitSaat(95), &depo(), "ayse"), Durum::Aktif { kalan_gun: 5 });
    }
    #[test]
    fn son_gun_hala_aktif() {
        assert_eq!(uyelik_durumu(&SabitSaat(100), &depo(), "ayse"), Durum::Aktif { kalan_gun: 0 });
    }
    #[test]
    fn suresi_dolmus_uye() {
        assert_eq!(uyelik_durumu(&SabitSaat(95), &depo(), "can"), Durum::SuresiDoldu);
    }
    #[test]
    fn olmayan_uye() {
        assert_eq!(uyelik_durumu(&SabitSaat(1), &depo(), "zeynep"), Durum::Bilinmiyor);
    }
}
