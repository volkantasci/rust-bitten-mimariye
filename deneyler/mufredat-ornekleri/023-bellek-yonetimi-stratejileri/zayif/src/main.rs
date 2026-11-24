// Kendin dene: çevrimin bir yönünü Weak (sayılmayan referans) yapınca ikisi de bırakılır.
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Dugum {
    ad: char,
    sahip: RefCell<Option<Rc<Dugum>>>,
    geri: RefCell<Weak<Dugum>>, // geri bağlantı sayaca eklenmez
}

impl Drop for Dugum {
    fn drop(&mut self) {
        println!("  {} bırakıldı", self.ad);
    }
}

fn main() {
    let x = Rc::new(Dugum { ad: 'x', sahip: RefCell::new(None), geri: RefCell::new(Weak::new()) });
    let y = Rc::new(Dugum { ad: 'y', sahip: RefCell::new(None), geri: RefCell::new(Weak::new()) });
    *x.sahip.borrow_mut() = Some(Rc::clone(&y)); // x -> y (sayılır)
    *y.geri.borrow_mut() = Rc::downgrade(&x); // y -> x (sayılmaz)
    println!("x: {} güçlü, {} zayıf; y: {} güçlü", Rc::strong_count(&x), Rc::weak_count(&x), Rc::strong_count(&y));
    drop(x);
    drop(y);
    println!("main bitiyor");
}
