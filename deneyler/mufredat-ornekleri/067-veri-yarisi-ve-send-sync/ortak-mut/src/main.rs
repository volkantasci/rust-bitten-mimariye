// Güvenli Rust: aynı değişkeni iki iş parçacığında birden değiştirmeye çalışmak
use std::thread;

fn main() {
    let mut sayac = 0u64;
    thread::scope(|s| {
        s.spawn(|| sayac += 1);
        s.spawn(|| sayac += 1);
    });
    println!("{sayac}");
}
