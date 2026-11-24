use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

// Ham istemci: isteği elle yazılmış baytlar olarak gönderir, yanıtın baytlarını olduğu gibi stdout'a basar
fn istemci() {
    let mut akis = TcpStream::connect("127.0.0.1:8099").unwrap();
    akis.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
    let mut yanit = Vec::new();
    akis.read_to_end(&mut yanit).unwrap(); // sunucu "Connection: close" ile bağlantıyı kapatınca biter
    std::io::stdout().write_all(&yanit).unwrap();
}

fn sunucu(adet: usize, uzunluk: Option<usize>) {
    let dinleyici = TcpListener::bind("127.0.0.1:8099").unwrap();
    for baglanti in dinleyici.incoming().take(adet) {
        let mut baglanti = baglanti.unwrap();
        let mut okuyucu = BufReader::new(baglanti.try_clone().unwrap());
        let mut istek_satiri = String::new();
        okuyucu.read_line(&mut istek_satiri).unwrap(); // "GET / HTTP/1.1\r\n"
        loop {
            let mut satir = String::new();
            okuyucu.read_line(&mut satir).unwrap();
            if satir == "\r\n" {
                break; // başlıklar boş satırla biter
            }
        }
        eprintln!("[sunucu] istek: {}", istek_satiri.trim_end());
        let govde = "Merhaba, HTTP!\n";
        let cevap = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{govde}",
            uzunluk.unwrap_or(govde.len()) // bilerek yanlış bir değer verilebilir
        );
        baglanti.write_all(cevap.as_bytes()).unwrap();
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("istemci") => istemci(),
        Some(n) => sunucu(n.parse().unwrap(), std::env::args().nth(2).map(|s| s.parse().unwrap())),
        None => sunucu(1, None),
    }
}
