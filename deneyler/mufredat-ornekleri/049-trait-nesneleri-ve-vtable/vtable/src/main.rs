use std::mem::size_of;

trait Sekil {
    fn alan(&self) -> f64;
}

struct Daire { r: f64 }
struct Kare { a: f64 }

impl Sekil for Daire {
    fn alan(&self) -> f64 { 3.14159 * self.r * self.r }
}
impl Sekil for Kare {
    fn alan(&self) -> f64 { self.a * self.a }
}

fn main() {
    println!("&Daire          : {:>2} bayt", size_of::<&Daire>());
    println!("&dyn Sekil      : {:>2} bayt (veri işaretçisi + vtable işaretçisi)", size_of::<&dyn Sekil>());
    println!("Box<dyn Sekil>  : {:>2} bayt", size_of::<Box<dyn Sekil>>());

    // Farklı tipler tek bir listede: her eleman kendi vtable'ının adresini taşır
    let sekiller: Vec<Box<dyn Sekil>> = vec![Box::new(Daire { r: 1.0 }), Box::new(Kare { a: 2.0 })];
    for s in &sekiller {
        println!("alan = {:.3}", s.alan());
    }

    // Mikroskop: geniş işaretçinin içini aç. Bu yerleşim derleyicinin iç ayrıntısıdır, dilin garantisi değil.
    let d: &dyn Sekil = &Daire { r: 1.0 };
    let [veri, vtablo]: [*const usize; 2] = unsafe { std::mem::transmute(d) };
    println!("veri işaretçisi : {veri:p}");
    println!("  Daire'nin adresiyle aynı mı: {}", veri as *const () == d as *const dyn Sekil as *const ());
    println!("vtable          : {vtablo:p}");
    unsafe {
        println!("  [0] drop      : {:#x}", *vtablo);
        println!("  [1] boyut     : {}", *vtablo.add(1));
        println!("  [2] hizalama  : {}", *vtablo.add(2));
        println!("  [3] alan()    : {:#x}", *vtablo.add(3));
    }
    println!("<Daire as Sekil>::alan adresi: {:p}", <Daire as Sekil>::alan as fn(&Daire) -> f64);
}
