//! Bayt kod derleyicisi (128. adım): AST'yi yığın makinesi komutlarına çevirir. Adlar
//! derleme anında çözülür: her yerel değişkene bir yuva numarası verilir. İleri atlamaların
//! hedefi henüz bilinmediği için önce yer tutucuyla yazılır, hedef belli olunca geri
//! dönülüp doldurulur (geri yamalama).
use crate::ast::{Deyim, IkiliOp, Ifade, TekliOp};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Komut {
    Sabit(i64),
    Dogru,
    Yanlis,
    YereliOku(u16),
    YereleYaz(u16),
    Ikili(IkiliOp),
    Eksi,
    Degil,
    Atla(u32),
    YanlissaAtla(u32),
    VeAtla(u32),   // tepe false ise atla (değeri bırak), değilse at
    VeyaAtla(u32), // tepe true ise atla (değeri bırak), değilse at
    Cagir(u16, u8),
    Don,
    DonBos,
    Yazdir,
    At,
}

#[derive(Debug, Default)]
pub struct Fonksiyon {
    pub ad: String,
    pub parametre_sayisi: usize,
    pub yerel_sayisi: usize,
    pub kod: Vec<Komut>,
}

#[derive(Debug)]
pub struct Program {
    pub fonksiyonlar: Vec<Fonksiyon>,
    pub ana: usize,
}

struct FonksiyonDerleyici<'a> {
    hedef: Fonksiyon,
    kapsamlar: Vec<HashMap<String, u16>>,
    adlar: &'a HashMap<String, (u16, usize)>, // fonksiyon adı → (sıra no, parametre sayısı)
}

impl FonksiyonDerleyici<'_> {
    fn yaz(&mut self, k: Komut) -> usize {
        self.hedef.kod.push(k);
        self.hedef.kod.len() - 1
    }
    /// Geri yamalama: `yer`deki atlama komutunun hedefini şimdiki konuma ayarla.
    fn yamala(&mut self, yer: usize) {
        let hedef = self.hedef.kod.len() as u32;
        match &mut self.hedef.kod[yer] {
            Komut::Atla(h) | Komut::YanlissaAtla(h) | Komut::VeAtla(h) | Komut::VeyaAtla(h) => *h = hedef,
            _ => unreachable!(),
        }
    }
    fn yeni_yerel(&mut self, ad: &str) -> u16 {
        let yuva = self.hedef.yerel_sayisi as u16;
        self.hedef.yerel_sayisi += 1;
        self.kapsamlar.last_mut().unwrap().insert(ad.to_string(), yuva);
        yuva
    }
    fn coz(&self, ad: &str) -> Result<u16, String> {
        self.kapsamlar.iter().rev().find_map(|k| k.get(ad).copied()).ok_or_else(|| format!("tanımsız değişken: {ad}"))
    }

    fn blok(&mut self, deyimler: &[Deyim]) -> Result<(), String> {
        self.kapsamlar.push(HashMap::new());
        for d in deyimler { self.deyim(d)?; }
        self.kapsamlar.pop();
        Ok(())
    }

    fn deyim(&mut self, d: &Deyim) -> Result<(), String> {
        match d {
            Deyim::Fonksiyon { ad, .. } => return Err(format!("iç içe fonksiyon desteklenmiyor: {ad}")),
            Deyim::Let(ad, i) => {
                self.ifade(i)?;
                let yuva = self.yeni_yerel(ad);
                self.yaz(Komut::YereleYaz(yuva));
            }
            Deyim::Atama(ad, i) => {
                self.ifade(i)?;
                let yuva = self.coz(ad)?;
                self.yaz(Komut::YereleYaz(yuva));
            }
            Deyim::If(k, evet, hayir) => {
                self.ifade(k)?;
                let atla_hayir = self.yaz(Komut::YanlissaAtla(0));
                self.blok(evet)?;
                let atla_son = self.yaz(Komut::Atla(0));
                self.yamala(atla_hayir);
                self.blok(hayir)?;
                self.yamala(atla_son);
            }
            Deyim::While(k, govde) => {
                let bas = self.hedef.kod.len() as u32;
                self.ifade(k)?;
                let cikis = self.yaz(Komut::YanlissaAtla(0));
                self.blok(govde)?;
                self.yaz(Komut::Atla(bas));
                self.yamala(cikis);
            }
            Deyim::Return(Some(i)) => {
                self.ifade(i)?;
                self.yaz(Komut::Don);
            }
            Deyim::Return(None) => { self.yaz(Komut::DonBos); }
            Deyim::Print(i) => {
                self.ifade(i)?;
                self.yaz(Komut::Yazdir);
            }
            Deyim::Ifade(i) => {
                self.ifade(i)?;
                self.yaz(Komut::At);
            }
        }
        Ok(())
    }

    fn ifade(&mut self, i: &Ifade) -> Result<(), String> {
        match i {
            Ifade::Sayi(n) => { self.yaz(Komut::Sabit(*n)); }
            Ifade::Mantiksal(true) => { self.yaz(Komut::Dogru); }
            Ifade::Mantiksal(false) => { self.yaz(Komut::Yanlis); }
            Ifade::Degisken(ad) => {
                let yuva = self.coz(ad)?;
                self.yaz(Komut::YereliOku(yuva));
            }
            Ifade::Tekli(op, x) => {
                self.ifade(x)?;
                self.yaz(if *op == TekliOp::Eksi { Komut::Eksi } else { Komut::Degil });
            }
            Ifade::Ikili(a, IkiliOp::Ve, b) => {
                self.ifade(a)?;
                let atla = self.yaz(Komut::VeAtla(0));
                self.ifade(b)?;
                self.yamala(atla);
            }
            Ifade::Ikili(a, IkiliOp::Veya, b) => {
                self.ifade(a)?;
                let atla = self.yaz(Komut::VeyaAtla(0));
                self.ifade(b)?;
                self.yamala(atla);
            }
            Ifade::Ikili(a, op, b) => {
                self.ifade(a)?;
                self.ifade(b)?;
                self.yaz(Komut::Ikili(*op));
            }
            Ifade::Cagri(ad, argumanlar) => {
                let (no, parametre) = *self.adlar.get(ad).ok_or_else(|| format!("tanımsız fonksiyon: {ad}"))?;
                if parametre != argumanlar.len() {
                    return Err(format!("{ad}: {parametre} argüman bekleniyordu, {} verildi", argumanlar.len()));
                }
                for a in argumanlar { self.ifade(a)?; }
                self.yaz(Komut::Cagir(no, argumanlar.len() as u8));
            }
        }
        Ok(())
    }
}

pub fn derle(program: &[Deyim]) -> Result<Program, String> {
    // 1. geçiş: fonksiyon adlarını topla (tanımdan önce çağrılabilsin)
    let mut adlar = HashMap::new();
    let mut tanimlar = Vec::new();
    for d in program {
        if let Deyim::Fonksiyon { ad, parametreler, govde } = d {
            adlar.insert(ad.clone(), (tanimlar.len() as u16, parametreler.len()));
            tanimlar.push((ad, parametreler, govde));
        }
    }
    // 2. geçiş: her fonksiyonu derle
    let mut fonksiyonlar = Vec::new();
    for (ad, parametreler, govde) in tanimlar {
        let mut fd = FonksiyonDerleyici { hedef: Fonksiyon { ad: ad.clone(), parametre_sayisi: parametreler.len(), ..Default::default() }, kapsamlar: vec![HashMap::new()], adlar: &adlar };
        for p in parametreler { fd.yeni_yerel(p); }
        for d in govde { fd.deyim(d)?; }
        fd.yaz(Komut::DonBos);
        fonksiyonlar.push(fd.hedef);
    }
    // Üst düzey deyimler: gizli "<ana>" fonksiyonu
    let mut ana = FonksiyonDerleyici { hedef: Fonksiyon { ad: "<ana>".into(), ..Default::default() }, kapsamlar: vec![HashMap::new()], adlar: &adlar };
    for d in program {
        if !matches!(d, Deyim::Fonksiyon { .. }) { ana.deyim(d)?; }
    }
    ana.yaz(Komut::DonBos);
    fonksiyonlar.push(ana.hedef);
    let ana = fonksiyonlar.len() - 1;
    Ok(Program { fonksiyonlar, ana })
}

pub fn listele(p: &Program) -> String {
    let mut s = String::new();
    for f in &p.fonksiyonlar {
        s.push_str(&format!("{} ({} parametre, {} yerel yuva):\n", f.ad, f.parametre_sayisi, f.yerel_sayisi));
        for (i, k) in f.kod.iter().enumerate() {
            s.push_str(&format!("  {i:04}  {k:?}\n"));
        }
    }
    s
}
