// Ajan Masası (K8): kullanıcının yazdığı port numarasını okuyan bir ajan kodu.
fn main() {
    let girdi = std::env::args().nth(1).unwrap_or_default();
    let port: u16 = girdi.trim().parse().unwrap(); // ajanın kodu
    println!("sunucu {port} numaralı portta başlıyor");
}
