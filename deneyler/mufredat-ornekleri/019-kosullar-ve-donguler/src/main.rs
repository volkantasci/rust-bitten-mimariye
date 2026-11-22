// Collatz: çiftse yarıya böl, tekse 3n+1. Her sayı 1'e ulaşır mı? (Kimse kanıtlayamadı.)
fn adimlar(mut n: u64) -> (u32, u64) {
    let mut adim = 0;
    let mut tepe = n;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        tepe = tepe.max(n);
        adim += 1;
    }
    (adim, tepe)
}

fn main() {
    for n in [6, 7, 27, 97] {
        let (adim, tepe) = adimlar(n);
        println!("n = {n:>3}: {adim:>3} adım, en yüksek değer {tepe}");
    }
    // 1..1_000_000 arasında en uzun yolculuk
    let (en_uzun, n) = (1..1_000_000u64).map(|n| (adimlar(n).0, n)).max().unwrap();
    println!("1..1_000_000 arasında en uzun: n = {n}, {en_uzun} adım");
}
