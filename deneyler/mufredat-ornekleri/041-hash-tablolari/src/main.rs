use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, RandomState};

fn main() {
    let sehirler = ["Ankara", "İzmir", "Bursa", "Van", "Rize", "Muş", "Kars"];
    let h: HashMap<&str, usize> = sehirler.iter().map(|s| (*s, s.len())).collect();
    let b: BTreeMap<&str, usize> = sehirler.iter().map(|s| (*s, s.len())).collect();
    println!("HashMap sırası : {:?}", h.keys().collect::<Vec<_>>());
    println!("BTreeMap sırası: {:?}", b.keys().collect::<Vec<_>>());
    let durum = RandomState::new(); // her süreçte rastgele anahtarlı SipHash
    println!("hash(\"Ankara\") = {:016x}", durum.hash_one("Ankara"));
}
