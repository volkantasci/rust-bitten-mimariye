use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
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
    for nodelay in [false, true] {
        let d = TcpListener::bind("127.0.0.1:0").unwrap();
        let adres = d.local_addr().unwrap();
        let s = thread::spawn(move || {
            let (mut b, _) = d.accept().unwrap();
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
            c.write_all(b"BASL").unwrap(); // küçük yazma 1
            c.write_all(b"GVDE").unwrap(); // küçük yazma 2: Nagle bunu ilk ACK gelene dek bekletir
            c.read_exact(&mut cevap).unwrap();
        }
        let ms = t.elapsed().as_secs_f64() * 1e3;
        println!("TCP_NODELAY = {nodelay:<5}: 20 istek {ms:>7.1} ms (istek başına {:.2} ms)", ms / 20.0);
        s.join().unwrap();
    }
}
