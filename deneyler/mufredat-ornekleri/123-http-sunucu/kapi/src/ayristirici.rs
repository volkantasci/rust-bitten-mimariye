//! Artımlı (incremental) HTTP/1.1 istek ayrıştırıcısı: bir sonlu durum makinesi.
//! TCP bir bayt akışıdır; bir istek birçok parçaya bölünmüş gelebilir, bir parçada birden
//! çok istek de olabilir. Her parça `besle` ile verilir; tam bir istek oluşunca döner.
//! HTTP/1.1'in bir alt kümesidir (RFC 9112): Content-Length'li gövde desteklenir,
//! Transfer-Encoding (ör. chunked) desteklenmez ve reddedilir.

#[derive(Debug, Clone, PartialEq)]
pub struct Istek {
    pub yontem: String,
    pub hedef: String,
    pub surum: String,
    pub basliklar: Vec<(String, String)>,
    pub govde: Vec<u8>,
}

impl Istek {
    /// Başlık adları büyük/küçük harf duyarsızdır (ASCII).
    pub fn baslik(&self, ad: &str) -> Option<&str> {
        self.basliklar.iter().find(|(a, _)| a.eq_ignore_ascii_case(ad)).map(|(_, d)| d.as_str())
    }
    /// HTTP/1.1'de bağlantı varsayılan olarak açık kalır; "Connection: close" ya da HTTP/1.0 kapatır.
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
    GovdeCokBuyuk,
    DesteklenmeyenAktarim,
}

#[derive(Debug)]
enum Durum {
    Basliklar,              // "\r\n\r\n" bekleniyor
    Govde { uzunluk: usize }, // gövdenin tamamı (Content-Length bayt) bekleniyor
}

/// Başlık bölümünün (istek satırı + başlıklar) üst sınırı: sınırsız girdi bellek tüketir.
pub const AZAMI_BASLIK: usize = 8 * 1024;
/// Gövdenin üst sınırı.
pub const AZAMI_GOVDE: usize = 1024 * 1024;

pub struct Ayristirici {
    tampon: Vec<u8>,
    taranan: usize, // tamponun "\r\n\r\n" aranmış öneki: her beslemede baştan aranmaz
    durum: Durum,
    yarim: Option<Istek>,
}

impl Default for Ayristirici {
    fn default() -> Self {
        Ayristirici { tampon: Vec::new(), taranan: 0, durum: Durum::Basliklar, yarim: None }
    }
}

fn token_mi(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&b))
}

impl Ayristirici {
    /// Durumun okunur adı (gösterim için).
    pub fn durum(&self) -> String {
        match self.durum {
            Durum::Basliklar => "Basliklar".into(),
            Durum::Govde { uzunluk } => format!("Govde {{ uzunluk: {uzunluk} }}"),
        }
    }

    pub fn tampondaki_bayt(&self) -> usize {
        self.tampon.len()
    }

    /// Yeni baytları ekler; tam bir istek oluştuysa döndürür. Fazla baytlar tamponda kalır:
    /// sonraki istek için `besle(b"")` çağrılabilir.
    pub fn besle(&mut self, veri: &[u8]) -> Result<Option<Istek>, Hata> {
        self.tampon.extend_from_slice(veri);
        loop {
            match self.durum {
                Durum::Basliklar => {
                    // Sınır, önceki parçanın son 3 baytıyla bu parçanın başına bölünmüş olabilir
                    let bas = self.taranan.saturating_sub(3);
                    let Some(konum) = self.tampon[bas..].windows(4).position(|w| w == b"\r\n\r\n") else {
                        self.taranan = self.tampon.len();
                        if self.tampon.len() > AZAMI_BASLIK {
                            return Err(Hata::BasliklarCokBuyuk);
                        }
                        return Ok(None); // daha fazla veri gerekiyor
                    };
                    let son = bas + konum;
                    if son > AZAMI_BASLIK {
                        return Err(Hata::BasliklarCokBuyuk);
                    }
                    let baslik_baytlari: Vec<u8> = self.tampon.drain(..son + 4).collect();
                    self.taranan = 0;
                    let istek = Self::basligi_coz(&baslik_baytlari[..son])?;
                    let uzunluk = Self::govde_uzunlugu(&istek)?;
                    self.yarim = Some(istek);
                    self.durum = Durum::Govde { uzunluk };
                }
                Durum::Govde { uzunluk } => {
                    if self.tampon.len() < uzunluk {
                        return Ok(None);
                    }
                    let mut istek = self.yarim.take().unwrap();
                    istek.govde = self.tampon.drain(..uzunluk).collect();
                    self.durum = Durum::Basliklar;
                    return Ok(Some(istek));
                }
            }
        }
    }

    fn basligi_coz(baytlar: &[u8]) -> Result<Istek, Hata> {
        let metin = String::from_utf8_lossy(baytlar);
        let mut satirlar = metin.split("\r\n");
        let ilk = satirlar.next().unwrap_or("");
        // istek satırı = yöntem SP hedef SP sürüm
        let [yontem, hedef, surum] = ilk.split(' ').collect::<Vec<_>>()[..] else {
            return Err(Hata::BozukIstekSatiri(ilk.to_string()));
        };
        if !token_mi(yontem) || hedef.is_empty() || !matches!(surum, "HTTP/1.0" | "HTTP/1.1") {
            return Err(Hata::BozukIstekSatiri(ilk.to_string()));
        }
        let mut basliklar = Vec::new();
        for s in satirlar {
            let (ad, deger) = s.split_once(':').ok_or_else(|| Hata::BozukBaslik(s.to_string()))?;
            if !token_mi(ad) {
                // ad ile ':' arasında boşluk da buraya düşer (RFC 9112 §5.1: 400 ile reddedilir)
                return Err(Hata::BozukBaslik(s.to_string()));
            }
            basliklar.push((ad.to_string(), deger.trim_matches([' ', '\t']).to_string()));
        }
        Ok(Istek { yontem: yontem.into(), hedef: hedef.into(), surum: surum.into(), basliklar, govde: Vec::new() })
    }

    fn govde_uzunlugu(istek: &Istek) -> Result<usize, Hata> {
        if istek.baslik("Transfer-Encoding").is_some() {
            return Err(Hata::DesteklenmeyenAktarim);
        }
        let mut uzunluk: Option<usize> = None;
        for (_, d) in istek.basliklar.iter().filter(|(a, _)| a.eq_ignore_ascii_case("Content-Length")) {
            // Content-Length = 1*DIGIT: "+5" ya da "-1" geçersiz; farklı değerli tekrarlar da
            if d.is_empty() || !d.bytes().all(|b| b.is_ascii_digit()) {
                return Err(Hata::GecersizUzunluk);
            }
            let u: usize = d.parse().map_err(|_| Hata::GecersizUzunluk)?;
            if uzunluk.is_some_and(|onceki| onceki != u) {
                return Err(Hata::GecersizUzunluk);
            }
            uzunluk = Some(u);
        }
        let u = uzunluk.unwrap_or(0);
        if u > AZAMI_GOVDE {
            return Err(Hata::GovdeCokBuyuk);
        }
        Ok(u)
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
            if let Some(i) = a.besle(std::slice::from_ref(b)).unwrap() {
                bulunan.push(i);
            }
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
    fn sinirsiz_baslik_reddedilir() {
        let mut a = Ayristirici::default();
        let cop = vec![b'a'; AZAMI_BASLIK + 1];
        assert_eq!(a.besle(&cop), Err(Hata::BasliklarCokBuyuk));
    }

    #[test]
    fn ad_ile_iki_nokta_arasinda_bosluk_reddedilir() {
        let mut a = Ayristirici::default();
        assert!(matches!(a.besle(b"GET / HTTP/1.1\r\nHost : kapi\r\n\r\n"), Err(Hata::BozukBaslik(_))));
    }

    #[test]
    fn content_length_yalniz_rakam() {
        let mut a = Ayristirici::default();
        assert_eq!(a.besle(b"POST / HTTP/1.1\r\nContent-Length: +5\r\n\r\n"), Err(Hata::GecersizUzunluk));
    }

    #[test]
    fn transfer_encoding_desteklenmiyor() {
        let mut a = Ayristirici::default();
        let istek = b"POST / HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n";
        assert_eq!(a.besle(istek), Err(Hata::DesteklenmeyenAktarim));
    }
}
