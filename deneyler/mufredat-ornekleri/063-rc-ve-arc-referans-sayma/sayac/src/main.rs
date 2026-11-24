use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    let a = Rc::new(String::from("paylaşılan metin"));
    println!("güçlü sayaç: {}", Rc::strong_count(&a));
    let b = Rc::clone(&a); // veri kopyalanmaz, sayaç artar
    {
        let _c = Rc::clone(&a);
        println!("güçlü sayaç: {} (a, b, c aynı adrese bakıyor: {})", Rc::strong_count(&a), Rc::ptr_eq(&a, &b));
    }
    println!("güçlü sayaç: {} (c kapsam dışında)", Rc::strong_count(&a));

    let zayif = Rc::downgrade(&a); // sahiplik vermeyen gözlemci
    println!("zayıf sayaç: {}, upgrade: {:?}", Rc::weak_count(&a), zayif.upgrade().map(|s| s.len()));
    drop(a);
    drop(b); // son güçlü sahip gitti: veri bırakıldı
    println!("upgrade (sahipler gittikten sonra): {:?}", zayif.upgrade());

    // İş parçacıkları arasında: Arc (atomik sayaç)
    let ortak = Arc::new(vec![1, 2, 3]);
    let tutamaklar: Vec<_> = (0..3)
        .map(|i| {
            let kopya = Arc::clone(&ortak);
            thread::spawn(move || kopya[i] * 10)
        })
        .collect();
    let sonuc: Vec<i32> = tutamaklar.into_iter().map(|t| t.join().unwrap()).collect();
    println!("Arc ile paylaşılan vektörden: {sonuc:?}, sayaç şimdi {}", Arc::strong_count(&ortak));
}
