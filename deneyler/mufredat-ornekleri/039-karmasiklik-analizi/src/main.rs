fn main() {
    for n in [1_000usize, 1_000_000, 100_000_000] {
        let v: Vec<u32> = (0..n as u32).collect(); // sıralı veri: 0, 1, 2, ..., n-1
        // En kötü durum (son eleman) ve en iyi durum (ilk eleman)
        for (ad, hedef) in [("son eleman", v[n - 1]), ("ilk eleman", v[0])] {
            let mut dogrusal = 0u64;
            v.iter().position(|&x| {
                dogrusal += 1; // her karşılaştırmada sayaç bir artar
                x == hedef
            });
            let mut ikili = 0u64;
            let _ = v.binary_search_by(|x| {
                ikili += 1;
                x.cmp(&hedef)
            });
            println!("n = {n:>11}, {ad}: doğrusal {dogrusal:>11} karşılaştırma | ikili {ikili:>2} (log2 n = {:.1})",
                     (n as f64).log2());
        }
    }
}
