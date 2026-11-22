fn main() {
    // Kararlılık: eşit anahtarlı kayıtlar ilk sıralarını korur mu?
    let mut kayitlar = vec![("Ali", 3), ("Ayşe", 1), ("Can", 3), ("Deniz", 2), ("Ece", 1)];
    kayitlar.sort_by_key(|k| k.1); // kararlı (stable)
    println!("sort_by_key (kararlı): {:?}", kayitlar);

    // Kaç karşılaştırma? n log2 n ile kıyaslayalım
    let n = 1_000_000usize;
    let mut durum = 88172645463325252u64;
    let veri: Vec<u32> = (0..n)
        .map(|_| {
            durum ^= durum << 13;
            durum ^= durum >> 7;
            durum ^= durum << 17;
            durum as u32
        })
        .collect();
    for (ad, kararli) in [("sort (kararlı)", true), ("sort_unstable", false)] {
        let mut v = veri.clone();
        let mut sayac = 0u64;
        let mut kiyas = |a: &u32, b: &u32| {
            sayac += 1;
            a.cmp(b)
        };
        if kararli { v.sort_by(&mut kiyas) } else { v.sort_unstable_by(&mut kiyas) }
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
        println!("{ad:<15}: {sayac:>9} karşılaştırma", );
    }
    println!("n log2 n       : {:>9.0}", n as f64 * (n as f64).log2());
}
