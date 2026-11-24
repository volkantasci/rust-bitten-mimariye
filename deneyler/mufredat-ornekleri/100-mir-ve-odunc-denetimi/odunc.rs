// Ödünç, son kullanıldığı yere kadar yaşar; denetim her yol için ayrı yapılır.
pub fn sira_dogru(v: &mut Vec<i32>) {
    let ilk = &v[0];
    println!("{ilk}"); // ilk'in son kullanımı
    v.push(4); // artık &mut ödünç almak serbest
}

pub fn yola_gore(v: &mut Vec<i32>, kosul: bool) {
    let ilk = &v[0];
    if kosul {
        v.push(4); // bu yolda ilk bir daha kullanılmıyor
    } else {
        println!("{ilk}"); // ilk yalnız bu yolda canlı
    }
}

pub fn sira_yanlis(v: &mut Vec<i32>) {
    let ilk = &v[0];
    v.push(4); // ilk hâlâ canlı: push için &mut gerekir
    println!("{ilk}");
}
