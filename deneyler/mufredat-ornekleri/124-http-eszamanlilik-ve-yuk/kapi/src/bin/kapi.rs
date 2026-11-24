//! Komut satırı (124. adım, bitmiş hâl): iki sunucu (`havuz`, `async`), ayrıştırıcı gösterimi
//! (`parca-parca`), tarama ölçümü (`tarama-olc`), yük üreteci (`yuk`) ve yavaş istemci denemesi (`yavas`).
use kapi::ayristirici::Ayristirici;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::exit;
use std::time::{Duration, Instant};

const KULLANIM: &str = "kullanım: kapi havuz ADRES KÖK İŞÇİ [SON_SÜRE_MS] | async ADRES KÖK [SON_SÜRE_MS] \
| parca-parca PARÇA... | tarama-olc | yuk ADRES BAĞLANTI İSTEK | yavas ADRES N";

fn hata(h: impl std::fmt::Display) -> ! {
    eprintln!("hata: {h}");
    exit(1)
}

fn sayi(s: &str) -> u64 {
    s.parse().unwrap_or_else(|h| hata(format!("{s}: {h}")))
}

/// "\r", "\n" ve "\\" kaçışlarını baytlara çevirir (komut satırından CR LF verebilmek için).
fn kacislari_coz(s: &str) -> Vec<u8> {
    let mut v = Vec::new();
    let mut k = s.bytes();
    while let Some(b) = k.next() {
        if b != b'\\' {
            v.push(b);
            continue;
        }
        match k.next() {
            Some(b'r') => v.push(b'\r'),
            Some(b'n') => v.push(b'\n'),
            Some(diger) => v.push(diger),
            None => v.push(b'\\'),
        }
    }
    v
}

fn parca_parca(parcalar: &[String]) {
    let mut ay = Ayristirici::default();
    for parca in parcalar {
        let baytlar = kacislari_coz(parca);
        let gosterim = String::from_utf8_lossy(&baytlar);
        match ay.besle(&baytlar) {
            Ok(None) => println!("besle({gosterim:?}) → eksik; tamponda {} bayt, durum: {}", ay.tampondaki_bayt(), ay.durum()),
            Ok(Some(i)) => println!(
                "besle({gosterim:?}) → istek: {} {} {}, başlıklar: {:?}, gövde: {} bayt",
                i.yontem,
                i.hedef,
                i.surum,
                i.basliklar,
                i.govde.len()
            ),
            Err(h) => {
                println!("besle({gosterim:?}) → hata: {h:?}");
                exit(1)
            }
        }
    }
}

/// Tarama maliyeti: 8 KiB'a yakın bir başlığı bayt bayt besle; (a) ayrıştırıcı boş satırı
/// kaldığı yerden arar, (b) saf döngü her baytta tamponun tamamını baştan arar.
fn tarama_olc() {
    let mut istek = b"GET / HTTP/1.1\r\n".to_vec();
    while istek.len() < 8000 {
        istek.extend_from_slice(b"X-Dolgu: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\r\n");
    }
    istek.extend_from_slice(b"\r\n");
    let t = Instant::now();
    let mut ay = Ayristirici::default();
    let mut artimli = false;
    for b in &istek {
        artimli |= ay.besle(std::slice::from_ref(b)).unwrap().is_some();
    }
    let sure_artimli = t.elapsed().as_secs_f64() * 1e3;
    let t = Instant::now();
    let mut tampon = Vec::new();
    let mut bastan = false;
    for b in &istek {
        tampon.push(*b);
        if tampon.windows(4).any(|w| w == b"\r\n\r\n") {
            bastan = true;
            break;
        }
    }
    let sure_bastan = t.elapsed().as_secs_f64() * 1e3;
    assert!(artimli && bastan);
    println!(
        "{} baytlık başlık bayt bayt: kaldığı yerden arama {sure_artimli:.3} ms, her baytta baştan arama {sure_bastan:.3} ms ({:.0} kat)",
        istek.len(),
        sure_bastan / sure_artimli
    );
}

/// BAĞLANTI ayrı bağlantı açılır; her biri aynı bağlantı üzerinden İSTEK kez GET gönderir.
fn yuk(adres: &str, baglanti: usize, istek: usize) {
    let t = Instant::now();
    let mut sureler: Vec<f64> = std::thread::scope(|s| {
        let isler: Vec<_> = (0..baglanti)
            .map(|_| {
                s.spawn(move || {
                    let t0 = Instant::now();
                    let mut akis = TcpStream::connect(adres).unwrap();
                    akis.set_nodelay(true).unwrap();
                    let mut okuyucu = BufReader::new(akis.try_clone().unwrap());
                    for _ in 0..istek {
                        akis.write_all(b"GET /index.html HTTP/1.1\r\nHost: kapi\r\n\r\n").unwrap();
                        let mut uzunluk = 0;
                        loop {
                            let mut satir = String::new();
                            okuyucu.read_line(&mut satir).unwrap();
                            if satir == "\r\n" {
                                break;
                            }
                            if let Some(u) = satir.strip_prefix("Content-Length: ") {
                                uzunluk = u.trim().parse().unwrap();
                            }
                        }
                        let mut govde = vec![0u8; uzunluk];
                        okuyucu.read_exact(&mut govde).unwrap();
                    }
                    t0.elapsed().as_secs_f64() * 1e3
                })
            })
            .collect();
        isler.into_iter().map(|i| i.join().unwrap()).collect()
    });
    let toplam = t.elapsed().as_secs_f64();
    sureler.sort_by(f64::total_cmp);
    println!(
        "{baglanti} bağlantı × {istek} istek: {:.0} istek/s; bağlantının bitme süresi en kısa {:.0} ms, ortanca {:.0} ms, en uzun {:.0} ms",
        (baglanti * istek) as f64 / toplam,
        sureler[0],
        sureler[sureler.len() / 2],
        sureler[sureler.len() - 1]
    );
}

/// N bağlantı başlığı yarım bırakıp susar (yavaş istemci); sonra normal bir istek denenir.
fn yavas(adres: &str, n: usize) {
    let bas = Instant::now();
    let mut yavaslar: Vec<TcpStream> = (0..n)
        .map(|_| {
            let mut s = TcpStream::connect(adres).unwrap();
            s.write_all(b"GET / HTTP/1.1\r\nHost: ").unwrap(); // ...ve susar
            s
        })
        .collect();
    std::thread::sleep(Duration::from_millis(300));
    let t = Instant::now();
    let mut s = TcpStream::connect(adres).unwrap();
    s.set_read_timeout(Some(Duration::from_secs(3))).unwrap(); // istemcinin sabrı: 3 s
    s.write_all(b"GET /index.html HTTP/1.1\r\nHost: kapi\r\nConnection: close\r\n\r\n").unwrap();
    let mut yanit = String::new();
    let sonuc = match s.read_to_string(&mut yanit) {
        Ok(_) => yanit.lines().next().unwrap_or("").to_string(),
        Err(e) => format!("yanıt yok ({e})"),
    };
    println!("{n} yavaş bağlantı açıkken normal bir istek: {} ms → {sonuc}", t.elapsed().as_millis());
    // Sunucu yavaş bağlantılara ne yaptı? İlk bağlantıdan en çok 1,5 s sonrasına kadar bak.
    let mut kapatilan = 0;
    for y in &mut yavaslar {
        let kalan = Duration::from_millis(1500).saturating_sub(bas.elapsed()).max(Duration::from_millis(10));
        y.set_read_timeout(Some(kalan)).unwrap();
        let mut tampon = [0u8; 64];
        if let Ok(k) = y.read(&mut tampon) {
            if tampon[..k].starts_with(b"HTTP/1.1 408") {
                kapatilan += 1;
            }
        }
    }
    println!("  yavaş bağlantılardan {kapatilan}/{n} tanesi sunucu tarafından 408 ile kapatıldı");
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let s: Vec<&str> = a.iter().map(String::as_str).collect();
    let son_sure = |m: Option<&&str>| m.map(|ms| Duration::from_millis(sayi(ms)));
    match s[..] {
        ["havuz", adres, kok, isci, ..] if s.len() <= 5 => {
            let r = kapi::havuz::calistir(adres, PathBuf::from(kok), sayi(isci) as usize, son_sure(s.get(4)));
            r.unwrap_or_else(|h| hata(h))
        }
        ["async", adres, kok, ..] if s.len() <= 4 => {
            let calisma = tokio::runtime::Runtime::new().unwrap_or_else(|h| hata(h));
            let r = calisma.block_on(kapi::zaman_uyumsuz::calistir(adres, PathBuf::from(kok), son_sure(s.get(3))));
            r.unwrap_or_else(|h| hata(h))
        }
        ["parca-parca", _, ..] => parca_parca(&a[1..]),
        ["tarama-olc"] => tarama_olc(),
        ["yuk", adres, baglanti, istek] => yuk(adres, sayi(baglanti) as usize, sayi(istek) as usize),
        ["yavas", adres, n] => yavas(adres, sayi(n) as usize),
        _ => {
            eprintln!("{KULLANIM}");
            exit(2)
        }
    }
}
