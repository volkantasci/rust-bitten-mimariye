trait Sekil {
    fn alan(&self) -> f64;
    fn ad(&self) -> String;
}

struct Ucgen { taban: f64, yukseklik: f64 }

impl Sekil for Ucgen {
    // alan() yazılmadı: sözleşmenin bir maddesi eksik
    fn ad(&self) -> String { "üçgen".into() }
}

fn main() {
    let u = Ucgen { taban: 4.0, yukseklik: 3.0 };
    println!("{} {} {}", u.ad(), u.taban, u.yukseklik);
}
