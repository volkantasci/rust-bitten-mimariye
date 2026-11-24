use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // accept'in kaç milisaniye geciktirileceği (varsayılan 300)
    let gecikme: u64 = std::env::args().nth(1).map_or(300, |s| s.parse().unwrap());
    let dinleyici = TcpListener::bind("127.0.0.1:0").unwrap(); // 0: çekirdek boş bir port seçsin
    let adres = dinleyici.local_addr().unwrap();
    println!("sunucu dinliyor: {adres}");
    let t0 = Instant::now();
    let sunucu = thread::spawn(move || {
        thread::sleep(Duration::from_millis(gecikme)); // accept'i bilerek geciktir
        println!("[{:>4} ms] sunucu accept çağırıyor", t0.elapsed().as_millis());
        let (mut baglanti, istemci) = dinleyici.accept().unwrap(); // kuyruktaki hazır bağlantıyı al
        println!("sunucu: {istemci} bağlandı");
        let mut tampon = [0u8; 64];
        let n = baglanti.read(&mut tampon).unwrap();
        let metin = String::from_utf8_lossy(&tampon[..n]).to_uppercase();
        baglanti.write_all(metin.as_bytes()).unwrap(); // yankı (büyük harfle)
    });
    let mut istemci = TcpStream::connect(adres).unwrap(); // el sıkışma: çekirdekler arasında
    println!("[{:>4} ms] istemci bağlandı: {} → {}", t0.elapsed().as_millis(), istemci.local_addr().unwrap(), istemci.peer_addr().unwrap());
    istemci.write_all("merhaba ağ".as_bytes()).unwrap();
    println!("[{:>4} ms] istemci veriyi gönderdi", t0.elapsed().as_millis());
    let mut cevap = String::new();
    istemci.read_to_string(&mut cevap).unwrap(); // sunucu bağlantıyı kapatınca biter
    println!("[{:>4} ms] istemci aldı: {cevap:?}", t0.elapsed().as_millis());
    sunucu.join().unwrap();
}
