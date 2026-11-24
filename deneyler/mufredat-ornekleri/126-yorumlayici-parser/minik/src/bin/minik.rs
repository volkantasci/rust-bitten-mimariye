//! Komut satırı (126. adım): `token` belirteçleri, `ast` söz dizimi ağacını S-ifadesi olarak,
//! `iz` Pratt ayrıştırmanın adımlarını ve ağacı yazar.
use minik::{ast, lexer, parser};

fn main() {
    let argumanlar: Vec<String> = std::env::args().collect();
    if argumanlar.len() != 3 {
        eprintln!("kullanım: minik <token|ast|iz> <dosya.mk>");
        std::process::exit(2);
    }
    let kaynak = std::fs::read_to_string(&argumanlar[2]).unwrap_or_else(|h| {
        eprintln!("hata: {}: {h}", argumanlar[2]);
        std::process::exit(2)
    });
    if let Err(h) = komut(&argumanlar[1], &kaynak) {
        eprintln!("hata: {h}");
        std::process::exit(1);
    }
}

fn agaci_yaz(program: &[ast::Deyim]) {
    let mut s = String::new();
    for d in program {
        ast::deyim_yaz(d, 0, &mut s);
    }
    print!("{s}");
}

fn komut(ne: &str, kaynak: &str) -> Result<(), String> {
    match ne {
        "token" => {
            for t in lexer::tokenlere_ayir(kaynak)? {
                println!("{:>2}:{:<2} {:?}", t.satir, t.sutun, t.tur);
            }
        }
        "ast" => agaci_yaz(&parser::ayristir(kaynak)?),
        "iz" => {
            let (program, iz) = parser::izleyerek_ayristir(kaynak)?;
            for satir in iz {
                println!("{satir}");
            }
            agaci_yaz(&program);
        }
        _ => return Err(format!("bilinmeyen komut: {ne}")),
    }
    Ok(())
}
