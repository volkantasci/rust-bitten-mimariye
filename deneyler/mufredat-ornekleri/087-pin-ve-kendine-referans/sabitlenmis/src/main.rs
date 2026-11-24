use std::marker::PhantomPinned;
use std::pin::Pin;

struct Sabit {
    _isaret: PhantomPinned, // "beni taşıma" işareti: tip Unpin olmaz
}

fn main() {
    let mut s = Sabit { _isaret: PhantomPinned };
    let _p = Pin::new(&mut s); // Pin::new yalnızca Unpin tipler için güvenli

    // Derleyicinin ürettiği future da kendine referans içerebilir
    let mut gelecek = async {
        let x = 5;
        let r = &x; // await boyunca yaşayan, kendi yerel değişkenine referans
        std::future::ready(()).await;
        *r
    };
    let _q = Pin::new(&mut gelecek);
}
