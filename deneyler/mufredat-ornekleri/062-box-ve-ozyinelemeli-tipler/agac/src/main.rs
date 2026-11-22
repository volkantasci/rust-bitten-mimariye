use std::mem::size_of;

enum Agac {
    Bos,
    Dugum(Box<Agac>, i32, Box<Agac>), // Box: çocuk düğümler öbekte, burada yalnızca işaretçi
}

impl Agac {
    fn ekle(self, x: i32) -> Agac {
        match self {
            Agac::Bos => Agac::Dugum(Box::new(Agac::Bos), x, Box::new(Agac::Bos)),
            Agac::Dugum(sol, v, sag) => {
                if x < v {
                    Agac::Dugum(Box::new((*sol).ekle(x)), v, sag)
                } else {
                    Agac::Dugum(sol, v, Box::new((*sag).ekle(x)))
                }
            }
        }
    }
    fn sirali(&self, cikti: &mut Vec<i32>) {
        if let Agac::Dugum(sol, v, sag) = self {
            sol.sirali(cikti);
            cikti.push(*v);
            sag.sirali(cikti);
        }
    }
    fn derinlik(&self) -> usize {
        match self {
            Agac::Bos => 0,
            Agac::Dugum(sol, _, sag) => 1 + sol.derinlik().max(sag.derinlik()),
        }
    }
}

fn main() {
    let mut a = Agac::Bos;
    for x in [50, 30, 70, 20, 40, 60, 80, 35] {
        a = a.ekle(x);
    }
    let mut v = Vec::new();
    a.sirali(&mut v);
    println!("sıralı gezinme: {v:?}, derinlik {}", a.derinlik());
    println!("size_of::<Box<Agac>>() = {} bayt, size_of::<Agac>() = {} bayt", size_of::<Box<Agac>>(), size_of::<Agac>());
}
