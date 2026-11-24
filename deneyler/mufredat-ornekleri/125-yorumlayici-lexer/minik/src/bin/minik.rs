//! Komut satırı (125. adım): `minik token DOSYA` kaynağın belirteçlerini konumlarıyla yazar.
use minik::lexer;

fn main() {
    let argumanlar: Vec<String> = std::env::args().collect();
    if argumanlar.len() != 3 || argumanlar[1] != "token" {
        eprintln!("kullanım: minik token <dosya.mk>");
        std::process::exit(2);
    }
    let kaynak = std::fs::read_to_string(&argumanlar[2]).unwrap_or_else(|h| {
        eprintln!("hata: {}: {h}", argumanlar[2]);
        std::process::exit(2)
    });
    match lexer::tokenlere_ayir(&kaynak) {
        Ok(tokenler) => {
            for t in tokenler {
                println!("{:>2}:{:<2} {:?}", t.satir, t.sutun, t.tur);
            }
        }
        Err(h) => {
            eprintln!("hata: {h}");
            std::process::exit(1);
        }
    }
}
