use std::net::{ToSocketAddrs, UdpSocket};

fn main() {
    if std::env::args().nth(1).as_deref() == Some("coz") {
        // Ad çözümleme: "localhost" hangi adreslere karşılık geliyor?
        let ad = std::env::args().nth(2).unwrap_or("localhost".into());
        let adresler: Vec<_> = format!("{ad}:80").to_socket_addrs().unwrap().collect();
        println!("{ad}:80 → {adresler:?}");
        return;
    }
    // 1) Her send_to ayrı bir datagram; her recv_from tam bir datagram döndürür
    let alici = UdpSocket::bind("127.0.0.1:0").unwrap();
    let gonderen = UdpSocket::bind("127.0.0.1:0").unwrap();
    let hedef = alici.local_addr().unwrap();
    for paket in ["bir", "iki", "üç"] {
        gonderen.send_to(paket.as_bytes(), hedef).unwrap();
    }
    let mut tampon = [0u8; 1500];
    for _ in 0..3 {
        let (n, kimden) = alici.recv_from(&mut tampon).unwrap();
        println!("{n} bayt datagram ({kimden} kaynaklı): {:?}", String::from_utf8_lossy(&tampon[..n]));
    }

    // 2) Bağlantısız: kimsenin dinlemediği bir porta gönderim de "başarılı" döner
    let bos_port = UdpSocket::bind("127.0.0.1:0").unwrap().local_addr().unwrap(); // al ve hemen bırak
    let s = UdpSocket::bind("127.0.0.1:0").unwrap();
    s.connect(bos_port).unwrap(); // UDP'de connect yalnız varsayılan hedefi kaydeder; paket gitmez
    println!("dinleyeni olmayan {bos_port} portuna send: {:?}", s.send(b"kimse yok mu?"));
    s.set_read_timeout(Some(std::time::Duration::from_millis(500))).unwrap();
    match s.recv(&mut tampon) {
        Ok(n) => println!("beklenmedik: {n} bayt"),
        Err(e) => println!("sonraki recv: {e} ({:?})", e.kind()),
    }
}
