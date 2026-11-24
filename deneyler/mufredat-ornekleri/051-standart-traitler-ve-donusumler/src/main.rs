use std::collections::HashSet;
use std::hash::{Hash, Hasher};

// Eşitlik büyük/küçük harfe duyarsız; ama hash duyarlı: SÖZLEŞME BOZUK
#[derive(Debug)]
struct Etiket(String);

impl PartialEq for Etiket {
    fn eq(&self, diger: &Self) -> bool {
        self.0.to_lowercase() == diger.0.to_lowercase()
    }
}
impl Eq for Etiket {}
impl Hash for Etiket {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.hash(h) // a == b iken hash(a) != hash(b) olabilir
    }
}

struct Celsius(f64);
struct Fahrenheit(f64);
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    println!("eşit mi: {}", Etiket("Rust".into()) == Etiket("rust".into()));
    // Sözleşme bozuk olduğu için sonuç tohuma bağlıdır: 1000 yeni küme (her biri yeni bir RandomState)
    let mut bir = 0;
    for _ in 0..1000 {
        let mut kume = HashSet::new();
        kume.insert(Etiket("Rust".into()));
        kume.insert(Etiket("rust".into()));
        if kume.len() == 1 {
            bir += 1;
        }
    }
    println!("1000 yeni küme: 2 elemanlı {}, 1 elemanlı {}", 1000 - bir, bir);

    let f: Fahrenheit = Celsius(100.0).into(); // From yazınca Into bedava
    println!("100 °C = {} °F", f.0);

    // -f64::NAN işaret biti 1 olan bir NaN'dır; total_cmp işaret bitini de sıralar
    let mut olcumler = vec![2.5, f64::NAN, -1.0, 0.0, -f64::NAN];
    olcumler.sort_by(|a, b| a.total_cmp(b)); // tam sıralama: NaN'ların da yeri var
    let yazi: Vec<String> = olcumler
        .iter()
        .map(|x| match (x.is_nan(), x.is_sign_negative()) {
            (true, true) => "-NaN".to_string(),
            (true, false) => "+NaN".to_string(),
            _ => x.to_string(),
        })
        .collect();
    println!("total_cmp ile: [{}]", yazi.join(", "));
    olcumler.sort_by(|a, b| a.partial_cmp(b).unwrap()); // NaN ile kısmi sıralama: None
}
