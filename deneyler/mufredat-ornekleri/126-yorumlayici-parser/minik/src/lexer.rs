//! Sözcüksel çözümleyici (lexer): karakter akışını belirteçlere (token) böler. Belirteç türlerinin
//! her biri bir düzenli dildir; tarama, bir sonlu otomat gibi karakter karakter ilerler.
//! İki kural: en uzun eşleşme ("==" iki "=" değil, tek belirteç) ve anahtar sözcük tablosu
//! (önce ad gibi okunur, sonra tabloya bakılır). Konum satır:sütun; sütun karakter sayar.

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
    let karakterler: Vec<char> = kaynak.chars().collect(); // sütunlar karakter (Unicode skaler değer) sayar
    let (mut i, mut satir, mut sutun) = (0, 1, 1);
    let mut tokenler = Vec::new();
    while i < karakterler.len() {
        let c = karakterler[i];
        let (bas_satir, bas_sutun) = (satir, sutun);
        if c == '\n' {
            i += 1;
            satir += 1;
            sutun = 1;
            continue;
        }
        if c.is_whitespace() {
            i += 1;
            sutun += 1;
            continue;
        }
        if c == '/' && karakterler.get(i + 1) == Some(&'/') {
            while i < karakterler.len() && karakterler[i] != '\n' {
                i += 1; // yorum satırın sonuna kadar
            }
            continue;
        }
        let tur = if c.is_ascii_digit() {
            let bas = i;
            while i < karakterler.len() && karakterler[i].is_ascii_digit() {
                i += 1;
            }
            sutun += i - bas;
            let metin: String = karakterler[bas..i].iter().collect();
            let deger = metin
                .parse::<i64>()
                .map_err(|_| format!("satır {bas_satir}, sütun {bas_sutun}: sayı çok büyük: {metin}"))?;
            Tur::Sayi(deger)
        } else if c.is_alphabetic() || c == '_' {
            let bas = i;
            while i < karakterler.len() && (karakterler[i].is_alphanumeric() || karakterler[i] == '_') {
                i += 1;
            }
            sutun += i - bas;
            let kelime: String = karakterler[bas..i].iter().collect();
            match kelime.as_str() {
                "fn" => Tur::Fn,
                "let" => Tur::Let,
                "if" => Tur::If,
                "else" => Tur::Else,
                "while" => Tur::While,
                "return" => Tur::Return,
                "print" => Tur::Print,
                "true" => Tur::True,
                "false" => Tur::False,
                _ => Tur::Ad(kelime),
            }
        } else {
            let sonraki = karakterler.get(i + 1).copied();
            // En uzun eşleşme: iki karakterli operatörler tek karakterlilerden önce denenir
            let (tur, uzunluk) = match (c, sonraki) {
                ('=', Some('=')) => (Tur::EsitEsit, 2),
                ('!', Some('=')) => (Tur::UnlemEsit, 2),
                ('<', Some('=')) => (Tur::KucukEsit, 2),
                ('>', Some('=')) => (Tur::BuyukEsit, 2),
                ('&', Some('&')) => (Tur::VeVe, 2),
                ('|', Some('|')) => (Tur::VeyaVeya, 2),
                ('=', _) => (Tur::Esit, 1),
                ('!', _) => (Tur::Unlem, 1),
                ('<', _) => (Tur::Kucuk, 1),
                ('>', _) => (Tur::Buyuk, 1),
                ('+', _) => (Tur::Arti, 1),
                ('-', _) => (Tur::Eksi, 1),
                ('*', _) => (Tur::Yildiz, 1),
                ('/', _) => (Tur::Bolu, 1),
                ('%', _) => (Tur::Yuzde, 1),
                ('(', _) => (Tur::SolParantez, 1),
                (')', _) => (Tur::SagParantez, 1),
                ('{', _) => (Tur::SolSuslu, 1),
                ('}', _) => (Tur::SagSuslu, 1),
                (',', _) => (Tur::Virgul, 1),
                (';', _) => (Tur::NoktaliVirgul, 1),
                _ => return Err(format!("satır {bas_satir}, sütun {bas_sutun}: beklenmeyen karakter {c:?}")),
            };
            i += uzunluk;
            sutun += uzunluk;
            tur
        };
        tokenler.push(Token { tur, satir: bas_satir, sutun: bas_sutun });
    }
    tokenler.push(Token { tur: Tur::Son, satir, sutun });
    Ok(tokenler)
}

#[cfg(test)]
mod testler {
    use super::*;

    fn turler(kaynak: &str) -> Vec<Tur> {
        tokenlere_ayir(kaynak).unwrap().into_iter().map(|t| t.tur).collect()
    }

    #[test]
    fn en_uzun_eslesme() {
        use Tur::*;
        assert_eq!(turler("a == b = c"), [Ad("a".into()), EsitEsit, Ad("b".into()), Esit, Ad("c".into()), Son]);
    }

    #[test]
    fn anahtar_sozcuk_once_ad_gibi_okunur() {
        use Tur::*;
        assert_eq!(turler("let letter"), [Let, Ad("letter".into()), Son]); // "letter" let ile başlasa da ad
    }

    #[test]
    fn sutun_karakter_sayar_bayt_degil() {
        let t = tokenlere_ayir("let ı = 1;").unwrap();
        assert_eq!((t[2].tur.clone(), t[2].sutun), (Tur::Esit, 7)); // "ı" UTF-8'de iki bayt, bir karakter
    }

    #[test]
    fn yorum_atlanir_ve_hata_konumu_bildirilir() {
        assert_eq!(turler("1 // 2\n3"), [Tur::Sayi(1), Tur::Sayi(3), Tur::Son]);
        assert_eq!(tokenlere_ayir("x\n  @").unwrap_err(), "satır 2, sütun 3: beklenmeyen karakter '@'");
    }
}
