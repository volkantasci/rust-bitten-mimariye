mod sirali {
    /// Tip değişmezi: `ogeler` her an küçükten büyüğe sıralıdır.
    pub struct SiraliListe {
        pub ogeler: Vec<i32>, // AÇIK: herkes doğrudan değiştirebilir
    }

    impl SiraliListe {
        pub fn new() -> Self {
            SiraliListe { ogeler: Vec::new() }
        }
        /// Yeni öğeyi sıranın bozulmayacağı yere koyar.
        pub fn ekle(&mut self, x: i32) {
            let yer = self.ogeler.binary_search(&x).unwrap_or_else(|y| y);
            self.ogeler.insert(yer, x);
        }
        /// İkili arama yalnız sıralı dizide doğru çalışır: değişmez burada işe yarar.
        pub fn iceriyor(&self, x: i32) -> bool {
            self.ogeler.binary_search(&x).is_ok()
        }
        pub fn ogeler(&self) -> &[i32] {
            &self.ogeler
        }
    }
}

fn main() {
    let mut liste = sirali::SiraliListe::new();
    for x in [5, 1, 4, 2, 3] {
        liste.ekle(x);
    }
    liste.ogeler.push(0); // değişmezi dışarıdan bozmaya çalış: sona 0 ekle
    println!("liste: {:?}", liste.ogeler());
    println!("0 var mı: {}, 4 var mı: {}", liste.iceriyor(0), liste.iceriyor(4));
}
