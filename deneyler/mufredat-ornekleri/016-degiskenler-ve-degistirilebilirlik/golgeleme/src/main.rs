fn main() {
    let girdi = "42"; // &str
    let girdi: i32 = girdi.parse().unwrap(); // aynı ad, yeni değişken, yeni tip
    let girdi = girdi * 2;
    println!("girdi = {girdi}");

    let mut sayac = 0; // değişebilir olduğunu açıkça söylüyoruz
    for _ in 0..3 {
        sayac += 1;
    }
    println!("sayac = {sayac}");
}
