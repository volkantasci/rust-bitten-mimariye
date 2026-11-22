// Aynı hata, ama iade işlemi başka bir fonksiyonda (C'deki kullan_sonra_birak2.c gibi)
fn temizle(s: String) {
    drop(s);
}

fn main() {
    let isim = String::from("Ferris");
    let referans = &isim;            // isim'e bakan bir ödünç (referans)
    temizle(isim);                   // belleği başka bir fonksiyonda iade etmeye çalış
    println!("isim = {referans}");   // iade edilmiş belleği okumak olurdu
}
