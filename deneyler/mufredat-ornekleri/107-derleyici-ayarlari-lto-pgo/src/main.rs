use std::hint::black_box;

fn main() {
    // Biraz gerçekçi iş: JSON üret, ayrıştır, topla
    let mut toplam = 0i64;
    for i in 0..20_000 {
        let metin = serde_json::json!({ "id": i, "etiketler": ["a", "b", "c"], "deger": i * 3 }).to_string();
        let v: serde_json::Value = serde_json::from_str(black_box(&metin)).unwrap();
        toplam += v["deger"].as_i64().unwrap();
    }
    println!("{toplam}");
}
