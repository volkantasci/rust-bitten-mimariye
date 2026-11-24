use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

// İstemci 20 istek gönderir; her istek 4 bayt başlık + 4 bayt gövdedir, her istekten sonra 1 baytlık yanıt bekler.
fn olc(ad: &str, nodelay: bool, tek_yazma: bool, sunucu_nodelay: bool) {
    let d = TcpListener::bind("127.0.0.1:0").unwrap();
    let adres = d.local_addr().unwrap();
    let s = thread::spawn(move || {
        let (mut b, _) = d.accept().unwrap();
        b.set_nodelay(sunucu_nodelay).unwrap();
        let mut tampon = [0u8; 8];
        for _ in 0..20 {
            b.read_exact(&mut tampon).unwrap(); // başlık (4) + gövde (4)
            b.write_all(b"T").unwrap(); // tamam
        }
    });
    let mut c = TcpStream::connect(adres).unwrap();
    c.set_nodelay(nodelay).unwrap();
    let t = Instant::now();
    let mut cevap = [0u8; 1];
    for _ in 0..20 {
        if tek_yazma {
            c.write_all(b"BASLGVDE").unwrap(); // başlık ve gövde tek yazmada
        } else {
            c.write_all(b"BASL").unwrap(); // küçük yazma 1
            c.write_all(b"GVDE").unwrap(); // küçük yazma 2: Nagle bunu ilk yazmanın onayı gelene dek bekletir
        }
        c.read_exact(&mut cevap).unwrap();
    }
    let ms = t.elapsed().as_secs_f64() * 1e3;
    println!("{ad:<40}: 20 istek {ms:>6.1} ms (istek başına {:.2} ms)", ms / 20.0);
    s.join().unwrap();
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("sunucu-nodelay") {
        // Kendin dene: TCP_NODELAY'i istemci yerine sunucu tarafında açmak
        olc("iki küçük yazma, yalnız sunucuda NODELAY", false, false, true);
        return;
    }
    // 1) TCP bir bayt akışıdır: yazma sınırları korunmaz
    let d = TcpListener::bind("127.0.0.1:0").unwrap();
    let adres = d.local_addr().unwrap();
    let s = thread::spawn(move || {
        let (mut b, _) = d.accept().unwrap();
        thread::sleep(Duration::from_millis(200)); // istemci üç kez yazsın
        let mut tampon = [0u8; 64];
        let n = b.read(&mut tampon).unwrap();
        println!("sunucu tek read ile {n} bayt aldı: {:?}", String::from_utf8_lossy(&tampon[..n]));
    });
    let mut c = TcpStream::connect(adres).unwrap();
    for parca in ["bir|", "iki|", "üç|"] {
        c.write_all(parca.as_bytes()).unwrap(); // üç ayrı write çağrısı
    }
    s.join().unwrap();

    // 2) Nagle algoritması + gecikmeli ACK: "yaz, yaz, oku" kalıbı
    olc("iki küçük yazma, TCP_NODELAY = false", false, false, false);
    olc("iki küçük yazma, TCP_NODELAY = true", true, false, false);
    olc("tek yazma,       TCP_NODELAY = false", false, true, false);
}
