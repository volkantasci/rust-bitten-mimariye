use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main(flavor = "current_thread")] // tek iş parçacığı: tek bir olay döngüsü
async fn main() {
    let dinleyici = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let adres = dinleyici.local_addr().unwrap();
    let sunucu = tokio::spawn(async move {
        let (mut b, _) = dinleyici.accept().await.unwrap(); // hazır değilse görev uyur, döngü başka işe bakar
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
