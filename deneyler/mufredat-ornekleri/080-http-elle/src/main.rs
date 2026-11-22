use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    let adet: usize = std::env::args().nth(1).map_or(1, |s| s.parse().unwrap());
    let dinleyici = TcpListener::bind("127.0.0.1:8099").unwrap();
    for baglanti in dinleyici.incoming().take(adet) {
        let mut baglanti = baglanti.unwrap();
        let mut okuyucu = BufReader::new(baglanti.try_clone().unwrap());
        let mut istek_satiri = String::new();
        okuyucu.read_line(&mut istek_satiri).unwrap(); // "GET / HTTP/1.1"
        loop {
            let mut satir = String::new();
            okuyucu.read_line(&mut satir).unwrap();
            if satir == "\r\n" { break; } // başlıklar boş satırla biter
        }
        eprintln!("[sunucu] istek: {}", istek_satiri.trim_end());
        let govde = "Merhaba, HTTP!\n";
        let cevap = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{govde}",
            govde.len()
        );
        baglanti.write_all(cevap.as_bytes()).unwrap();
    }
}
