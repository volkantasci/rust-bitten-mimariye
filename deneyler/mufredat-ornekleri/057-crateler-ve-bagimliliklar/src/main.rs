use serde_json::json;

fn main() {
    let kayit = json!({ "ad": "Deniz", "tur": "ahtapot", "kol": 8 });
    println!("{kayit}");
}
