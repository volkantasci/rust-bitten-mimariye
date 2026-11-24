use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// std ile engellemeyen soket: hazır bağlantı yoksa accept beklemez, WouldBlock döner
fn std_engellemeyen(engellemeyen: bool) {
    let dinleyici = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    dinleyici.set_nonblocking(engellemeyen).unwrap(); // false: varsayılan, engelleyen soket
    match dinleyici.accept() {
        Ok(_) => println!("beklenmedik: bağlantı vardı"),
        Err(e) => println!("engellemeyen accept: {e} ({:?})", e.kind()),
    }
}

async fn yanki() {
    let dinleyici = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let adres = dinleyici.local_addr().unwrap();
    let sunucu = tokio::spawn(async move {
        let (mut b, _) = dinleyici.accept().await.unwrap(); // hazır değilse görev bekler, döngü başka işe bakar
        let mut tampon = [0u8; 32];
        let n = b.read(&mut tampon).await.unwrap();
        b.write_all(&tampon[..n]).await.unwrap();
    });
    let mut c = TcpStream::connect(adres).await.unwrap();
    c.write_all(b"yanki").await.unwrap();
    let mut cevap = [0u8; 5];
    c.read_exact(&mut cevap).await.unwrap();
    println!("yankı: {}", String::from_utf8_lossy(&cevap));
    sunucu.await.unwrap();
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("std") => return std_engellemeyen(true),
        Some("std-engelleyen") => return std_engellemeyen(false),
        _ => {}
    }
    // Tek iş parçacıklı çalışma zamanı: tek bir olay döngüsü
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(yanki());
}
