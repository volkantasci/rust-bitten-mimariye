use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher, RandomState};

fn main() {
    let sehirler = ["Ankara", "İzmir", "Bursa", "Van", "Rize", "Muş", "Kars"];
    let h: HashMap<&str, usize> = sehirler.iter().map(|s| (*s, s.len())).collect();
    let b: BTreeMap<&str, usize> = sehirler.iter().map(|s| (*s, s.len())).collect();
    println!("HashMap sırası : {:?}", h.keys().collect::<Vec<_>>());
    println!("BTreeMap sırası: {:?}", b.keys().collect::<Vec<_>>());

    // Tablonun kendi hasher'ı: SipHash-1-3; anahtarları (tohum) her iş parçacığında ilk RandomState
    // oluşturulurken işletim sisteminden bir kez alınır ve önbelleğe konur
    println!("hash(\"Ankara\") = {:016x}  (tablonun kendi hasher'ıyla)", h.hasher().hash_one("Ankara"));
    // Aynı süreçte yeni bir RandomState: önbellekteki anahtarlardan biri bir artırılır, hash değişir
    let yeni = RandomState::new();
    println!("aynı süreçte yeni bir RandomState aynı hash'i verdi mi: {}",
             yeni.hash_one("Ankara") == h.hasher().hash_one("Ankara"));

    // Tohumu sabitlersek (anahtarlar 0, 0) sıra her çalıştırmada aynı olur
    let sabit: HashMap<&str, usize, BuildHasherDefault<DefaultHasher>> =
        sehirler.iter().map(|s| (*s, s.len())).collect();
    println!("sabit tohumlu HashMap sırası: {:?}", sabit.keys().collect::<Vec<_>>());

    // &str'nin Ord'u baytları sırayla karşılaştırır, BTreeMap bu sırayı kullanır: 'İ' UTF-8'de iki bayttır ve ilki ASCII harflerden büyüktür
    println!("ilk bayt: \"İzmir\" {:#04x}, \"Van\" {:#04x}", "İzmir".as_bytes()[0], "Van".as_bytes()[0]);
}
