use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let adet: usize = std::env::args().nth(1).map_or(100, |s| s.parse().unwrap());
    let isci = tokio::runtime::Handle::current().metrics().num_workers();
    println!("çalışma zamanı işçi iş parçacığı: {isci}");

    let dinleyici = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let adres = dinleyici.local_addr().unwrap();
    tokio::spawn(async move {
        loop {
            let (mut b, _) = dinleyici.accept().await.unwrap();
            tokio::spawn(async move { // her bağlantıya bir görev (iş parçacığı değil)
                let mut tampon = [0u8; 64];
                let n = b.read(&mut tampon).await.unwrap();
                tokio::time::sleep(Duration::from_millis(100)).await; // yavaş bir arka uç gibi
                b.write_all(&tampon[..n]).await.unwrap();
            });
        }
    });

    let t = Instant::now();
    let mut istemciler = JoinSet::new();
    for i in 0..adet {
        istemciler.spawn(async move {
            let t0 = Instant::now();
            let mut c = TcpStream::connect(adres).await.unwrap();
            c.write_all(format!("istek {i}").as_bytes()).await.unwrap();
            let mut cevap = Vec::new();
            c.read_to_end(&mut cevap).await.unwrap();
            t0.elapsed().as_millis()
        });
    }
    let mut kova = [0usize; 4]; // <0,5 s | 0,5–1,5 s | 1,5–2,5 s | ≥2,5 s
    while let Some(ms) = istemciler.join_next().await {
        let ms = ms.unwrap();
        kova[((ms + 500) / 1000).min(3) as usize] += 1;
    }
    println!("{adet} eşzamanlı istemci (her biri sunucuda 100 ms bekletildi): toplam {:.0} ms", t.elapsed().as_secs_f64() * 1e3);
    println!("  istemci süreleri: <0,5 s: {}  ~1 s: {}  ~2 s: {}  ≥2,5 s: {}", kova[0], kova[1], kova[2], kova[3]);

    // Zaman aşımı: select! ile iki geleceği yarıştır
    let sonuc = tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(5)) => "yavaş iş bitti",
        _ = tokio::time::sleep(Duration::from_millis(50)) => "zaman aşımı: yavaş iş iptal edildi",
    };
    println!("{sonuc}");
}
