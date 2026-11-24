//! Komut satırı (127. adım): `token`, `ast`, `iz` ve `calistir` (ağaç dolaşan yorumlayıcı).
//! `calistir --derinlik N` çağrı derinliği sınırını değiştirir (varsayılan 2000).
use minik::{ast, lexer, parser, yorumlayici};

fn main() {
    let mut argumanlar: Vec<String> = std::env::args().skip(1).collect();
    let mut derinlik = yorumlayici::AZAMI_DERINLIK;
    if let Some(i) = argumanlar.iter().position(|a| a == "--derinlik") {
        derinlik = argumanlar.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or_else(|| {
            eprintln!("hata: --derinlik bir sayı ister");
            std::process::exit(2)
        });
        argumanlar.drain(i..i + 2);
    }
    if argumanlar.len() != 2 {
        eprintln!("kullanım: minik <token|ast|iz|calistir [--derinlik N]> <dosya.mk>");
        std::process::exit(2);
    }
    let kaynak = std::fs::read_to_string(&argumanlar[1]).unwrap_or_else(|h| {
        eprintln!("hata: {}: {h}", argumanlar[1]);
        std::process::exit(2)
    });
    if let Err(h) = komut(&argumanlar[0], &kaynak, derinlik) {
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

fn komut(ne: &str, kaynak: &str, derinlik: usize) -> Result<(), String> {
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
        "calistir" => {
            let program = parser::ayristir(kaynak)?;
            yorumlayici::calistir(&program, derinlik, &mut std::io::stdout())?;
        }
        _ => return Err(format!("bilinmeyen komut: {ne}")),
    }
    Ok(())
}
