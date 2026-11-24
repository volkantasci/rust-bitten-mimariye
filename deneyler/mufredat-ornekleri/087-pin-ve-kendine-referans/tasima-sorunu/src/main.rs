use std::marker::PhantomPinned;
use std::pin::Pin;

struct KendineBakan {
    veri: [u8; 4],
    isaretci: *const u8, // kendi 'veri' alanını göstermesi amaçlanıyor
}

fn olustur() -> KendineBakan {
    let mut k = KendineBakan { veri: *b"abcd", isaretci: std::ptr::null() };
    k.isaretci = k.veri.as_ptr();
    k // dönüşte taşınır: bu fonksiyonun yığın çerçevesi ve k'nin oradaki yeri yok olur
}

struct Sabit {
    veri: [u8; 4],
    isaretci: *const u8,
    _sabit: PhantomPinned, // "beni taşıma" işareti: tip Unpin olmaz
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("oku") {
        let b = olustur();
        let ilk = unsafe { *b.isaretci }; // eski (artık geçersiz) yeri okumak
        println!("ilk bayt: {}", ilk as char);
        return;
    }
    // 1) Taşıma bir bit kopyasıdır: işaretçinin kendisi de kopyalanır, güncellenmez
    let mut a = KendineBakan { veri: *b"abcd", isaretci: std::ptr::null() };
    a.isaretci = a.veri.as_ptr();
    println!("a.veri @ {:p}   a.isaretci = {:p}", a.veri.as_ptr(), a.isaretci);
    let b = a; // taşıma: bitler yeni bir yere kopyalandı
    println!("b.veri @ {:p}   b.isaretci = {:p}  ← hâlâ eski yeri gösteriyor", b.veri.as_ptr(), b.isaretci);

    // 2) Box::pin: değer öbekte durur; Pin<Box<_>> taşınsa da değerin adresi değişmez
    let mut kutu = Box::new(Sabit { veri: *b"wxyz", isaretci: std::ptr::null(), _sabit: PhantomPinned });
    kutu.isaretci = kutu.veri.as_ptr();
    let s: Pin<Box<Sabit>> = Box::into_pin(kutu);
    println!("sabitlenmiş değer @ {:p}, isaretci = {:p}", s.veri.as_ptr(), s.isaretci);
    let t = s; // kutunun kendisini (öbeği gösteren işaretçiyi) taşı
    println!("kutu taşındıktan sonra @ {:p}, isaretci = {:p}  ← aynı yer", t.veri.as_ptr(), t.isaretci);
}
