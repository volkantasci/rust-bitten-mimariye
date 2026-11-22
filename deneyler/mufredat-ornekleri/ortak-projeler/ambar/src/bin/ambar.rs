use ambar::motor::{FsyncKipi, Motor};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::Path;
use std::time::Instant;

fn kip_oku(s: &str) -> FsyncKipi {
    match s {
        "her" => FsyncKipi::Her,
        "toplu" => FsyncKipi::Toplu,
        "yok" => FsyncKipi::Yok,
        _ => panic!("fsync kipi: her | toplu | yok"),
    }
}

fn istek_gonder(akis: &mut TcpStream, okuyucu: &mut BufReader<TcpStream>, istek: &str) -> std::io::Result<String> {
    akis.write_all(format!("{istek}\n").as_bytes())?;
    let mut yanit = String::new();
    if okuyucu.read_line(&mut yanit)? == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "bağlantı kapandı"));
    }
    Ok(yanit.trim_end().to_string())
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let arg = |i: usize| a.get(i).map(String::as_str).unwrap_or("");
    match arg(1) {
        "demo" => {
            let (m, _) = Motor::ac(Path::new(arg(2)), FsyncKipi::Her).unwrap();
            m.yaz("dil", Some("rust")).unwrap();
            m.yaz("yil", Some("2015")).unwrap();
            m.yaz("dil", Some("Rust")).unwrap(); // üzerine yazma: günlüğe yeni kayıt eklenir
            m.yaz("yil", None).unwrap(); // silme: mezar taşı kaydı
            m.yaz("maskot", Some("Ferris")).unwrap();
            println!("bellekteki durum: {:?}", m.anahtarlar());
        }
        "kurtar" => {
            let (m, r) = Motor::ac(Path::new(arg(2)), FsyncKipi::Yok).unwrap();
            println!("geçerli kayıt: {}, geçerli bayt: {}, atılan bayt: {}", r.gecerli_kayit, r.gecerli_bayt, r.atilan_bayt);
            if let Some(n) = r.neden {
                println!("kurtarma notu: {n}");
            }
            let v = m.anahtarlar();
            println!("anahtar sayısı: {}", v.len());
            for (k, d) in v.iter().take(3) {
                println!("  {k} = {d}");
            }
        }
        "doldur" => {
            let (m, _) = Motor::ac(Path::new(arg(2)), FsyncKipi::Yok).unwrap();
            let n: usize = arg(3).parse().unwrap();
            for i in 0..n {
                m.yaz(&format!("anahtar{i:05}"), Some(&format!("değer-{i}"))).unwrap();
            }
            println!("{n} kayıt yazıldı");
        }
        "sunucu" => {
            let (m, r) = Motor::ac(Path::new(arg(3)), kip_oku(arg(4))).unwrap();
            eprintln!("kurtarıldı: {} kayıt", r.gecerli_kayit);
            ambar::sunucu::calistir(arg(2), m).unwrap();
        }
        "istemci" => {
            let mut akis = TcpStream::connect(arg(2)).unwrap();
            let mut okuyucu = BufReader::new(akis.try_clone().unwrap());
            for istek in &a[3..] {
                println!("> {istek}");
                println!("< {}", istek_gonder(&mut akis, &mut okuyucu, istek).unwrap());
            }
        }
        "yuk" => {
            let (istemci, adet): (usize, usize) = (arg(3).parse().unwrap(), arg(4).parse().unwrap());
            let adres = arg(2).to_string();
            let t = Instant::now();
            std::thread::scope(|s| {
                for c in 0..istemci {
                    let adres = adres.clone();
                    s.spawn(move || {
                        let mut akis = TcpStream::connect(&adres).unwrap();
                        akis.set_nodelay(true).unwrap();
                        let mut okuyucu = BufReader::new(akis.try_clone().unwrap());
                        for i in 0..adet {
                            let y = istek_gonder(&mut akis, &mut okuyucu, &format!("SET i{c}-{i} değer")).unwrap();
                            assert_eq!(y, "OK");
                        }
                    });
                }
            });
            let sn = t.elapsed().as_secs_f64();
            let toplam = istemci * adet;
            let mut akis = TcpStream::connect(&adres).unwrap();
            let mut okuyucu = BufReader::new(akis.try_clone().unwrap());
            let say = istek_gonder(&mut akis, &mut okuyucu, "SAY").unwrap();
            println!("{istemci} istemci × {adet} SET = {toplam} yazma, {sn:.2} s, {:.0} yazma/s, sunucu: {say}", toplam as f64 / sn);
        }
        "yazici" => {
            // Çökme deneyi: onaylanan (OK alınan) son yazmayı say
            let mut akis = TcpStream::connect(arg(2)).unwrap();
            let mut okuyucu = BufReader::new(akis.try_clone().unwrap());
            let n: usize = arg(3).parse().unwrap();
            let mut onaylanan = 0;
            for i in 0..n {
                match istek_gonder(&mut akis, &mut okuyucu, &format!("SET k{i:07} v{i}")) {
                    Ok(y) if y == "OK" => onaylanan += 1,
                    _ => break,
                }
            }
            println!("onaylanan yazma: {onaylanan}");
        }
        _ => eprintln!("kullanım: ambar <demo|kurtar|doldur|sunucu|istemci|yuk|yazici> ..."),
    }
}
