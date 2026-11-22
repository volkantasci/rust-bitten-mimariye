use std::collections::HashMap;

// Güvenli bir fonksiyon; ama bugünkü ödünç denetleyicisi reddeder.
fn al_veya_ekle(harita: &mut HashMap<u32, String>, anahtar: u32) -> &String {
    if let Some(deger) = harita.get(&anahtar) {
        return deger;
    }
    harita.insert(anahtar, String::from("yeni"));
    harita.get(&anahtar).unwrap()
}

fn main() {
    let mut h = HashMap::new();
    println!("{}", al_veya_ekle(&mut h, 1));
}
