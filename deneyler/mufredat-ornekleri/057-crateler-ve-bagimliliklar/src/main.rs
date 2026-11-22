use serde_json::json;

fn main() {
    let kayit = json!({ "ad": "Ferris", "tur": "yengeç", "bacak": 10 });
    println!("{kayit}");
}
