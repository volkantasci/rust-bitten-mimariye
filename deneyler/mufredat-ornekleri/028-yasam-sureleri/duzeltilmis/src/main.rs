// 'a: dönen referans, iki girdiden kısa yaşayanı kadar yaşar
fn uzun<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let birinci = String::from("ödünç");
    let sonuc;
    {
        let ikinci = String::from("sahiplik");
        sonuc = uzun(&birinci, &ikinci);
        println!("daha uzun: {sonuc}");
    }
}
