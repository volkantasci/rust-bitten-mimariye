use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const EN_BUYUK_YUK: u32 = 1 << 20; // protokolün kabul ettiği en büyük mesaj: 1 MiB

// Çerçeve: [4 bayt uzunluk, big-endian (ağ bayt sırası)] [o kadar bayt yük]
fn cerceve_yaz(akis: &mut impl Write, yuk: &[u8]) -> io::Result<()> {
    akis.write_all(&(yuk.len() as u32).to_be_bytes())?;
    akis.write_all(yuk)
}

fn cerceve_oku(akis: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut uzunluk = [0u8; 4];
    akis.read_exact(&mut uzunluk)?; // tam 4 bayt gelene dek bekle
    let n = u32::from_be_bytes(uzunluk);
    if n > EN_BUYUK_YUK {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("çerçeve çok büyük: {n} bayt")));
    }
    let mut yuk = vec![0u8; n as usize];
    akis.read_exact(&mut yuk)?; // tam yük gelene dek bekle
    Ok(yuk)
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("kendin-dene") {
        // Gönderen uzunluğu little-endian yazar, alıcı big-endian okur
        let mut yanlis = Vec::new();
        yanlis.extend_from_slice(&7u32.to_le_bytes());
        yanlis.extend_from_slice(b"merhaba");
        println!("tel üzerindeki baytlar: {:02x?}", yanlis);
        println!("big-endian okuyan alıcının gördüğü uzunluk: {}", u32::from_be_bytes(yanlis[..4].try_into().unwrap()));
        println!("çerçeve okuma: {:?}", cerceve_oku(&mut &yanlis[..]).unwrap_err().to_string());
        return;
    }
    let mut ornek = Vec::new();
    cerceve_yaz(&mut ornek, b"merhaba").unwrap();
    println!("tel üzerindeki baytlar: {:02x?}", ornek);

    let d = TcpListener::bind("127.0.0.1:0").unwrap();
    let adres = d.local_addr().unwrap();
    let sunucu = thread::spawn(move || {
        // 1) Saf sunucu "bir read = bir mesaj" sanıyor; istemci üç mesajı art arda yazdı
        let (mut b, _) = d.accept().unwrap();
        thread::sleep(Duration::from_millis(200));
        let mut tampon = [0u8; 256];
        let n = b.read(&mut tampon).unwrap();
        println!("saf sunucu tek read'de {n} bayt aldı (3 mesaj yapışık geldi)");
        // 2) Çerçeveli sunucu: aynı baytlardan mesajları geri kurar
        let (mut b, _) = d.accept().unwrap();
        for _ in 0..3 {
            let m = cerceve_oku(&mut b).unwrap();
            println!("çerçeveli sunucu mesaj aldı: {:?}", String::from_utf8_lossy(&m));
        }
        // 3) Kısmi okuma: istemci bir mesajı iki parça hâlinde, araya 200 ms koyarak gönderiyor
        let (mut b, _) = d.accept().unwrap();
        let n = b.read(&mut tampon).unwrap();
        println!("saf sunucu tek read'de {n} bayt aldı (mesajın yalnız başı)");
        let (mut b, _) = d.accept().unwrap();
        let m = cerceve_oku(&mut b).unwrap();
        println!("çerçeveli sunucu kalanı bekledi, mesajı tam aldı: {:?}", String::from_utf8_lossy(&m));
        // 4) Bozuk ya da kötü niyetli uzunluk: 4 GiB'lık ayırma denenmez, çerçeve reddedilir
        let (mut b, _) = d.accept().unwrap();
        println!("dev uzunluklu çerçeve: {:?}", cerceve_oku(&mut b).unwrap_err().to_string());
    });
    for _ in 0..2 {
        let mut c = TcpStream::connect(adres).unwrap();
        for m in ["GET anahtar", "SET anahtar 42", "DEL anahtar"] {
            cerceve_yaz(&mut c, m.as_bytes()).unwrap();
        }
        thread::sleep(Duration::from_millis(300));
    }
    // Kısmi okuma: önce uzunluk ve yükün ilk 3 baytı, 200 ms sonra kalanı (iki bağlantıda aynısı)
    let mut mesaj = Vec::new();
    cerceve_yaz(&mut mesaj, b"GET anahtar").unwrap();
    for _ in 0..2 {
        let mut c = TcpStream::connect(adres).unwrap();
        c.write_all(&mesaj[..7]).unwrap();
        thread::sleep(Duration::from_millis(200));
        c.write_all(&mesaj[7..]).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    // Uzunluk alanında ff ff ff ff (4 294 967 295)
    let mut c2 = TcpStream::connect(adres).unwrap();
    c2.write_all(&[0xff, 0xff, 0xff, 0xff, b'x']).unwrap();
    sunucu.join().unwrap();
}
