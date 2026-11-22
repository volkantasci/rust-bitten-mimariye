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

struct Celcius(f64);
struct Fahrenheit(f64);
impl From<Celcius> for Fahrenheit {
    fn from(c: Celcius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    println!("eşit mi: {}", Etiket("Rust".into()) == Etiket("rust".into()));
    let mut kume = HashSet::new();
    kume.insert(Etiket("Rust".into()));
    kume.insert(Etiket("rust".into()));
    println!("kümede {} eleman var", kume.len());

    let f: Fahrenheit = Celcius(100.0).into(); // From yazınca Into bedava
    println!("100 °C = {} °F", f.0);

    let mut olcumler = vec![2.5, f64::NAN, -1.0, 0.0];
    olcumler.sort_by(|a, b| a.total_cmp(b)); // toplam sıralama: NaN'ın da yeri var
    println!("total_cmp ile: {:?}", olcumler);
    olcumler.sort_by(|a, b| a.partial_cmp(b).unwrap()); // NaN ile kısmi sıralama: None
}
