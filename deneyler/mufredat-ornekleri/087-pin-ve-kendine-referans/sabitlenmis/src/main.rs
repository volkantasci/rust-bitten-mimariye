use std::marker::PhantomPinned;
use std::pin::Pin;

struct Sabit {
    _isaret: PhantomPinned, // "beni taşıma" işareti: tip Unpin olmaz
}

fn main() {
    let mut s = Sabit { _isaret: PhantomPinned };
    let _p = Pin::new(&mut s); // Pin::new yalnızca Unpin tipler için güvenli
}
