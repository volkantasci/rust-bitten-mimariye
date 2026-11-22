fn main() {
    for n in [1_000usize, 1_000_000, 100_000_000] {
        let v: Vec<u32> = (0..n as u32).collect(); // sıralı veri
        let hedef = v[n - 1]; // en kötü durum: son eleman
        let mut dogrusal = 0u64;
        v.iter().position(|&x| {
            dogrusal += 1;
            x == hedef
        });
        let mut ikili = 0u64;
        let _ = v.binary_search_by(|x| {
            ikili += 1;
            x.cmp(&hedef)
        });
        println!("n = {n:>11}: doğrusal {dogrusal:>11} karşılaştırma | ikili {ikili:>2} (log2 n = {:.1})",
                 (n as f64).log2());
    }
}
