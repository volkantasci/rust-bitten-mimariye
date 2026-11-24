use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

struct Dugum {
    ad: &'static str,
    komsu: RefCell<Option<Rc<Dugum>>>,
}
impl Drop for Dugum {
    fn drop(&mut self) {
        println!("  {} bırakıldı", self.ad);
    }
}

struct Ebeveyn {
    ad: &'static str,
    cocuk: RefCell<Option<Rc<Cocuk>>>,
}
struct Cocuk {
    ebeveyn: RefCell<Weak<Ebeveyn>>, // geri bağlantı zayıf: döngü sayılmaz
}
impl Drop for Ebeveyn {
    fn drop(&mut self) {
        println!("  {} bırakıldı", self.ad);
    }
}

fn main() {
    // Cell: & üzerinden kopyalanabilir değeri değiştir
    let sayac = Cell::new(0);
    let artir = || sayac.set(sayac.get() + 1);
    artir();
    artir();
    println!("Cell sayaç: {}", sayac.get());

    // RefCell: ödünç kuralları derleme yerine çalışma anında denetlenir
    let liste = RefCell::new(vec![1, 2]);
    liste.borrow_mut().push(3);
    println!("RefCell: {:?}", liste.borrow());

    println!("--- Rc + RefCell ile döngü:");
    let gozcu = {
        let x = Rc::new(Dugum { ad: "x", komsu: RefCell::new(None) });
        let y = Rc::new(Dugum { ad: "y", komsu: RefCell::new(Some(Rc::clone(&x))) });
        *x.komsu.borrow_mut() = Some(Rc::clone(&y)); // x → y → x
        println!("x sayaç {}, y sayaç {}", Rc::strong_count(&x), Rc::strong_count(&y));
        Rc::downgrade(&x) // gözlemci: sahiplik vermez, sayacı kapsam dışından okumamızı sağlar
    };
    println!("kapsam bitti: x'in güçlü sayacı hâlâ {}, hiçbiri bırakılmadı: bellek sızdı (güvenli ama sızıntı)",
             gozcu.strong_count());

    println!("--- Weak geri bağlantı ile:");
    {
        let e = Rc::new(Ebeveyn { ad: "ebeveyn", cocuk: RefCell::new(None) });
        let c = Rc::new(Cocuk { ebeveyn: RefCell::new(Weak::new()) });
        *c.ebeveyn.borrow_mut() = Rc::downgrade(&e);
        *e.cocuk.borrow_mut() = Some(Rc::clone(&c));
        println!("ebeveyn güçlü {}, zayıf {}", Rc::strong_count(&e), Rc::weak_count(&e));
    }
    println!("kapsam bitti");

    println!("--- aynı anda iki borrow_mut:");
    let hucre = RefCell::new(String::from("tek yazar"));
    let _birinci = hucre.borrow_mut();
    let _ikinci = hucre.borrow_mut(); // çalışma anında panik
}
