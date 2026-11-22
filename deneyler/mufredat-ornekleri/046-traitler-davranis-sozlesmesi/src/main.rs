trait Sekil {
    fn alan(&self) -> f64;
    fn ad(&self) -> String;
    // Varsayılan metot: uygulayan her tip bedavaya alır
    fn tanit(&self) -> String {
        format!("{} (alan {:.2})", self.ad(), self.alan())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Daire { r: f64 }

#[derive(Debug, Clone, PartialEq)]
struct Kare { kenar: f64 }

impl Sekil for Daire {
    fn alan(&self) -> f64 { std::f64::consts::PI * self.r * self.r }
    fn ad(&self) -> String { "daire".into() }
}

impl Sekil for Kare {
    fn alan(&self) -> f64 { self.kenar * self.kenar }
    fn ad(&self) -> String { "kare".into() }
    fn tanit(&self) -> String { format!("{} kenarlı kare", self.kenar) } // geçersiz kılma
}

fn main() {
    let d = Daire { r: 2.0 };
    let k = Kare { kenar: 3.0 };
    println!("{}", d.tanit());
    println!("{}", k.tanit());
    println!("{:?} == {:?} → {}", d, d.clone(), d == d.clone()); // derive ile gelen davranışlar
}
