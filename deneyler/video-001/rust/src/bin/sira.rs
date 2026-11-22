// Aynı satırlar, doğru sırada: önce ödünç kullanılır, sonra değer iade edilir
fn main() {
    let isim = String::from("Ferris");
    let referans = &isim;            // isim'e bakan bir ödünç (referans)
    println!("isim = {referans}");   // önce kullan
    drop(isim);                      // sonra iade et
}
