//! Soyut sözdizimi ağacı: dilin yapısı Rust enum'larıyla (cebirsel veri tipleri)

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IkiliOp { Topla, Cikar, Carp, Bol, Mod, Esit, EsitDegil, Kucuk, KucukEsit, Buyuk, BuyukEsit, Ve, Veya }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TekliOp { Eksi, Degil }

#[derive(Debug, Clone, PartialEq)]
pub enum Ifade {
    Sayi(i64),
    Mantiksal(bool),
    Degisken(String),
    Tekli(TekliOp, Box<Ifade>),
    Ikili(Box<Ifade>, IkiliOp, Box<Ifade>),
    Cagri(String, Vec<Ifade>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Deyim {
    Fonksiyon { ad: String, parametreler: Vec<String>, govde: Vec<Deyim> },
    Let(String, Ifade),
    Atama(String, Ifade),
    If(Ifade, Vec<Deyim>, Vec<Deyim>),
    While(Ifade, Vec<Deyim>),
    Return(Option<Ifade>),
    Print(Ifade),
    Ifade(Ifade),
}

impl IkiliOp {
    pub fn simge(self) -> &'static str {
        use IkiliOp::*;
        match self {
            Topla => "+", Cikar => "-", Carp => "*", Bol => "/", Mod => "%", Esit => "==", EsitDegil => "!=",
            Kucuk => "<", KucukEsit => "<=", Buyuk => ">", BuyukEsit => ">=", Ve => "&&", Veya => "||",
        }
    }
}

/// AST'yi S-ifadesi olarak yazar: öncelik parantezlerden okunur
pub fn s_ifade(i: &Ifade) -> String {
    match i {
        Ifade::Sayi(n) => n.to_string(),
        Ifade::Mantiksal(b) => b.to_string(),
        Ifade::Degisken(a) => a.clone(),
        Ifade::Tekli(TekliOp::Eksi, x) => format!("(- {})", s_ifade(x)),
        Ifade::Tekli(TekliOp::Degil, x) => format!("(! {})", s_ifade(x)),
        Ifade::Ikili(a, op, b) => format!("({} {} {})", op.simge(), s_ifade(a), s_ifade(b)),
        Ifade::Cagri(ad, arg) => format!("({ad}{})", arg.iter().map(|a| format!(" {}", s_ifade(a))).collect::<String>()),
    }
}

pub fn deyim_yaz(d: &Deyim, girinti: usize, cikti: &mut String) {
    let b = "  ".repeat(girinti);
    match d {
        Deyim::Fonksiyon { ad, parametreler, govde } => {
            cikti.push_str(&format!("{b}(fn {ad} ({})\n", parametreler.join(" ")));
            for g in govde { deyim_yaz(g, girinti + 1, cikti); }
            cikti.push_str(&format!("{b})\n"));
        }
        Deyim::Let(a, i) => cikti.push_str(&format!("{b}(let {a} {})\n", s_ifade(i))),
        Deyim::Atama(a, i) => cikti.push_str(&format!("{b}(= {a} {})\n", s_ifade(i))),
        Deyim::If(k, e, h) => {
            cikti.push_str(&format!("{b}(if {}\n", s_ifade(k)));
            for g in e { deyim_yaz(g, girinti + 1, cikti); }
            if !h.is_empty() {
                cikti.push_str(&format!("{b} else\n"));
                for g in h { deyim_yaz(g, girinti + 1, cikti); }
            }
            cikti.push_str(&format!("{b})\n"));
        }
        Deyim::While(k, g) => {
            cikti.push_str(&format!("{b}(while {}\n", s_ifade(k)));
            for x in g { deyim_yaz(x, girinti + 1, cikti); }
            cikti.push_str(&format!("{b})\n"));
        }
        Deyim::Return(Some(i)) => cikti.push_str(&format!("{b}(return {})\n", s_ifade(i))),
        Deyim::Return(None) => cikti.push_str(&format!("{b}(return)\n")),
        Deyim::Print(i) => cikti.push_str(&format!("{b}(print {})\n", s_ifade(i))),
        Deyim::Ifade(i) => cikti.push_str(&format!("{b}{}\n", s_ifade(i))),
    }
}
