//! Artımlı (incremental) HTTP/1.1 istek ayrıştırıcısı: bir durum makinesi.
//! Veri parça parça gelir (TCP bir akıştır); her parça `besle` ile verilir.

#[derive(Debug, Clone, PartialEq)]
pub struct Istek {
    pub yontem: String,
    pub hedef: String,
    pub surum: String,
    pub basliklar: Vec<(String, String)>,
    pub govde: Vec<u8>,
}

impl Istek {
    pub fn baslik(&self, ad: &str) -> Option<&str> {
        self.basliklar.iter().find(|(a, _)| a.eq_ignore_ascii_case(ad)).map(|(_, d)| d.as_str())
    }
    pub fn baglanti_kapansin_mi(&self) -> bool {
        self.baslik("Connection").is_some_and(|d| d.eq_ignore_ascii_case("close")) || self.surum == "HTTP/1.0"
    }
}

#[derive(Debug, PartialEq)]
pub enum Hata {
    BozukIstekSatiri(String),
    BozukBaslik(String),
    BasliklarCokBuyuk,
    GecersizUzunluk,
}

#[derive(Debug)]
enum Durum {
    Basliklar,             // "\r\n\r\n" bekleniyor
    Govde { kalan: usize }, // Content-Length kadar bayt bekleniyor
}

pub struct Ayristirici {
    tampon: Vec<u8>,
    durum: Durum,
    yarim: Option<Istek>,
}

const AZAMI_BASLIK: usize = 8 * 1024;

impl Default for Ayristirici {
    fn default() -> Self {
        Ayristirici { tampon: Vec::new(), durum: Durum::Basliklar, yarim: None }
    }
}

impl Ayristirici {
    /// Yeni baytları ekler; tam bir istek oluştuysa döndürür (fazla baytlar sonraki istek için saklanır).
    pub fn besle(&mut self, veri: &[u8]) -> Result<Option<Istek>, Hata> {
        self.tampon.extend_from_slice(veri);
        loop {
            match self.durum {
                Durum::Basliklar => {
                    let Some(son) = self.tampon.windows(4).position(|w| w == b"\r\n\r\n") else {
                        if self.tampon.len() > AZAMI_BASLIK { return Err(Hata::BasliklarCokBuyuk); }
                        return Ok(None); // daha fazla veri gerekiyor
                    };
                    let bas: Vec<u8> = self.tampon.drain(..son + 4).collect();
                    let metin = String::from_utf8_lossy(&bas[..son]).into_owned();
                    let mut satirlar = metin.split("\r\n");
                    let ilk = satirlar.next().unwrap_or("");
                    let parcalar: Vec<&str> = ilk.split(' ').collect();
                    let [yontem, hedef, surum] = parcalar[..] else {
                        return Err(Hata::BozukIstekSatiri(ilk.to_string()));
                    };
                    if !surum.starts_with("HTTP/1.") || hedef.is_empty() {
                        return Err(Hata::BozukIstekSatiri(ilk.to_string()));
                    }
                    let mut basliklar = Vec::new();
                    for s in satirlar {
                        let (a, d) = s.split_once(':').ok_or_else(|| Hata::BozukBaslik(s.to_string()))?;
                        basliklar.push((a.trim().to_string(), d.trim().to_string()));
                    }
                    let istek = Istek { yontem: yontem.into(), hedef: hedef.into(), surum: surum.into(), basliklar, govde: Vec::new() };
                    let uzunluk = match istek.baslik("Content-Length") {
                        Some(u) => u.parse::<usize>().map_err(|_| Hata::GecersizUzunluk)?,
                        None => 0,
                    };
                    self.yarim = Some(istek);
                    self.durum = Durum::Govde { kalan: uzunluk };
                }
                Durum::Govde { kalan } => {
                    if self.tampon.len() < kalan { return Ok(None); }
                    let mut istek = self.yarim.take().unwrap();
                    istek.govde = self.tampon.drain(..kalan).collect();
                    self.durum = Durum::Basliklar;
                    return Ok(Some(istek));
                }
            }
        }
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    const ORNEK: &[u8] = b"POST /kayit HTTP/1.1\r\nHost: ornek\r\nContent-Length: 5\r\n\r\nmerhaGET / HTTP/1.1\r\n\r\n";

    #[test]
    fn tek_parcada_iki_istek() {
        let mut a = Ayristirici::default();
        let birinci = a.besle(ORNEK).unwrap().unwrap();
        assert_eq!((birinci.yontem.as_str(), birinci.govde.as_slice()), ("POST", &b"merha"[..]));
        let ikinci = a.besle(b"").unwrap().unwrap(); // tamponda kalan ikinci istek
        assert_eq!(ikinci.hedef, "/");
    }

    #[test]
    fn bayt_bayt_beslemek_ayni_sonucu_verir() {
        let mut a = Ayristirici::default();
        let mut bulunan = Vec::new();
        for b in ORNEK {
            if let Some(i) = a.besle(std::slice::from_ref(b)).unwrap() { bulunan.push(i); }
        }
        let mut b = Ayristirici::default();
        let toplu = vec![b.besle(ORNEK).unwrap().unwrap(), b.besle(b"").unwrap().unwrap()];
        assert_eq!(bulunan, toplu);
    }

    #[test]
    fn bozuk_istek_satiri() {
        let mut a = Ayristirici::default();
        assert!(matches!(a.besle(b"MERHABA\r\n\r\n"), Err(Hata::BozukIstekSatiri(_))));
    }

    #[test]
    fn sonsuz_baslik_reddedilir() {
        let mut a = Ayristirici::default();
        let cop = vec![b'a'; AZAMI_BASLIK + 1];
        assert_eq!(a.besle(&cop), Err(Hata::BasliklarCokBuyuk));
    }
}
