fn main() {
    let isim = String::from("Ferris");
    let referans = &isim;            // isim'e bakan bir ödünç (referans)
    drop(isim);                      // belleği iade etmeye çalış
    println!("isim = {referans}");   // iade edilmiş belleği okumak olurdu
}
