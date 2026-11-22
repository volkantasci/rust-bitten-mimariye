use minik::{ast, derleyici, lexer, parser, vm, yorumlayici};
use std::time::Instant;

fn main() {
    let argumanlar: Vec<String> = std::env::args().collect();
    if argumanlar.len() != 3 {
        eprintln!("kullanım: minik <token|ast|calistir|listele|vm|kiyasla> <dosya.mk>");
        std::process::exit(2);
    }
    let kaynak = std::fs::read_to_string(&argumanlar[2]).expect("dosya okunamadı");
    if let Err(h) = komut(&argumanlar[1], &kaynak) {
        eprintln!("hata: {h}");
        std::process::exit(1);
    }
}

fn komut(ne: &str, kaynak: &str) -> Result<(), String> {
    match ne {
        "token" => {
            for t in lexer::tokenlere_ayir(kaynak)? {
                println!("{:>2}:{:<2} {:?}", t.satir, t.sutun, t.tur);
            }
        }
        "ast" => {
            let mut s = String::new();
            for d in parser::ayristir(kaynak)? { ast::deyim_yaz(&d, 0, &mut s); }
            print!("{s}");
        }
        "calistir" => {
            let program = parser::ayristir(kaynak)?;
            yorumlayici::Yorumlayici::calistir(&program)?;
        }
        "listele" => print!("{}", derleyici::listele(&derleyici::derle(&parser::ayristir(kaynak)?)?)),
        "vm" => {
            vm::calistir(&derleyici::derle(&parser::ayristir(kaynak)?)?)?;
        }
        "kiyasla" => {
            let program = parser::ayristir(kaynak)?;
            let t = Instant::now();
            let dugum = yorumlayici::Yorumlayici::calistir(&program)?;
            let t_agac = t.elapsed().as_secs_f64() * 1e3;
            let bayt_kod = derleyici::derle(&program)?;
            let t = Instant::now();
            let adim = vm::calistir(&bayt_kod)?;
            let t_vm = t.elapsed().as_secs_f64() * 1e3;
            println!("ağaç yürüten yorumlayıcı: {t_agac:>8.1} ms ({dugum} ifade düğümü değerlendirildi)");
            println!("bayt kod + yığın makinesi: {t_vm:>8.1} ms ({adim} komut yürütüldü)");
            println!("hızlanma: {:.2}×", t_agac / t_vm);
        }
        _ => return Err(format!("bilinmeyen komut: {ne}")),
    }
    Ok(())
}
