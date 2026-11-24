use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Olcum {
    sensor: u16,
    sicaklik: f32,
    zaman: u64,
    #[serde(default)] // eski kayıtlarda yoksa varsayılan değer
    birim: String,
}

// Elle ikili kodlama: sabit sıralı alanlar, little-endian, metin için 1 baytlık uzunluk öneki
fn ikili_kodla(o: &Olcum) -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(&o.sensor.to_le_bytes());
    v.extend_from_slice(&o.sicaklik.to_le_bytes());
    v.extend_from_slice(&o.zaman.to_le_bytes());
    v.push(o.birim.len() as u8);
    v.extend_from_slice(o.birim.as_bytes());
    v
}

// Çözme: her alanı okumadan önce yeterli bayt var mı diye bak; eksikse hata dön
fn ikili_coz(mut b: &[u8]) -> Result<Olcum, String> {
    fn al<'a>(b: &mut &'a [u8], n: usize, alan: &str) -> Result<&'a [u8], String> {
        if b.len() < n {
            return Err(format!("{alan} için {n} bayt gerekli, {} bayt kaldı", b.len()));
        }
        let (bas, kalan) = b.split_at(n);
        *b = kalan;
        Ok(bas)
    }
    let sensor = u16::from_le_bytes(al(&mut b, 2, "sensor")?.try_into().unwrap());
    let sicaklik = f32::from_le_bytes(al(&mut b, 4, "sicaklik")?.try_into().unwrap());
    let zaman = u64::from_le_bytes(al(&mut b, 8, "zaman")?.try_into().unwrap());
    let n = al(&mut b, 1, "birim uzunluğu")?[0] as usize;
    let birim = String::from_utf8(al(&mut b, n, "birim")?.to_vec()).map_err(|e| e.to_string())?;
    Ok(Olcum { sensor, sicaklik, zaman, birim })
}

// Kendin dene: şemaya varsayılansız yeni bir alan eklenmiş sürüm ve Option'lı sürüm
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OlcumKonumlu {
    sensor: u16,
    sicaklik: f32,
    zaman: u64,
    konum: String, // #[serde(default)] yok
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OlcumKonumSecimli {
    sensor: u16,
    sicaklik: f32,
    zaman: u64,
    konum: Option<String>, // eksikse None
}

// Kendin dene: çözücü alanları yanlış sırayla okursa (önce sicaklik, sonra sensor)
fn ikili_coz_ters_sira(b: &[u8]) -> (f32, u16) {
    (f32::from_le_bytes(b[0..4].try_into().unwrap()), u16::from_le_bytes(b[4..6].try_into().unwrap()))
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("kendin-dene") {
        let eski = r#"{"sensor":3,"sicaklik":19.0,"zaman":1700000000}"#;
        println!("varsayılansız yeni alan: {:?}", serde_json::from_str::<OlcumKonumlu>(eski).unwrap_err().to_string());
        println!("Option'lı yeni alan    : {:?}", serde_json::from_str::<OlcumKonumSecimli>(eski).unwrap());
        let o = Olcum { sensor: 7, sicaklik: 21.5, zaman: 1_763_640_000, birim: "C".into() };
        let (sicaklik, sensor) = ikili_coz_ters_sira(&ikili_kodla(&o));
        println!("ters sırayla çözülen: sicaklik = {sicaklik:e}, sensor = {sensor}");
        return;
    }
    let o = Olcum { sensor: 7, sicaklik: 21.5, zaman: 1_763_640_000, birim: "C".into() };
    let json = serde_json::to_string(&o).unwrap();
    let ikili = ikili_kodla(&o);
    println!("JSON  ({} bayt): {json}", json.len());
    println!("ikili ({} bayt): {:02x?}", ikili.len(), ikili);
    println!("JSON'dan geri okundu, aynı mı: {}", serde_json::from_str::<Olcum>(&json).unwrap() == o);
    println!("ikiliden geri okundu, aynı mı: {}", ikili_coz(&ikili).unwrap() == o);
    // Şema evrimi: 'birim' alanı eklenmeden önce yazılmış eski bir kayıt
    let eski = r#"{"sensor":3,"sicaklik":19.0,"zaman":1700000000}"#;
    println!("eski kayıt okundu: {:?}", serde_json::from_str::<Olcum>(eski).unwrap());
    // Bozuk girdi: tip uyuşmazlığı ve yarıda kesilmiş ikili kayıt
    println!("{:?}", serde_json::from_str::<Olcum>(r#"{"sensor":"yedi","sicaklik":1.0,"zaman":0}"#).unwrap_err().to_string());
    println!("ilk 10 baytı gelen ikili kayıt: {:?}", ikili_coz(&ikili[..10]));
}
