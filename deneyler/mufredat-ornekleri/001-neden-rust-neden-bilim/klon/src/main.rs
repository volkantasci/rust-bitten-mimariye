fn main() {
    let isim = String::from("Ferris");
    let referans = &isim.clone();    // derleyicinin önerisi: referans artık bir kopyaya bakar
    drop(isim);                      // asıl değer iade edilir
    println!("isim = {referans}");   // kopya hâlâ yaşıyor
}
