//! İstek hedefini güvenli biçimde dosya yoluna çevirip yanıt üretir (124. adım).
use crate::ayristirici::{Hata, Istek};
use std::path::{Component, Path, PathBuf};

/// Yanıtı oluşturur. GET ve HEAD desteklenir; HEAD aynı başlıkları gövdesiz gönderir.
pub fn yanit(kok: &Path, istek: &Istek) -> Vec<u8> {
    let kapat = istek.baglanti_kapansin_mi();
    let yalniz_baslik = istek.yontem == "HEAD";
    if istek.yontem != "GET" && istek.yontem != "HEAD" {
        return paketle(405, "Method Not Allowed", &[("Allow", "GET, HEAD")], b"yalnizca GET ve HEAD\n", false, kapat);
    }
    match guvenli_yol(kok, &istek.hedef) {
        None => paketle(403, "Forbidden", &[], b"izin yok\n", yalniz_baslik, kapat),
        Some(yol) => match std::fs::read(&yol) {
            Ok(icerik) => paketle(200, "OK", &[("Content-Type", mime_turu(&yol))], &icerik, yalniz_baslik, kapat),
            Err(_) => paketle(404, "Not Found", &[], b"bulunamadi\n", yalniz_baslik, kapat),
        },
    }
}

/// Ayrıştırma hatasının yanıtı; ardından bağlantı kapatılır.
pub fn hata_yaniti(h: &Hata) -> Vec<u8> {
    let (kod, aciklama) = match h {
        Hata::BasliklarCokBuyuk => (431, "Request Header Fields Too Large"),
        Hata::GovdeCokBuyuk => (413, "Content Too Large"),
        Hata::DesteklenmeyenAktarim => (501, "Not Implemented"),
        _ => (400, "Bad Request"),
    };
    paketle(kod, aciklama, &[], format!("{h:?}\n").as_bytes(), false, true)
}

/// İstek, sunucunun tanıdığı süre içinde tamamlanmadı; ardından bağlantı kapatılır.
pub fn zaman_asimi_yaniti() -> Vec<u8> {
    paketle(408, "Request Timeout", &[], b"istek zamaninda tamamlanmadi\n", false, true)
}

/// Dizin geçişi (path traversal) saldırısına karşı: hedef kökün içinde kalmalı.
/// Kural basit ve katıdır: ".." ve kök ("/") bileşenleri hiç kabul edilmez.
pub fn guvenli_yol(kok: &Path, hedef: &str) -> Option<PathBuf> {
    let yol = hedef.split('?').next()?; // sorgu kısmı dosya adına dahil değil
    let goreli = yol.strip_prefix('/')?; // yalnız "/..." biçimindeki hedefler
    let goreli = if goreli.is_empty() { "index.html" } else { goreli };
    let p = Path::new(goreli);
    if p.components().any(|c| !matches!(c, Component::Normal(_))) {
        return None; // ParentDir (".."), RootDir ("/"), baştaki CurDir (".")
    }
    Some(kok.join(p))
}

/// Dosya uzantısından MIME türü (medya türü: "tür/alt tür", RFC 9110 §8.3.1).
fn mime_turu(yol: &Path) -> &'static str {
    match yol.extension().and_then(|u| u.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        Some("css") => "text/css",
        Some("js") => "text/javascript",
        Some("png") => "image/png",
        _ => "application/octet-stream",
    }
}

/// Durum satırı + başlıklar + boş satır + gövde. Content-Length bayt sayısıdır.
pub fn paketle(kod: u16, aciklama: &str, ek: &[(&str, &str)], govde: &[u8], yalniz_baslik: bool, kapat: bool) -> Vec<u8> {
    let mut v = format!("HTTP/1.1 {kod} {aciklama}\r\nContent-Length: {}\r\n", govde.len());
    if !ek.iter().any(|(a, _)| *a == "Content-Type") {
        v.push_str("Content-Type: text/plain; charset=utf-8\r\n");
    }
    for (a, d) in ek {
        v.push_str(&format!("{a}: {d}\r\n"));
    }
    if kapat {
        v.push_str("Connection: close\r\n");
    }
    v.push_str("\r\n");
    let mut v = v.into_bytes();
    if !yalniz_baslik {
        v.extend_from_slice(govde);
    }
    v
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn kokun_icindeki_yollar() {
        let kok = Path::new("kok");
        assert_eq!(guvenli_yol(kok, "/"), Some(kok.join("index.html")));
        assert_eq!(guvenli_yol(kok, "/belgeler/not.txt?x=1"), Some(kok.join("belgeler/not.txt")));
    }

    #[test]
    fn kokun_disina_cikan_yollar_reddedilir() {
        let kok = Path::new("kok");
        for hedef in ["/../../etc/passwd", "//etc/passwd", "/belgeler/../index.html", "/./index.html", "belgeler"] {
            assert_eq!(guvenli_yol(kok, hedef), None, "{hedef}");
        }
    }

    #[test]
    fn content_length_bayt_sayar() {
        let v = paketle(200, "OK", &[], "Kapı\n".as_bytes(), false, false);
        assert!(String::from_utf8(v).unwrap().contains("Content-Length: 6\r\n")); // ı iki bayt
    }
}
