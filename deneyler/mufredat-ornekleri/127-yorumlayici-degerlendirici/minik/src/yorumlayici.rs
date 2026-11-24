//! Ağaç dolaşan yorumlayıcı (127. adım): AST düğümlerini doğrudan, özyinelemeli olarak
//! değerlendirir. Değişkenler bir kapsam yığınında (her blok bir HashMap) adla aranır;
//! her fonksiyon çağrısı yeni bir kapsam yığınıyla başlar (yalnız parametrelerini görür).
use crate::ast::{Deyim, IkiliOp, Ifade, TekliOp};
use std::collections::HashMap;
use std::fmt;
use std::io::Write;

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

/// İkili işlemin anlamı: taşma ve sıfıra bölme çalışma zamanı hatasıdır (sessizce sarmaz).
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

/// Normal akışın dışına çıkış: `return` (değeriyle) ya da çalışma zamanı hatası.
/// İkisi de `?` ile çağrı yığınında yukarı taşınır.
enum Kesinti {
    Donus(Deger),
    Hata(String),
}

impl From<String> for Kesinti {
    fn from(s: String) -> Self {
        Kesinti::Hata(s)
    }
}

/// Varsayılan çağrı derinliği sınırı. Minik'teki her çağrı, Rust'ta birkaç iç içe fonksiyon
/// çağrısı (yığın çerçevesi) demektir; sınır, yorumlayıcının yığınına rahatça sığmalı.
pub const AZAMI_DERINLIK: usize = 2_000;
/// Yorumlayıcının çalıştığı iş parçacığının yığın boyutu (ana iş parçacığında Linux'ta
/// genellikle 8 MiB, test iş parçacıklarında 2 MiB olurdu; burada biz seçiyoruz).
pub const YIGIN_BOYUTU: usize = 64 * 1024 * 1024;

pub struct Yorumlayici<'p, 'c> {
    fonksiyonlar: HashMap<&'p str, (&'p [String], &'p [Deyim])>,
    kapsamlar: Vec<HashMap<String, Deger>>, // o anki çağrının kapsam yığını
    derinlik: usize,
    azami_derinlik: usize,
    cikti: &'c mut (dyn Write + Send),
}

/// Programı, yığın boyutu bilinen ayrı bir iş parçacığında çalıştırır; `print` çıktısı
/// `cikti`ya yazılır.
pub fn calistir(program: &[Deyim], azami_derinlik: usize, cikti: &mut (dyn Write + Send)) -> Result<(), String> {
    std::thread::scope(|kapsam| {
        std::thread::Builder::new()
            .name("yorumlayici".into())
            .stack_size(YIGIN_BOYUTU)
            .spawn_scoped(kapsam, || burada_calistir(program, azami_derinlik, cikti))
            .map_err(|h| h.to_string())?
            .join()
            .map_err(|_| "yorumlayıcı iş parçacığı paniğe düştü".to_string())?
    })
}

fn burada_calistir(program: &[Deyim], azami_derinlik: usize, cikti: &mut (dyn Write + Send)) -> Result<(), String> {
    let mut y = Yorumlayici { fonksiyonlar: HashMap::new(), kapsamlar: vec![HashMap::new()], derinlik: 0, azami_derinlik, cikti };
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
    Ok(())
}

impl<'p> Yorumlayici<'p, '_> {
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
                let v = match i {
                    Some(i) => self.ifade(i)?,
                    None => Deger::Bos,
                };
                Err(Kesinti::Donus(v))
            }
            Deyim::Print(i) => {
                let v = self.ifade(i)?;
                writeln!(self.cikti, "{v}").map_err(|h| Kesinti::Hata(h.to_string()))
            }
            Deyim::Ifade(i) => self.ifade(i).map(|_| ()),
        }
    }

    fn ifade(&mut self, i: &'p Ifade) -> Result<Deger, Kesinti> {
        Ok(match i {
            Ifade::Sayi(n) => Deger::Sayi(*n),
            Ifade::Mantiksal(b) => Deger::Mantiksal(*b),
            // Ad, kapsam yığınında içten dışa aranır (her adımda bir HashMap araması)
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
            // Kısa devre: sol taraf sonucu belirliyorsa sağ taraf hiç değerlendirilmez
            Ifade::Ikili(a, IkiliOp::Ve, b) => match self.ifade(a)? {
                Deger::Mantiksal(false) => Deger::Mantiksal(false),
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
            Ifade::Cagri(ad, argumanlar) => self.cagir(ad, argumanlar)?,
        })
    }

    fn cagir(&mut self, ad: &str, argumanlar: &'p [Ifade]) -> Result<Deger, Kesinti> {
        let (parametreler, govde) = *self
            .fonksiyonlar
            .get(ad)
            .ok_or_else(|| Kesinti::Hata(format!("tanımsız fonksiyon: {ad}")))?;
        if parametreler.len() != argumanlar.len() {
            return Err(Kesinti::Hata(format!("{ad}: {} argüman bekleniyordu, {} verildi", parametreler.len(), argumanlar.len())));
        }
        let mut yeni = HashMap::new();
        for (p, a) in parametreler.iter().zip(argumanlar) {
            let v = self.ifade(a)?;
            yeni.insert(p.clone(), v);
        }
        if self.derinlik >= self.azami_derinlik {
            return Err(Kesinti::Hata(format!("çağrı derinliği sınırı aşıldı ({})", self.azami_derinlik)));
        }
        // Yeni çağrı çerçevesi: yalnız parametrelerini gören yeni bir kapsam yığını
        let eski = std::mem::replace(&mut self.kapsamlar, vec![yeni]);
        self.derinlik += 1;
        let sonuc = govde.iter().try_for_each(|d| self.deyim(d));
        self.derinlik -= 1;
        self.kapsamlar = eski;
        match sonuc {
            Ok(()) => Ok(Deger::Bos),
            Err(Kesinti::Donus(v)) => Ok(v),
            Err(h) => Err(h),
        }
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    fn calistir_metin(kaynak: &str) -> Result<String, String> {
        let program = crate::parser::ayristir(kaynak)?;
        let mut cikti = Vec::new();
        calistir(&program, AZAMI_DERINLIK, &mut cikti)?;
        Ok(String::from_utf8(cikti).unwrap())
    }

    #[test]
    fn tasma_sessizce_sarmaz() {
        assert!(ikili(IkiliOp::Carp, Deger::Sayi(i64::MAX), Deger::Sayi(2)).is_err());
    }

    #[test]
    fn kapsam_ve_ozyineleme() {
        let kaynak = "fn f(n) { if n < 2 { return n; } return f(n - 1) + f(n - 2); } let x = 1; print(f(10)); print(x);";
        assert_eq!(calistir_metin(kaynak), Ok("55\n1\n".into()));
    }

    #[test]
    fn kisa_devre_sag_tarafi_degerlendirmez() {
        assert_eq!(calistir_metin("print(false && 1 / 0 == 0);"), Ok("false\n".into()));
        assert_eq!(calistir_metin("print(true && 1 / 0 == 0);"), Err("çalışma hatası: sıfıra bölme".into()));
    }

    #[test]
    fn derinlik_siniri() {
        let h = calistir_metin("fn s(n) { return s(n + 1); } print(s(0));").unwrap_err();
        assert_eq!(h, "çalışma hatası: çağrı derinliği sınırı aşıldı (2000)");
    }
}
