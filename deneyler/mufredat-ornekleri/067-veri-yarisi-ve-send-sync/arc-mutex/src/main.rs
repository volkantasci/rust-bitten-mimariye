// rc-gonderilemez/ deneyinin düzeltilmiş hâli: Rc → Arc (atomik sayaç), RefCell → Mutex (kilit)
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let sayac = Arc::new(Mutex::new(0));
    let kopya = Arc::clone(&sayac);
    let t = thread::spawn(move || {
        *kopya.lock().unwrap() += 1; // borrow_mut() yerine lock()
    });
    t.join().unwrap();
    println!("{}", sayac.lock().unwrap()); // borrow() yerine lock()
}
