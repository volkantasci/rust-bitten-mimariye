//! Sözcük çözümleyici: karakter akışını token'lara böler (düzenli dil / sonlu otomat).

#[derive(Debug, Clone, PartialEq)]
pub enum Tur {
    Sayi(i64),
    Ad(String),
    Fn, Let, If, Else, While, Return, Print, True, False,
    Arti, Eksi, Yildiz, Bolu, Yuzde,
    Esit, EsitEsit, Unlem, UnlemEsit, Kucuk, KucukEsit, Buyuk, BuyukEsit, VeVe, VeyaVeya,
    SolParantez, SagParantez, SolSuslu, SagSuslu, Virgul, NoktaliVirgul,
    Son,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tur: Tur,
    pub satir: usize,
    pub sutun: usize,
}

pub fn tokenlere_ayir(kaynak: &str) -> Result<Vec<Token>, String> {
    let karakterler: Vec<char> = kaynak.chars().collect();
    let (mut i, mut satir, mut sutun) = (0, 1, 1);
    let mut tokenler = Vec::new();
    while i < karakterler.len() {
        let c = karakterler[i];
        let (bas_satir, bas_sutun) = (satir, sutun);
        let ilerle = |n: usize, i: &mut usize, sutun: &mut usize| { *i += n; *sutun += n; };
        if c == '\n' {
            i += 1; satir += 1; sutun = 1;
            continue;
        }
        if c.is_whitespace() { ilerle(1, &mut i, &mut sutun); continue; }
        if c == '/' && karakterler.get(i + 1) == Some(&'/') {
            while i < karakterler.len() && karakterler[i] != '\n' { i += 1; } // yorum satırı
            continue;
        }
        let tur = if c.is_ascii_digit() {
            let bas = i;
            while i < karakterler.len() && karakterler[i].is_ascii_digit() { i += 1; }
            sutun += i - bas;
            let metin: String = karakterler[bas..i].iter().collect();
            let deger = metin.parse::<i64>().map_err(|_| format!("satır {bas_satir}, sütun {bas_sutun}: sayı çok büyük: {metin}"))?;
            tokenler.push(Token { tur: Tur::Sayi(deger), satir: bas_satir, sutun: bas_sutun });
            continue;
        } else if c.is_alphabetic() || c == '_' {
            let bas = i;
            while i < karakterler.len() && (karakterler[i].is_alphanumeric() || karakterler[i] == '_') { i += 1; }
            sutun += i - bas;
            let kelime: String = karakterler[bas..i].iter().collect();
            let tur = match kelime.as_str() {
                "fn" => Tur::Fn, "let" => Tur::Let, "if" => Tur::If, "else" => Tur::Else,
                "while" => Tur::While, "return" => Tur::Return, "print" => Tur::Print,
                "true" => Tur::True, "false" => Tur::False,
                _ => Tur::Ad(kelime),
            };
            tokenler.push(Token { tur, satir: bas_satir, sutun: bas_sutun });
            continue;
        } else {
            let sonraki = karakterler.get(i + 1).copied();
            let (tur, uzunluk) = match (c, sonraki) {
                ('=', Some('=')) => (Tur::EsitEsit, 2),
                ('!', Some('=')) => (Tur::UnlemEsit, 2),
                ('<', Some('=')) => (Tur::KucukEsit, 2),
                ('>', Some('=')) => (Tur::BuyukEsit, 2),
                ('&', Some('&')) => (Tur::VeVe, 2),
                ('|', Some('|')) => (Tur::VeyaVeya, 2),
                ('=', _) => (Tur::Esit, 1), ('!', _) => (Tur::Unlem, 1),
                ('<', _) => (Tur::Kucuk, 1), ('>', _) => (Tur::Buyuk, 1),
                ('+', _) => (Tur::Arti, 1), ('-', _) => (Tur::Eksi, 1),
                ('*', _) => (Tur::Yildiz, 1), ('/', _) => (Tur::Bolu, 1), ('%', _) => (Tur::Yuzde, 1),
                ('(', _) => (Tur::SolParantez, 1), (')', _) => (Tur::SagParantez, 1),
                ('{', _) => (Tur::SolSuslu, 1), ('}', _) => (Tur::SagSuslu, 1),
                (',', _) => (Tur::Virgul, 1), (';', _) => (Tur::NoktaliVirgul, 1),
                _ => return Err(format!("satır {bas_satir}, sütun {bas_sutun}: beklenmeyen karakter {c:?}")),
            };
            ilerle(uzunluk, &mut i, &mut sutun);
            tur
        };
        tokenler.push(Token { tur, satir: bas_satir, sutun: bas_sutun });
    }
    tokenler.push(Token { tur: Tur::Son, satir, sutun });
    Ok(tokenler)
}
