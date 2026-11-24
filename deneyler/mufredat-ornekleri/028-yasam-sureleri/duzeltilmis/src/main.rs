// 'a: dönen referans, iki girdiden kısa yaşayanı kadar yaşar
fn uzun<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let birinci = String::from("sahiplik");
    let sonuc;
    {
        let ikinci = String::from("dilim");
        sonuc = uzun(&birinci, &ikinci);
        println!("daha uzun: {sonuc}");
    }
}
