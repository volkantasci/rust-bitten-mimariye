//! Minik — bitirme projesi, 128. adım (bitmiş hâl): kaynak metin → sözcüksel çözümleyici
//! (lexer) → ayrıştırıcı (parser) → AST → (a) ağaç dolaşan yorumlayıcı, (b) bayt kod
//! derleyicisi + yığın makinesi (VM). İki motor aynı programda aynı çıktıyı vermeli.
pub mod ast;
pub mod derleyici;
pub mod lexer;
pub mod parser;
pub mod vm;
pub mod yorumlayici;

#[cfg(test)]
mod fark_testi {
    //! Fark testi (differential testing): aynı program iki ayrı gerçekleştirime verilir;
    //! çıktılar ve hata mesajları aynı olmalı. Farkı, bir tarafta hata olduğunu gösterir.
    use crate::{derleyici, parser, vm, yorumlayici};

    type Sonuc = (String, Result<(), String>);

    fn iki_motor(kaynak: &str) -> (Sonuc, Sonuc) {
        let program = parser::ayristir(kaynak).unwrap();
        let mut a = Vec::new();
        let ra = yorumlayici::calistir(&program, yorumlayici::AZAMI_DERINLIK, &mut a).map(|_| ());
        let mut b = Vec::new();
        let rb = derleyici::derle(&program)
            .and_then(|p| vm::calistir(&p, yorumlayici::AZAMI_DERINLIK, &mut b))
            .map(|_| ());
        ((String::from_utf8(a).unwrap(), ra), (String::from_utf8(b).unwrap(), rb))
    }

    #[test]
    fn ornek_programlarda_iki_motor_ayni() {
        for ad in ["fib", "kare", "kisa_devre", "sifira_bolme", "sonsuz", "ifade", "esitlik", "oncelik"] {
            let yol = format!("{}/ornekler/{ad}.mk", env!("CARGO_MANIFEST_DIR"));
            let kaynak = std::fs::read_to_string(&yol).unwrap();
            let (agac, vm) = iki_motor(&kaynak);
            assert_eq!(agac, vm, "{ad}.mk");
        }
    }

    #[test]
    fn tanimsiz_ad_iki_motorda_farkli_anda_yakalanir() {
        let (agac, vm) = iki_motor("print(1); print(y);");
        assert_eq!(agac.0, "1\n"); // ağaç dolaşan: önce 1'i yazar, adı çalışırken arar
        assert_eq!(vm.0, ""); // derleyici: adı derlerken çözer, hiçbir şey çalışmadan hata
        assert_ne!(agac.1, vm.1);
    }
}
