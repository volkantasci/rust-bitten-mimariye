//! Ağaç yürüten yorumlayıcı: AST düğümlerini doğrudan, özyinelemeli olarak değerlendirir.
use crate::ast::{Deyim, IkiliOp, Ifade, TekliOp};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Deger {
    Sayi(i64),
    Mantiksal(bool),
    Bos,
}

impl fmt::Display for Deger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Deger::Sayi(n) => write!(f, "{n}"),
            Deger::Mantiksal(b) => write!(f, "{b}"),
            Deger::Bos => write!(f, "boş"),
        }
    }
}

/// İkili işlem: hem yorumlayıcı hem VM aynı anlamı kullanır (taşma ve sıfıra bölme hata)
pub fn ikili(op: IkiliOp, a: Deger, b: Deger) -> Result<Deger, String> {
    use Deger::*;
    use IkiliOp::*;
    let tasma = || format!("aritmetik taşma: {a} {} {b}", op.simge());
    Ok(match (op, a, b) {
        (Topla, Sayi(x), Sayi(y)) => Sayi(x.checked_add(y).ok_or_else(tasma)?),
        (Cikar, Sayi(x), Sayi(y)) => Sayi(x.checked_sub(y).ok_or_else(tasma)?),
        (Carp, Sayi(x), Sayi(y)) => Sayi(x.checked_mul(y).ok_or_else(tasma)?),
        (Bol | Mod, Sayi(_), Sayi(0)) => return Err("sıfıra bölme".into()),
        (Bol, Sayi(x), Sayi(y)) => Sayi(x.checked_div(y).ok_or_else(tasma)?),
        (Mod, Sayi(x), Sayi(y)) => Sayi(x.checked_rem(y).ok_or_else(tasma)?),
        (Kucuk, Sayi(x), Sayi(y)) => Mantiksal(x < y),
        (KucukEsit, Sayi(x), Sayi(y)) => Mantiksal(x <= y),
        (Buyuk, Sayi(x), Sayi(y)) => Mantiksal(x > y),
        (BuyukEsit, Sayi(x), Sayi(y)) => Mantiksal(x >= y),
        (Esit, x, y) => Mantiksal(x == y),
        (EsitDegil, x, y) => Mantiksal(x != y),
        _ => return Err(format!("tip hatası: {a} {} {b}", op.simge())),
    })
}

enum Kesinti {
    Donus(Deger),
    Hata(String),
}

impl From<String> for Kesinti {
    fn from(s: String) -> Self { Kesinti::Hata(s) }
}

pub struct Yorumlayici<'p> {
    fonksiyonlar: HashMap<&'p str, (&'p [String], &'p [Deyim])>,
    kapsamlar: Vec<HashMap<String, Deger>>, // o anki çağrının kapsam yığını
    derinlik: usize,
    pub dugum_sayisi: u64,
}

impl<'p> Yorumlayici<'p> {
    pub fn calistir(program: &'p [Deyim]) -> Result<u64, String> {
        let mut y = Yorumlayici { fonksiyonlar: HashMap::new(), kapsamlar: vec![HashMap::new()], derinlik: 0, dugum_sayisi: 0 };
        for d in program {
            if let Deyim::Fonksiyon { ad, parametreler, govde } = d {
                y.fonksiyonlar.insert(ad.as_str(), (parametreler.as_slice(), govde.as_slice()));
            }
        }
        for d in program {
            if !matches!(d, Deyim::Fonksiyon { .. }) {
                match y.deyim(d) {
                    Ok(()) => {}
                    Err(Kesinti::Donus(_)) => break,
                    Err(Kesinti::Hata(h)) => return Err(format!("çalışma hatası: {h}")),
                }
            }
        }
        Ok(y.dugum_sayisi)
    }

    fn blok(&mut self, deyimler: &'p [Deyim]) -> Result<(), Kesinti> {
        self.kapsamlar.push(HashMap::new());
        let sonuc = deyimler.iter().try_for_each(|d| self.deyim(d));
        self.kapsamlar.pop();
        sonuc
    }

    fn deyim(&mut self, d: &'p Deyim) -> Result<(), Kesinti> {
        match d {
            Deyim::Fonksiyon { ad, .. } => Err(Kesinti::Hata(format!("iç içe fonksiyon desteklenmiyor: {ad}"))),
            Deyim::Let(ad, i) => {
                let v = self.ifade(i)?;
                self.kapsamlar.last_mut().unwrap().insert(ad.clone(), v);
                Ok(())
            }
            Deyim::Atama(ad, i) => {
                let v = self.ifade(i)?;
                for k in self.kapsamlar.iter_mut().rev() {
                    if let Some(yer) = k.get_mut(ad) {
                        *yer = v;
                        return Ok(());
                    }
                }
                Err(Kesinti::Hata(format!("tanımsız değişken: {ad}")))
            }
            Deyim::If(k, evet, hayir) => match self.ifade(k)? {
                Deger::Mantiksal(true) => self.blok(evet),
                Deger::Mantiksal(false) => self.blok(hayir),
                v => Err(Kesinti::Hata(format!("if koşulu mantıksal olmalı, {v} bulundu"))),
            },
            Deyim::While(k, govde) => loop {
                match self.ifade(k)? {
                    Deger::Mantiksal(true) => self.blok(govde)?,
                    Deger::Mantiksal(false) => return Ok(()),
                    v => return Err(Kesinti::Hata(format!("while koşulu mantıksal olmalı, {v} bulundu"))),
                }
            },
            Deyim::Return(i) => {
                let v = match i { Some(i) => self.ifade(i)?, None => Deger::Bos };
                Err(Kesinti::Donus(v))
            }
            Deyim::Print(i) => {
                println!("{}", self.ifade(i)?);
                Ok(())
            }
            Deyim::Ifade(i) => self.ifade(i).map(|_| ()),
        }
    }

    fn ifade(&mut self, i: &'p Ifade) -> Result<Deger, Kesinti> {
        self.dugum_sayisi += 1;
        Ok(match i {
            Ifade::Sayi(n) => Deger::Sayi(*n),
            Ifade::Mantiksal(b) => Deger::Mantiksal(*b),
            Ifade::Degisken(ad) => *self
                .kapsamlar
                .iter()
                .rev()
                .find_map(|k| k.get(ad))
                .ok_or_else(|| Kesinti::Hata(format!("tanımsız değişken: {ad}")))?,
            Ifade::Tekli(op, x) => match (op, self.ifade(x)?) {
                (TekliOp::Eksi, Deger::Sayi(n)) => Deger::Sayi(n.checked_neg().ok_or_else(|| "aritmetik taşma".to_string())?),
                (TekliOp::Degil, Deger::Mantiksal(b)) => Deger::Mantiksal(!b),
                (_, v) => return Err(Kesinti::Hata(format!("tip hatası: tekli işlem, {v}"))),
            },
            Ifade::Ikili(a, IkiliOp::Ve, b) => match self.ifade(a)? {
                Deger::Mantiksal(false) => Deger::Mantiksal(false), // kısa devre
                Deger::Mantiksal(true) => self.ifade(b)?,
                v => return Err(Kesinti::Hata(format!("&& için mantıksal değer gerekir, {v} bulundu"))),
            },
            Ifade::Ikili(a, IkiliOp::Veya, b) => match self.ifade(a)? {
                Deger::Mantiksal(true) => Deger::Mantiksal(true),
                Deger::Mantiksal(false) => self.ifade(b)?,
                v => return Err(Kesinti::Hata(format!("|| için mantıksal değer gerekir, {v} bulundu"))),
            },
            Ifade::Ikili(a, op, b) => {
                let x = self.ifade(a)?;
                let y = self.ifade(b)?;
                ikili(*op, x, y)?
            }
            Ifade::Cagri(ad, argumanlar) => {
                let (parametreler, govde) = *self
                    .fonksiyonlar
                    .get(ad.as_str())
                    .ok_or_else(|| Kesinti::Hata(format!("tanımsız fonksiyon: {ad}")))?;
                if parametreler.len() != argumanlar.len() {
                    return Err(Kesinti::Hata(format!("{ad}: {} argüman bekleniyordu, {} verildi", parametreler.len(), argumanlar.len())));
                }
                let mut yeni = HashMap::new();
                for (p, a) in parametreler.iter().zip(argumanlar) {
                    let v = self.ifade(a)?;
                    yeni.insert(p.clone(), v);
                }
                if self.derinlik >= 2_000 {
                    return Err(Kesinti::Hata("çağrı derinliği sınırı aşıldı (2000)".into()));
                }
                // Yeni çağrı: yalnızca parametrelerini gören yeni bir kapsam yığını
                let eski = std::mem::replace(&mut self.kapsamlar, vec![yeni]);
                self.derinlik += 1;
                let sonuc = govde.iter().try_for_each(|d| self.deyim(d));
                self.derinlik -= 1;
                self.kapsamlar = eski;
                match sonuc {
                    Ok(()) => Deger::Bos,
                    Err(Kesinti::Donus(v)) => v,
                    Err(h) => return Err(h),
                }
            }
        })
    }
}
