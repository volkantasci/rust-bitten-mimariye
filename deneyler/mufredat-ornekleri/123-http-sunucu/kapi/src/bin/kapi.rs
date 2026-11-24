//! Komut satırı (123. adım): `havuz` sunucuyu başlatır; `parca-parca` ayrıştırıcıyı
//! parça parça besleyip her adımı gösterir; `tarama-olc` boş satırı aramanın maliyetini ölçer.
use kapi::ayristirici::Ayristirici;
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;

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

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("havuz") if a.len() == 4 => {
            let isci: usize = a[3].parse().unwrap_or_else(|h| {
                eprintln!("hata: işçi sayısı: {h}");
                exit(2)
            });
            if let Err(h) = kapi::havuz::calistir(&a[1], PathBuf::from(&a[2]), isci) {
                eprintln!("hata: {h}");
                exit(1)
            }
        }
        Some("tarama-olc") => tarama_olc(),
        Some("parca-parca") if a.len() > 1 => {
            let mut ay = Ayristirici::default();
            for parca in &a[1..] {
                let baytlar = kacislari_coz(parca);
                let gosterim = String::from_utf8_lossy(&baytlar);
                match ay.besle(&baytlar) {
                    Ok(None) => println!(
                        "besle({gosterim:?}) → eksik; tamponda {} bayt, durum: {}",
                        ay.tampondaki_bayt(),
                        ay.durum()
                    ),
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
        _ => {
            eprintln!("kullanım: kapi havuz ADRES KÖK İŞÇİ | tarama-olc | parca-parca PARÇA... (PARÇA içinde \\r ve \\n kullanılabilir)");
            exit(2)
        }
    }
}
