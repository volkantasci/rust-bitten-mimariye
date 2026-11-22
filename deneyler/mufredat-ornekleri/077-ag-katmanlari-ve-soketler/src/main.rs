use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let dinleyici = TcpListener::bind("127.0.0.1:0").unwrap(); // 0: çekirdek boş bir port seçsin
    let adres = dinleyici.local_addr().unwrap();
    println!("sunucu dinliyor: {adres}");
    let sunucu = thread::spawn(move || {
        let (mut baglanti, istemci) = dinleyici.accept().unwrap(); // üçlü el sıkışma tamamlandı
        println!("sunucu: {istemci} bağlandı");
        let mut tampon = [0u8; 64];
        let n = baglanti.read(&mut tampon).unwrap();
        let metin = String::from_utf8_lossy(&tampon[..n]).to_uppercase();
        baglanti.write_all(metin.as_bytes()).unwrap(); // yankı (büyük harfle)
    });
    let mut istemci = TcpStream::connect(adres).unwrap();
    println!("istemci: {} → {}", istemci.local_addr().unwrap(), istemci.peer_addr().unwrap());
    istemci.write_all("merhaba ağ".as_bytes()).unwrap();
    let mut cevap = String::new();
    istemci.read_to_string(&mut cevap).unwrap(); // sunucu bağlantıyı kapatınca biter
    println!("istemci aldı: {cevap:?}");
    sunucu.join().unwrap();
}
