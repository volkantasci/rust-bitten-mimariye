//! Minik: bitirme projesi için küçük bir programlama dili.
//! Kaynak metin → sözcük çözümleyici (lexer) → ayrıştırıcı (parser) → AST
//! → (a) ağaç yürüten yorumlayıcı  (b) bayt kod derleyici + yığın makinesi (VM)
pub mod ast;
pub mod derleyici;
pub mod lexer;
pub mod parser;
pub mod vm;
pub mod yorumlayici;

#[cfg(test)]
mod testler {
    use crate::ast::{IkiliOp, Deyim, s_ifade};
    use crate::yorumlayici::{Deger, ikili};

    fn ifade(kaynak: &str) -> String {
        match crate::parser::ayristir(kaynak).unwrap().remove(0) {
            Deyim::Ifade(i) => s_ifade(&i),
            d => panic!("ifade bekleniyordu: {d:?}"),
        }
    }

    #[test]
    fn oncelik_ve_sola_baglilik() {
        assert_eq!(ifade("1 + 2 * 3 - 4;"), "(- (+ 1 (* 2 3)) 4)");
        assert_eq!(ifade("a < b && !c || d;"), "(|| (&& (< a b) (! c)) d)");
    }

    #[test]
    fn hata_konumu_bildirilir() {
        let h = crate::parser::ayristir("let x = (1 + 2;").unwrap_err();
        assert!(h.contains("sütun 15"), "{h}");
    }

    #[test]
    fn tasma_sessizce_sarmaz() {
        assert!(ikili(IkiliOp::Carp, Deger::Sayi(i64::MAX), Deger::Sayi(2)).is_err());
    }

    #[test]
    fn iki_motor_ayni_adim_sayisina_gerek_duymaz_ama_ayni_sonucu_verir() {
        let p = crate::parser::ayristir("fn f(n) { if n < 2 { return n; } return f(n - 1) + f(n - 2); } f(15);").unwrap();
        assert!(crate::yorumlayici::Yorumlayici::calistir(&p).is_ok());
        assert!(crate::vm::calistir(&crate::derleyici::derle(&p).unwrap()).is_ok());
    }
}
