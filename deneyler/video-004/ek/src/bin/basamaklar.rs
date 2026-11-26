// 1_000_000'un basamakları: konumsal sayı sisteminde her basamağın değeri ve ara toplamlar
fn main() {
    let n: u32 = 1_000_000;

    println!("onluk (taban 10): sıfır olmayan basamaklar");
    let mut toplam = 0;
    for us in (0..10).rev() {
        let agirlik = 10u32.pow(us);
        let rakam = (n / agirlik) % 10;
        if rakam != 0 {
            toplam += rakam * agirlik;
            println!("  {rakam} × 10^{us} = {:>7}", rakam * agirlik);
        }
    }
    println!("  toplam   = {toplam:>7}");

    println!("ikilik (taban 2): 1 olan bitler");
    let mut toplam = 0;
    for us in (0..32).rev() {
        let agirlik = 1u32 << us;
        if n & agirlik != 0 {
            toplam += agirlik;
            println!("  bit {us:>2} : 2^{us:<2} = {agirlik:>7}   ara toplam {toplam:>7}");
        }
    }
    println!("  toplam        = {toplam:>7}");

    println!("onaltılık (taban 16): dörder bit bir hane");
    let mut toplam = 0;
    for us in (0..8).rev() {
        let hane = (n >> (4 * us)) & 0xf;
        let agirlik = 16u32.pow(us);
        toplam += hane * agirlik;
        println!("  {hane:04b} = {hane:x} : {hane:>2} × 16^{us} = {:>7}   ara toplam {toplam:>7}", hane * agirlik);
    }
    println!("  toplam               = {toplam:>7}");

    println!("dört bitin 16 değeri:");
    for a in 0..8u32 {
        let b = a + 8;
        println!("  {a:04b} : {a:x} ({a})      {b:04b} : {b:x} ({b})");
    }
}
