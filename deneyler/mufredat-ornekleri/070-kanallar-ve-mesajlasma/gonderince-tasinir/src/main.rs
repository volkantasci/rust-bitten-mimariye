use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<Vec<u32>>();
    let alici = thread::spawn(move || rx.recv().unwrap().len());
    let olcumler = vec![1, 2, 3];
    tx.send(olcumler).unwrap(); // değer kanala taşındı: sahibi artık alıcı
    println!("{olcumler:?}"); // gönderdikten sonra kullanmaya çalışmak
    println!("{}", alici.join().unwrap());
}
