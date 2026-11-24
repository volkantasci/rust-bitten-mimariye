fn main() {
    let isim = String::from("Ferris");
    let referans = &isim;            // isim'e bakan bir ödünç (referans)
    println!("isim = {referans}");   // ödüncün son kullanımı
    drop(isim);                      // artık bakan kimse yok: iade serbest
}
