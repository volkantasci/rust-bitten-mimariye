fn main() {
    let tampon = [0u8; 8];
    let i = std::env::args().count() + 19; // çalışma anında hesaplanan indis: 20
    println!("tampon[{i}] = {}", tampon[i]);
}
