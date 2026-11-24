//! Yığın makinesi: komutları tek bir döngüde, açık bir değer yığını ve çağrı çerçeveleriyle yürütür.
use crate::derleyici::{Komut, Program};
use crate::yorumlayici::{Deger, ikili};

struct Cerceve {
    fonk: usize,
    ip: usize,    // komut işaretçisi
    taban: usize, // bu çağrının yerel yuvalarının yığındaki başlangıcı
}

pub fn calistir(p: &Program) -> Result<u64, String> {
    let mut yigin: Vec<Deger> = vec![Deger::Bos; p.fonksiyonlar[p.ana].yerel_sayisi];
    let mut cerceveler = vec![Cerceve { fonk: p.ana, ip: 0, taban: 0 }];
    let mut adim = 0u64;
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
                yigin.push(ikili(op, a, b).map_err(|h| format!("çalışma hatası: {h}"))?);
            }
            Komut::Eksi => match yigin.pop().unwrap() {
                Deger::Sayi(n) => yigin.push(Deger::Sayi(n.checked_neg().ok_or("çalışma hatası: aritmetik taşma")?)),
                v => return Err(format!("çalışma hatası: tip hatası: tekli işlem, {v}")),
            },
            Komut::Degil => match yigin.pop().unwrap() {
                Deger::Mantiksal(b) => yigin.push(Deger::Mantiksal(!b)),
                v => return Err(format!("çalışma hatası: tip hatası: tekli işlem, {v}")),
            },
            Komut::Atla(h) => c.ip = h as usize,
            Komut::YanlissaAtla(h) => match yigin.pop().unwrap() {
                Deger::Mantiksal(false) => c.ip = h as usize,
                Deger::Mantiksal(true) => {}
                v => return Err(format!("çalışma hatası: koşul mantıksal olmalı, {v} bulundu")),
            },
            Komut::VeAtla(h) => match yigin.last() {
                Some(Deger::Mantiksal(false)) => c.ip = h as usize,
                Some(Deger::Mantiksal(true)) => { yigin.pop(); }
                _ => return Err("çalışma hatası: && için mantıksal değer gerekir".into()),
            },
            Komut::VeyaAtla(h) => match yigin.last() {
                Some(Deger::Mantiksal(true)) => c.ip = h as usize,
                Some(Deger::Mantiksal(false)) => { yigin.pop(); }
                _ => return Err("çalışma hatası: || için mantıksal değer gerekir".into()),
            },
            Komut::Cagir(no, arg) => {
                let hedef = &p.fonksiyonlar[no as usize];
                let yeni_taban = yigin.len() - arg as usize; // argümanlar ilk yerel yuvalar olur
                yigin.resize(yeni_taban + hedef.yerel_sayisi, Deger::Bos);
                if cerceveler.len() >= 2_000 {
                    return Err("çalışma hatası: çağrı derinliği sınırı aşıldı (2000)".into());
                }
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
            Komut::Yazdir => println!("{}", yigin.pop().unwrap()),
            Komut::At => { yigin.pop(); }
        }
    }
}
