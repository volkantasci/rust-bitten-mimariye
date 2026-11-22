fn main() {
    // Eratosthenes eleği: 20 milyona kadar asal sayıları say
    let n = 20_000_000;
    let mut asal = vec![true; n + 1];
    asal[0] = false;
    asal[1] = false;
    let mut i = 2;
    while i * i <= n {
        if asal[i] {
            let mut j = i * i;
            while j <= n {
                asal[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    println!("{} asal", asal.iter().filter(|&&a| a).count());
}
