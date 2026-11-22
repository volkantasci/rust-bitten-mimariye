use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Olcum {
    sensor: u16,
    sicaklik: f32,
    zaman: u64,
    #[serde(default)] // eski kayıtlarda yoksa varsayılan değer
    birim: String,
}

// Elle ikili kodlama: sabit sıralı alanlar, küçük uçlu
fn ikili_kodla(o: &Olcum) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&o.sensor.to_le_bytes());
    v.extend_from_slice(&o.sicaklik.to_le_bytes());
    v.extend_from_slice(&o.zaman.to_le_bytes());
    v.push(o.birim.len() as u8); // uzunluk öneki
    v.extend_from_slice(o.birim.as_bytes());
    v
}

fn main() {
    let o = Olcum { sensor: 7, sicaklik: 21.5, zaman: 1_763_640_000, birim: "C".into() };
    let json = serde_json::to_string(&o).unwrap();
    let ikili = ikili_kodla(&o);
    println!("JSON  ({} bayt): {json}", json.len());
    println!("ikili ({} bayt): {:02x?}", ikili.len(), ikili);
    let geri: Olcum = serde_json::from_str(&json).unwrap();
    println!("JSON'dan geri okundu, aynı mı: {}", geri == o);
    // Şema evrimi: 'birim' alanı eklenmeden önce yazılmış eski bir kayıt
    let eski = r#"{"sensor":3,"sicaklik":19.0,"zaman":1700000000}"#;
    println!("eski kayıt okundu: {:?}", serde_json::from_str::<Olcum>(eski).unwrap());
    // Bozuk girdi: tip uyuşmazlığı
    println!("{:?}", serde_json::from_str::<Olcum>(r#"{"sensor":"yedi","sicaklik":1.0,"zaman":0}"#).unwrap_err().to_string());
}
