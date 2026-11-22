use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

// Çerçeve: [4 bayt uzunluk, büyük uçlu (ağ bayt sırası)] [o kadar bayt yük]
fn cerceve_yaz(akis: &mut impl Write, yuk: &[u8]) -> io::Result<()> {
    akis.write_all(&(yuk.len() as u32).to_be_bytes())?;
    akis.write_all(yuk)
}

fn cerceve_oku(akis: &mut impl Read) -> io::Result<Vec<u8>> {
    let mut uzunluk = [0u8; 4];
    akis.read_exact(&mut uzunluk)?; // tam 4 bayt gelene dek bekle
    let mut yuk = vec![0u8; u32::from_be_bytes(uzunluk) as usize];
    akis.read_exact(&mut yuk)?; // tam yük gelene dek bekle
    Ok(yuk)
}

fn main() {
    let mut ornek = Vec::new();
    cerceve_yaz(&mut ornek, b"merhaba").unwrap();
    println!("tel üzerindeki baytlar: {:02x?}", ornek);

    let d = TcpListener::bind("127.0.0.1:0").unwrap();
    let adres = d.local_addr().unwrap();
    let sunucu = thread::spawn(move || {
        // Saf sunucu: "bir read = bir mesaj" sanıyor
        let (mut b, _) = d.accept().unwrap();
        thread::sleep(Duration::from_millis(200));
        let mut tampon = [0u8; 256];
        let n = b.read(&mut tampon).unwrap();
        println!("saf sunucu tek read'de {n} bayt aldı (3 mesaj yapışık geldi)");
        // Çerçeveli sunucu: aynı baytlardan mesajları geri kur
        let (mut b2, _) = d.accept().unwrap();
        for _ in 0..3 {
            let m = cerceve_oku(&mut b2).unwrap();
            println!("çerçeveli sunucu mesaj aldı: {:?}", String::from_utf8_lossy(&m));
        }
    });
    for _ in 0..2 {
        let mut c = TcpStream::connect(adres).unwrap();
        for m in ["GET anahtar", "SET anahtar 42", "DEL anahtar"] {
            cerceve_yaz(&mut c, m.as_bytes()).unwrap();
        }
        thread::sleep(Duration::from_millis(300));
    }
    sunucu.join().unwrap();
}
