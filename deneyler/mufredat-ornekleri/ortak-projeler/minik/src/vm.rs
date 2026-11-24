//! Yığın makinesi (VM, 128. adım): bayt kodu tek bir döngüde yürütür. Değerler bir değer
//! yığınında, çağrılar çağrı çerçevesi yığınında durur; ikisi de öbekteki birer Vec'tir,
//! bu yüzden Minik'teki özyineleme Rust'ın yığınını büyütmez. Değişkenlere adla değil,
//! derleme anında atanmış yuva numarasıyla (dizi indisi) erişilir.
use crate::derleyici::{Komut, Program};
use crate::yorumlayici::{Deger, ikili};
use std::io::Write;

struct Cerceve {
    fonk: usize,
    ip: usize,    // komut işaretçisi: sıradaki komutun indisi
    taban: usize, // bu çağrının yerel yuvalarının değer yığınındaki başlangıcı
}

/// Programı yürütür; dönüş: yürütülen komut sayısı. Hata mesajları ağaç dolaşan
/// yorumlayıcınınkilerle aynıdır (iki motorun çıktısı karşılaştırılabilsin).
pub fn calistir(p: &Program, azami_derinlik: usize, cikti: &mut dyn Write) -> Result<u64, String> {
    let mut yigin: Vec<Deger> = vec![Deger::Bos; p.fonksiyonlar[p.ana].yerel_sayisi];
    let mut cerceveler = vec![Cerceve { fonk: p.ana, ip: 0, taban: 0 }];
    let mut adim = 0u64;
    let hata = |h: String| format!("çalışma hatası: {h}");
    loop {
        adim += 1;
        let c = cerceveler.last_mut().unwrap();
        let komut = p.fonksiyonlar[c.fonk].kod[c.ip];
        c.ip += 1;
        let taban = c.taban;
        match komut {
            Komut::Sabit(n) => yigin.push(Deger::Sayi(n)),
            Komut::Dogru => yigin.push(Deger::Mantiksal(true)),
            Komut::Yanlis => yigin.push(Deger::Mantiksal(false)),
            Komut::YereliOku(i) => yigin.push(yigin[taban + i as usize]),
            Komut::YereleYaz(i) => {
                let v = yigin.pop().unwrap();
                yigin[taban + i as usize] = v;
            }
            Komut::Ikili(op) => {
                let b = yigin.pop().unwrap();
                let a = yigin.pop().unwrap();
                yigin.push(ikili(op, a, b).map_err(hata)?);
            }
            Komut::Eksi => match yigin.pop().unwrap() {
                Deger::Sayi(n) => yigin.push(Deger::Sayi(n.checked_neg().ok_or_else(|| hata("aritmetik taşma".into()))?)),
                v => return Err(hata(format!("tip hatası: tekli işlem, {v}"))),
            },
            Komut::Degil => match yigin.pop().unwrap() {
                Deger::Mantiksal(b) => yigin.push(Deger::Mantiksal(!b)),
                v => return Err(hata(format!("tip hatası: tekli işlem, {v}"))),
            },
            Komut::Atla(h) => c.ip = h as usize,
            Komut::YanlissaAtla(h) => match yigin.pop().unwrap() {
                Deger::Mantiksal(false) => c.ip = h as usize,
                Deger::Mantiksal(true) => {}
                v => return Err(hata(format!("koşul mantıksal olmalı, {v} bulundu"))),
            },
            // Kısa devre: sol taraf sonucu belirliyorsa sağ tarafın komutlarının üstünden atla
            Komut::VeAtla(h) => match yigin.last().copied() {
                Some(Deger::Mantiksal(false)) => c.ip = h as usize,
                Some(Deger::Mantiksal(true)) => {
                    yigin.pop();
                }
                v => return Err(hata(format!("&& için mantıksal değer gerekir, {} bulundu", v.unwrap_or(Deger::Bos)))),
            },
            Komut::VeyaAtla(h) => match yigin.last().copied() {
                Some(Deger::Mantiksal(true)) => c.ip = h as usize,
                Some(Deger::Mantiksal(false)) => {
                    yigin.pop();
                }
                v => return Err(hata(format!("|| için mantıksal değer gerekir, {} bulundu", v.unwrap_or(Deger::Bos)))),
            },
            Komut::Cagir(no, arg) => {
                let hedef = &p.fonksiyonlar[no as usize];
                if cerceveler.len() > azami_derinlik {
                    return Err(hata(format!("çağrı derinliği sınırı aşıldı ({azami_derinlik})")));
                }
                let yeni_taban = yigin.len() - arg as usize; // argümanlar ilk yerel yuvalar olur
                yigin.resize(yeni_taban + hedef.yerel_sayisi, Deger::Bos);
                cerceveler.push(Cerceve { fonk: no as usize, ip: 0, taban: yeni_taban });
            }
            Komut::Don | Komut::DonBos => {
                let v = if komut == Komut::Don { yigin.pop().unwrap() } else { Deger::Bos };
                let biten = cerceveler.pop().unwrap();
                yigin.truncate(biten.taban);
                if cerceveler.is_empty() {
                    return Ok(adim);
                }
                yigin.push(v);
            }
            Komut::Yazdir => {
                let v = yigin.pop().unwrap();
                writeln!(cikti, "{v}").map_err(|h| h.to_string())?;
            }
            Komut::At => {
                yigin.pop();
            }
        }
    }
}
