//! Ayrıştırıcı: token dizisi → AST. Deyimler için özyinelemeli iniş,
//! ifadeler için Pratt ayrıştırma (operatör öncelikleri "bağlanma gücü" ile).
use crate::ast::{Deyim, IkiliOp, Ifade, TekliOp};
use crate::lexer::{Token, Tur};

pub struct Ayristirici {
    tokenler: Vec<Token>,
    konum: usize,
}

type Sonuc<T> = Result<T, String>;

impl Ayristirici {
    pub fn yeni(tokenler: Vec<Token>) -> Self {
        Ayristirici { tokenler, konum: 0 }
    }

    fn bak(&self) -> &Token { &self.tokenler[self.konum] }
    fn bak_sonraki(&self) -> &Tur { &self.tokenler[(self.konum + 1).min(self.tokenler.len() - 1)].tur }
    fn ilerle(&mut self) -> Token {
        let t = self.tokenler[self.konum].clone();
        if t.tur != Tur::Son { self.konum += 1; }
        t
    }
    fn hata<T>(&self, beklenen: &str) -> Sonuc<T> {
        let t = self.bak();
        Err(format!("satır {}, sütun {}: {beklenen} bekleniyordu, {:?} bulundu", t.satir, t.sutun, t.tur))
    }
    fn bekle(&mut self, tur: Tur, aciklama: &str) -> Sonuc<()> {
        if self.bak().tur == tur { self.ilerle(); Ok(()) } else { self.hata(aciklama) }
    }
    fn ad_bekle(&mut self) -> Sonuc<String> {
        match self.bak().tur.clone() {
            Tur::Ad(a) => { self.ilerle(); Ok(a) }
            _ => self.hata("bir ad"),
        }
    }

    pub fn program(&mut self) -> Sonuc<Vec<Deyim>> {
        let mut deyimler = Vec::new();
        while self.bak().tur != Tur::Son { deyimler.push(self.deyim()?); }
        Ok(deyimler)
    }

    fn blok(&mut self) -> Sonuc<Vec<Deyim>> {
        self.bekle(Tur::SolSuslu, "'{'")?;
        let mut deyimler = Vec::new();
        while self.bak().tur != Tur::SagSuslu {
            if self.bak().tur == Tur::Son { return self.hata("'}'"); }
            deyimler.push(self.deyim()?);
        }
        self.ilerle();
        Ok(deyimler)
    }

    fn deyim(&mut self) -> Sonuc<Deyim> {
        match self.bak().tur.clone() {
            Tur::Fn => {
                self.ilerle();
                let ad = self.ad_bekle()?;
                self.bekle(Tur::SolParantez, "'('")?;
                let mut parametreler = Vec::new();
                while self.bak().tur != Tur::SagParantez {
                    parametreler.push(self.ad_bekle()?);
                    if self.bak().tur == Tur::Virgul { self.ilerle(); } else { break; }
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

    /// Pratt: sol bağlanma gücü `en_az`'dan büyük operatörleri topla
    fn ifade(&mut self, en_az: u8) -> Sonuc<Ifade> {
        let mut sol = self.onek()?;
        loop {
            let (op, guc) = match self.bak().tur {
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
                _ => break,
            };
            if guc <= en_az { break; }
            self.ilerle();
            let sag = self.ifade(guc)?; // sola bağlı: aynı güçteki operatör sağa geçmez
            sol = Ifade::Ikili(Box::new(sol), op, Box::new(sag));
        }
        Ok(sol)
    }

    fn onek(&mut self) -> Sonuc<Ifade> {
        let t = self.bak().clone();
        match t.tur {
            Tur::Sayi(n) => { self.ilerle(); Ok(Ifade::Sayi(n)) }
            Tur::True => { self.ilerle(); Ok(Ifade::Mantiksal(true)) }
            Tur::False => { self.ilerle(); Ok(Ifade::Mantiksal(false)) }
            Tur::Eksi => { self.ilerle(); Ok(Ifade::Tekli(TekliOp::Eksi, Box::new(self.ifade(7)?))) }
            Tur::Unlem => { self.ilerle(); Ok(Ifade::Tekli(TekliOp::Degil, Box::new(self.ifade(7)?))) }
            Tur::SolParantez => {
                self.ilerle();
                let i = self.ifade(0)?;
                self.bekle(Tur::SagParantez, "')'")?;
                Ok(i)
            }
            Tur::Ad(ad) => {
                self.ilerle();
                if self.bak().tur == Tur::SolParantez {
                    self.ilerle();
                    let mut argumanlar = Vec::new();
                    while self.bak().tur != Tur::SagParantez {
                        argumanlar.push(self.ifade(0)?);
                        if self.bak().tur == Tur::Virgul { self.ilerle(); } else { break; }
                    }
                    self.bekle(Tur::SagParantez, "')'")?;
                    Ok(Ifade::Cagri(ad, argumanlar))
                } else {
                    Ok(Ifade::Degisken(ad))
                }
            }
            _ => self.hata("bir ifade"),
        }
    }
}

pub fn ayristir(kaynak: &str) -> Result<Vec<Deyim>, String> {
    let tokenler = crate::lexer::tokenlere_ayir(kaynak)?;
    Ayristirici::yeni(tokenler).program()
}
