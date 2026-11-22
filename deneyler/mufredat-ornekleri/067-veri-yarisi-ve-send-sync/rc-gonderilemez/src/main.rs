use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

fn main() {
    let sayac = Rc::new(RefCell::new(0));
    let kopya = Rc::clone(&sayac);
    let t = thread::spawn(move || {
        *kopya.borrow_mut() += 1;
    });
    t.join().unwrap();
    println!("{}", sayac.borrow());
}
