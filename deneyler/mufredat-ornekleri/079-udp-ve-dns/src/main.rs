use std::net::{ToSocketAddrs, UdpSocket};

fn main() {
    let alici = UdpSocket::bind("127.0.0.1:0").unwrap();
    let gonderen = UdpSocket::bind("127.0.0.1:0").unwrap();
    let hedef = alici.local_addr().unwrap();
    for paket in ["bir", "iki", "üç"] {
        gonderen.send_to(paket.as_bytes(), hedef).unwrap(); // her çağrı ayrı bir datagram
    }
    let mut tampon = [0u8; 1500];
    for _ in 0..3 {
        let (n, kimden) = alici.recv_from(&mut tampon).unwrap();
        println!("{n} bayt datagram ({kimden} kaynaklı): {:?}", String::from_utf8_lossy(&tampon[..n]));
    }
    // Ad çözümleme: "localhost" hangi adreslere karşılık geliyor?
    let adresler: Vec<_> = "localhost:80".to_socket_addrs().unwrap().collect();
    println!("localhost:80 → {adresler:?}");
}
