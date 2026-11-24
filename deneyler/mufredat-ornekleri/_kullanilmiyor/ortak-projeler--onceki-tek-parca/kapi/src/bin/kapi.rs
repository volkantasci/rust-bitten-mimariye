use kapi::ayristirici::Ayristirici;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let arg = |i: usize| a.get(i).cloned().unwrap_or_default();
    match arg(1).as_str() {
        "havuz" => kapi::havuz::calistir(&arg(2), PathBuf::from(arg(3)), arg(4).parse().unwrap()).unwrap(),
        "async" => tokio::runtime::Runtime::new().unwrap().block_on(kapi::zaman_uyumsuz::calistir(&arg(2), PathBuf::from(arg(3)))).unwrap(),
        "parca-parca" => {
            // Ayrıştırıcı durum makinesini adım adım göster
            let mut ay = Ayristirici::default();
            for parca in ["GET /belgeler/not", ".txt HTTP/1.1\r\nHo", "st: kapi\r\n", "\r\n"] {
                println!("besle({parca:?}) → {:?}", ay.besle(parca.as_bytes()).map(|s| s.map(|i| (i.yontem, i.hedef, i.basliklar))));
            }
        }
        "yuk" => {
            let (baglanti, istek): (usize, usize) = (arg(3).parse().unwrap(), arg(4).parse().unwrap());
            let adres = arg(2);
            let t = Instant::now();
            let mut sureler: Vec<f64> = std::thread::scope(|s| {
                let h: Vec<_> = (0..baglanti)
                    .map(|_| {
                        let adres = adres.clone();
                        s.spawn(move || {
                            let t0 = Instant::now();
                            let mut akis = TcpStream::connect(&adres).unwrap();
                            akis.set_nodelay(true).unwrap();
                            let mut okuyucu = BufReader::new(akis.try_clone().unwrap());
                            for _ in 0..istek {
                                akis.write_all(b"GET /index.html HTTP/1.1\r\nHost: kapi\r\n\r\n").unwrap();
                                let mut uzunluk = 0;
                                loop {
                                    let mut satir = String::new();
                                    okuyucu.read_line(&mut satir).unwrap();
                                    if satir == "\r\n" { break; }
                                    if let Some(u) = satir.strip_prefix("Content-Length: ") { uzunluk = u.trim().parse().unwrap(); }
                                }
                                let mut govde = vec![0u8; uzunluk];
                                okuyucu.read_exact(&mut govde).unwrap();
                            }
                            t0.elapsed().as_secs_f64() * 1e3
                        })
                    })
                    .collect();
                h.into_iter().map(|x| x.join().unwrap()).collect()
            });
            let toplam = t.elapsed().as_secs_f64();
            sureler.sort_by(f64::total_cmp);
            println!("{baglanti} bağlantı × {istek} istek: {:.0} istek/s; bağlantı süresi en kısa {:.0} ms, ortanca {:.0} ms, en uzun {:.0} ms",
                     (baglanti * istek) as f64 / toplam, sureler[0], sureler[sureler.len() / 2], sureler[sureler.len() - 1]);
        }
        "yavas" => {
            // Yavaş istemciler (slowloris benzeri): başlığı hiç bitirmeyen n bağlantı
            let (adres, n): (String, usize) = (arg(2), arg(3).parse().unwrap());
            let _yavaslar: Vec<TcpStream> = (0..n)
                .map(|_| {
                    let mut s = TcpStream::connect(&adres).unwrap();
                    s.write_all(b"GET / HTTP/1.1\r\nHost: ").unwrap(); // ...ve sonsuza dek sus
                    s
                })
                .collect();
            std::thread::sleep(Duration::from_millis(300));
            let t = Instant::now();
            let mut s = TcpStream::connect(&adres).unwrap();
            s.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
            s.write_all(b"GET /index.html HTTP/1.1\r\nHost: kapi\r\nConnection: close\r\n\r\n").unwrap();
            let mut yanit = String::new();
            let sonuc = match s.read_to_string(&mut yanit) {
                Ok(_) => yanit.lines().next().unwrap_or("").to_string(),
                Err(e) => format!("yanıt yok ({e})"),
            };
            println!("{n} yavaş bağlantı açıkken normal bir istek: {} ms → {sonuc}", t.elapsed().as_millis());
        }
        _ => eprintln!("kullanım: kapi <havuz ADRES KÖK İŞÇİ | async ADRES KÖK | parca-parca | yuk ADRES BAĞLANTI İSTEK>"),
    }
}
