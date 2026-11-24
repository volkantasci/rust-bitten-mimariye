//! İstek hedefini güvenli biçimde dosya yoluna çevirip yanıt üretir.
use crate::ayristirici::Istek;
use std::path::{Component, Path, PathBuf};

pub fn yanit(kok: &Path, istek: &Istek) -> Vec<u8> {
    if istek.yontem != "GET" {
        return paketle(405, "Method Not Allowed", b"yalnizca GET\n");
    }
    match guvenli_yol(kok, &istek.hedef) {
        None => paketle(403, "Forbidden", b"izin yok\n"),
        Some(yol) => match std::fs::read(&yol) {
            Ok(icerik) => paketle(200, "OK", &icerik),
            Err(_) => paketle(404, "Not Found", b"bulunamadi\n"),
        },
    }
}

/// Dizin geçişi (path traversal) saldırısına karşı: ".." bileşenlerini ve mutlak yolları reddet
pub fn guvenli_yol(kok: &Path, hedef: &str) -> Option<PathBuf> {
    let yol = hedef.split('?').next()?;
    let goreli = yol.strip_prefix('/')?;
    let goreli = if goreli.is_empty() { "index.html" } else { goreli };
    let p = Path::new(goreli);
    if p.components().any(|c| !matches!(c, Component::Normal(_))) {
        return None; // "..", "/", "." gibi bileşenler
    }
    Some(kok.join(p))
}

pub fn paketle(kod: u16, aciklama: &str, govde: &[u8]) -> Vec<u8> {
    let mut v = format!("HTTP/1.1 {kod} {aciklama}\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n", govde.len()).into_bytes();
    v.extend_from_slice(govde);
    v
}
