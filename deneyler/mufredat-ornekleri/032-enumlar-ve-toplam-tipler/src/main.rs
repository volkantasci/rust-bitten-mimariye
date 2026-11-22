use std::mem::size_of;

enum Sekil {
    Daire { r: f64 },
    Dikdortgen { en: f64, boy: f64 },
    Nokta,
}

fn alan(s: &Sekil) -> f64 {
    match s {
        Sekil::Daire { r } => std::f64::consts::PI * r * r,
        Sekil::Dikdortgen { en, boy } => en * boy,
        Sekil::Nokta => 0.0,
    }
}

fn main() {
    let sekiller = [Sekil::Daire { r: 1.0 }, Sekil::Dikdortgen { en: 2.0, boy: 3.0 }, Sekil::Nokta];
    for s in &sekiller {
        println!("alan = {:.4}", alan(s));
    }
    println!("size_of::<Sekil>() = {} bayt", size_of::<Sekil>());
    println!("size_of::<f64>() * 2 = {} bayt (en büyük varyantın yükü)", size_of::<f64>() * 2);
}
