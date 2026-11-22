// Derleyicinin önerisi ("help: consider cloning the value ...") 3. satıra olduğu gibi uygulandı
fn main() {
    let isim = String::from("Ferris");
    let referans = &isim.clone();            // isim'e bakan bir ödünç (referans)
    drop(isim);                      // belleği iade etmeye çalış
    println!("isim = {referans}");   // iade edilmiş belleği okumak olurdu
}
