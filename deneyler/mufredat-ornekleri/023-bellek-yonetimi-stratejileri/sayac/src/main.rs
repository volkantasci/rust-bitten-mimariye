// Referans sayımı: Rc her sahibi sayar; sayaç sıfıra inince değer hemen bırakılır.
use std::cell::RefCell;
use std::rc::Rc;

struct Dugum {
    ad: char,
    diger: RefCell<Option<Rc<Dugum>>>, // başka bir düğüme sahip olabilir (RefCell: K14)
}

impl Drop for Dugum {
    fn drop(&mut self) {
        println!("  {} bırakıldı", self.ad);
    }
}

fn dugum(ad: char) -> Rc<Dugum> {
    Rc::new(Dugum { ad, diger: RefCell::new(None) })
}

fn main() {
    let a = dugum('a');
    let a2 = Rc::clone(&a); // ikinci sahip: yalnızca sayaç artar
    println!("a'nın sahip sayısı: {}", Rc::strong_count(&a));
    drop(a2);
    println!("a'nın sahip sayısı: {}", Rc::strong_count(&a));
    drop(a); // sayaç 0: hemen bırakılır
    println!("--- çevrim: x -> y -> x");
    let x = dugum('x');
    let y = dugum('y');
    *x.diger.borrow_mut() = Some(Rc::clone(&y));
    *y.diger.borrow_mut() = Some(Rc::clone(&x));
    println!("x: {}, y: {}", Rc::strong_count(&x), Rc::strong_count(&y));
    let (gozlem_x, gozlem_y) = (Rc::downgrade(&x), Rc::downgrade(&y)); // yalnızca gözlem için (Weak: K14)
    drop(x);
    drop(y); // sayaçlar 1'de kalır: kimse bırakılmaz
    println!("drop sonrası x: {}, y: {}", gozlem_x.strong_count(), gozlem_y.strong_count());
    println!("main bitiyor");
}
