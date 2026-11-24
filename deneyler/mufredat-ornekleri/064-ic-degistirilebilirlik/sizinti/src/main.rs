use std::cell::RefCell;
use std::rc::Rc;

struct Dugum {
    komsu: RefCell<Option<Rc<Dugum>>>,
}

fn main() {
    let x = Rc::new(Dugum { komsu: RefCell::new(None) });
    let y = Rc::new(Dugum { komsu: RefCell::new(Some(Rc::clone(&x))) });
    *x.komsu.borrow_mut() = Some(Rc::clone(&y)); // x → y → x: döngü
    println!("x sayaç {}, y sayaç {}", Rc::strong_count(&x), Rc::strong_count(&y));
} // x ve y kapsamdan çıkar; ama sayaçlar 1'de kalır, iki düğüm hiç iade edilmez
