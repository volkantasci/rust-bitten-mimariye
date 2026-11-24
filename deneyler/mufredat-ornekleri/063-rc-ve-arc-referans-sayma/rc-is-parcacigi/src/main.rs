use std::rc::Rc;
use std::thread;

fn main() {
    let ortak = Rc::new(vec![1, 2, 3]);
    let kopya = Rc::clone(&ortak);
    // Rc'nin sayacı atomik değil: iki iş parçacığı aynı anda artırıp azaltırsa sayaç bozulabilir
    let t = thread::spawn(move || kopya.len());
    println!("{}", t.join().unwrap());
}
