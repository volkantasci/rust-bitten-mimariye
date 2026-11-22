trait Kaydedici {
    fn kaydet<T: std::fmt::Debug>(&self, deger: T);
}

struct Ekran;
impl Kaydedici for Ekran {
    fn kaydet<T: std::fmt::Debug>(&self, deger: T) {
        println!("{deger:?}");
    }
}

fn main() {
    let k: &dyn Kaydedici = &Ekran;
    k.kaydet(42);
}
