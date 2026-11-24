//! Komut satırı (128. adım, bitmiş hâl): `token`, `ast`, `iz`, `calistir` (ağaç dolaşan
//! yorumlayıcı), `listele` (bayt kod), `vm` (yığın makinesi) ve `kiyasla` (iki motor).
//! `--derinlik N` çağrı derinliği sınırını değiştirir (varsayılan 2000).
use minik::{ast, derleyici, lexer, parser, vm, yorumlayici};
use std::time::Instant;

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
        eprintln!("kullanım: minik <token|ast|iz|calistir|listele|vm|kiyasla> [--derinlik N] <dosya.mk>");
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
        "listele" => print!("{}", derleyici::listele(&derleyici::derle(&parser::ayristir(kaynak)?)?)),
        "vm" => {
            let program = derleyici::derle(&parser::ayristir(kaynak)?)?;
            vm::calistir(&program, derinlik, &mut std::io::stdout().lock())?;
        }
        "kiyasla" => {
            let program = parser::ayristir(kaynak)?;
            let t = Instant::now();
            let s = yorumlayici::calistir(&program, derinlik, &mut std::io::stdout())?;
            let t_agac = t.elapsed().as_secs_f64() * 1e3;
            let bayt_kod = derleyici::derle(&program)?;
            let t = Instant::now();
            let adim = vm::calistir(&bayt_kod, derinlik, &mut std::io::stdout().lock())?;
            let t_vm = t.elapsed().as_secs_f64() * 1e3;
            println!(
                "ağaç dolaşan yorumlayıcı:  {t_agac:>7.1} ms; {} ifade düğümü, {} HashMap ad araması, {} yeni kapsam",
                s.ifade_dugumu, s.ad_aramasi, s.yeni_kapsam
            );
            println!("bayt kod + yığın makinesi: {t_vm:>7.1} ms; {adim} komut; ad araması yok (yuva numaraları derleme anında)");
            println!("hızlanma: {:.2}×", t_agac / t_vm);
        }
        _ => return Err(format!("bilinmeyen komut: {ne}")),
    }
    Ok(())
}
