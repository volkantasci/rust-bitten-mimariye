//! Ayrıştırıcı (parser): belirteç dizisi → AST. Deyimler için özyinelemeli iniş (her dil
//! bilgisi kuralına bir fonksiyon), ifadeler için Pratt ayrıştırma (yukarıdan aşağı operatör
//! önceliği): her ikili operatörün bir bağlama gücü vardır; güçlü operatör işlenenini önce bağlar.
use crate::ast::{Deyim, IkiliOp, Ifade, TekliOp, s_ifade};
use crate::lexer::{Token, Tur};

pub struct Ayristirici {
    tokenler: Vec<Token>,
    konum: usize,
    iz: Option<Vec<String>>, // Pratt adımlarının izi (isteğe bağlı, gösterim için)
    derinlik: usize,
}

type Sonuc<T> = Result<T, String>;

/// İkili operatörün bağlama gücü: büyük olan önce bağlar. Aynı güçtekiler sola birleşir (soldan gruplanır).
fn baglama_gucu(tur: &Tur) -> Option<(IkiliOp, u8)> {
    Some(match tur {
        Tur::VeyaVeya => (IkiliOp::Veya, 1),
        Tur::VeVe => (IkiliOp::Ve, 2),
        Tur::EsitEsit => (IkiliOp::Esit, 3),
        Tur::UnlemEsit => (IkiliOp::EsitDegil, 3),
        Tur::Kucuk => (IkiliOp::Kucuk, 4),
        Tur::KucukEsit => (IkiliOp::KucukEsit, 4),
        Tur::Buyuk => (IkiliOp::Buyuk, 4),
        Tur::BuyukEsit => (IkiliOp::BuyukEsit, 4),
        Tur::Arti => (IkiliOp::Topla, 5),
        Tur::Eksi => (IkiliOp::Cikar, 5),
        Tur::Yildiz => (IkiliOp::Carp, 6),
        Tur::Bolu => (IkiliOp::Bol, 6),
        Tur::Yuzde => (IkiliOp::Mod, 6),
        _ => return None,
    })
}

impl Ayristirici {
    pub fn yeni(tokenler: Vec<Token>) -> Self {
        Ayristirici { tokenler, konum: 0, iz: None, derinlik: 0 }
    }

    fn izle(&mut self, satir: String) {
        let girinti = "  ".repeat(self.derinlik);
        if let Some(iz) = &mut self.iz {
            iz.push(format!("{girinti}{satir}"));
        }
    }

    fn bak(&self) -> &Token {
        &self.tokenler[self.konum]
    }
    fn bak_sonraki(&self) -> &Tur {
        &self.tokenler[(self.konum + 1).min(self.tokenler.len() - 1)].tur
    }
    fn ilerle(&mut self) -> Token {
        let t = self.tokenler[self.konum].clone();
        if t.tur != Tur::Son {
            self.konum += 1;
        }
        t
    }
    fn hata<T>(&self, beklenen: &str) -> Sonuc<T> {
        let t = self.bak();
        Err(format!("satır {}, sütun {}: {beklenen} bekleniyordu, {:?} bulundu", t.satir, t.sutun, t.tur))
    }
    fn bekle(&mut self, tur: Tur, aciklama: &str) -> Sonuc<()> {
        if self.bak().tur == tur {
            self.ilerle();
            Ok(())
        } else {
            self.hata(aciklama)
        }
    }
    fn ad_bekle(&mut self) -> Sonuc<String> {
        match self.bak().tur.clone() {
            Tur::Ad(a) => {
                self.ilerle();
                Ok(a)
            }
            _ => self.hata("bir ad"),
        }
    }

    pub fn program(&mut self) -> Sonuc<Vec<Deyim>> {
        let mut deyimler = Vec::new();
        while self.bak().tur != Tur::Son {
            deyimler.push(self.deyim()?);
        }
        Ok(deyimler)
    }

    fn blok(&mut self) -> Sonuc<Vec<Deyim>> {
        self.bekle(Tur::SolSuslu, "'{'")?;
        let mut deyimler = Vec::new();
        while self.bak().tur != Tur::SagSuslu {
            if self.bak().tur == Tur::Son {
                return self.hata("'}'");
            }
            deyimler.push(self.deyim()?);
        }
        self.ilerle();
        Ok(deyimler)
    }

    /// Özyinelemeli iniş: ilk belirteç hangi deyim kuralının uygulanacağını seçer.
    fn deyim(&mut self) -> Sonuc<Deyim> {
        match self.bak().tur.clone() {
            Tur::Fn => {
                self.ilerle();
                let ad = self.ad_bekle()?;
                self.bekle(Tur::SolParantez, "'('")?;
                let mut parametreler = Vec::new();
                while self.bak().tur != Tur::SagParantez {
                    parametreler.push(self.ad_bekle()?);
                    if self.bak().tur == Tur::Virgul {
                        self.ilerle();
                    } else {
                        break;
                    }
                }
                self.bekle(Tur::SagParantez, "')'")?;
                Ok(Deyim::Fonksiyon { ad, parametreler, govde: self.blok()? })
            }
            Tur::Let => {
                self.ilerle();
                let ad = self.ad_bekle()?;
                self.bekle(Tur::Esit, "'='")?;
                let i = self.ifade(0)?;
                self.bekle(Tur::NoktaliVirgul, "';'")?;
                Ok(Deyim::Let(ad, i))
            }
            Tur::Ad(ad) if *self.bak_sonraki() == Tur::Esit => {
                self.ilerle();
                self.ilerle();
                let i = self.ifade(0)?;
                self.bekle(Tur::NoktaliVirgul, "';'")?;
                Ok(Deyim::Atama(ad, i))
            }
            Tur::If => {
                self.ilerle();
                let kosul = self.ifade(0)?;
                let evet = self.blok()?;
                let hayir = if self.bak().tur == Tur::Else {
                    self.ilerle();
                    if self.bak().tur == Tur::If { vec![self.deyim()?] } else { self.blok()? }
                } else {
                    Vec::new()
                };
                Ok(Deyim::If(kosul, evet, hayir))
            }
            Tur::While => {
                self.ilerle();
                let kosul = self.ifade(0)?;
                Ok(Deyim::While(kosul, self.blok()?))
            }
            Tur::Return => {
                self.ilerle();
                let deger = if self.bak().tur == Tur::NoktaliVirgul { None } else { Some(self.ifade(0)?) };
                self.bekle(Tur::NoktaliVirgul, "';'")?;
                Ok(Deyim::Return(deger))
            }
            Tur::Print => {
                self.ilerle();
                self.bekle(Tur::SolParantez, "'('")?;
                let i = self.ifade(0)?;
                self.bekle(Tur::SagParantez, "')'")?;
                self.bekle(Tur::NoktaliVirgul, "';'")?;
                Ok(Deyim::Print(i))
            }
            _ => {
                let i = self.ifade(0)?;
                self.bekle(Tur::NoktaliVirgul, "';'")?;
                Ok(Deyim::Ifade(i))
            }
        }
    }

    /// Pratt: bağlama gücü `en_az`'dan büyük operatörleri topla; küçük ya da eşitse dur ve
    /// o ana kadar kurulan ağacı çağırana bırak (aynı güçtekiler böylece sola birleşir).
    fn ifade(&mut self, en_az: u8) -> Sonuc<Ifade> {
        self.izle(format!("ifade(en_az = {en_az})"));
        self.derinlik += 1;
        let mut sol = self.onek()?;
        self.izle(format!("önek: {}", s_ifade(&sol)));
        while let Some((op, guc)) = baglama_gucu(&self.bak().tur) {
            if guc <= en_az {
                self.izle(format!("'{}' gücü {guc} ≤ {en_az}: dur", op.simge()));
                break;
            }
            self.izle(format!("'{}' gücü {guc} > {en_az}: sağ işleneni oku", op.simge()));
            self.ilerle();
            let sag = self.ifade(guc)?;
            sol = Ifade::Ikili(Box::new(sol), op, Box::new(sag));
            self.izle(format!("kuruldu: {}", s_ifade(&sol)));
        }
        self.derinlik -= 1;
        Ok(sol)
    }

    /// Önek konumundaki şeyler: sayı, mantıksal değer, ad, çağrı, parantez, tekli operatör.
    fn onek(&mut self) -> Sonuc<Ifade> {
        let t = self.bak().clone();
        match t.tur {
            Tur::Sayi(n) => {
                self.ilerle();
                Ok(Ifade::Sayi(n))
            }
            Tur::True => {
                self.ilerle();
                Ok(Ifade::Mantiksal(true))
            }
            Tur::False => {
                self.ilerle();
                Ok(Ifade::Mantiksal(false))
            }
            // Tekli operatörler bütün ikililerden güçlü bağlar: -a * b = (-a) * b
            Tur::Eksi => {
                self.ilerle();
                Ok(Ifade::Tekli(TekliOp::Eksi, Box::new(self.ifade(7)?)))
            }
            Tur::Unlem => {
                self.ilerle();
                Ok(Ifade::Tekli(TekliOp::Degil, Box::new(self.ifade(7)?)))
            }
            Tur::SolParantez => {
                self.ilerle();
                let i = self.ifade(0)?;
                self.bekle(Tur::SagParantez, "')'")?;
                Ok(i)
            }
            Tur::Ad(ad) => {
                self.ilerle();
                if self.bak().tur != Tur::SolParantez {
                    return Ok(Ifade::Degisken(ad));
                }
                self.ilerle();
                let mut argumanlar = Vec::new();
                while self.bak().tur != Tur::SagParantez {
                    argumanlar.push(self.ifade(0)?);
                    if self.bak().tur == Tur::Virgul {
                        self.ilerle();
                    } else {
                        break;
                    }
                }
                self.bekle(Tur::SagParantez, "')'")?;
                Ok(Ifade::Cagri(ad, argumanlar))
            }
            _ => self.hata("bir ifade"),
        }
    }
}

pub fn ayristir(kaynak: &str) -> Result<Vec<Deyim>, String> {
    let tokenler = crate::lexer::tokenlere_ayir(kaynak)?;
    Ayristirici::yeni(tokenler).program()
}

/// Ayrıştırır ve Pratt adımlarının izini de döndürür.
pub fn izleyerek_ayristir(kaynak: &str) -> Result<(Vec<Deyim>, Vec<String>), String> {
    let mut a = Ayristirici::yeni(crate::lexer::tokenlere_ayir(kaynak)?);
    a.iz = Some(Vec::new());
    let program = a.program()?;
    Ok((program, a.iz.take().unwrap()))
}

#[cfg(test)]
mod testler {
    use super::*;

    fn ifade(kaynak: &str) -> String {
        match ayristir(kaynak).unwrap().remove(0) {
            Deyim::Ifade(i) => s_ifade(&i),
            d => panic!("ifade bekleniyordu: {d:?}"),
        }
    }

    #[test]
    fn oncelik_ve_sola_birlesme() {
        assert_eq!(ifade("1 + 2 * 3 - 4;"), "(- (+ 1 (* 2 3)) 4)");
        assert_eq!(ifade("a < b && !c || d;"), "(|| (&& (< a b) (! c)) d)");
        assert_eq!(ifade("-a * b;"), "(* (- a) b)");
    }

    #[test]
    fn parantez_onceligi_degistirir() {
        assert_eq!(ifade("(1 + 2) * 3;"), "(* (+ 1 2) 3)");
    }

    #[test]
    fn hata_konumu_bildirilir() {
        let h = ayristir("let x = (1 + 2;").unwrap_err();
        assert_eq!(h, "satır 1, sütun 15: ')' bekleniyordu, NoktaliVirgul bulundu");
    }
}
