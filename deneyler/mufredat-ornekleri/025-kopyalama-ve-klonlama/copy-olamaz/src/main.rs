#[derive(Clone, Copy)]
struct Kisi {
    ad: String,
    yas: u8,
}

fn main() {
    let k = Kisi { ad: String::from("Ada"), yas: 36 };
    let _ = (k.ad, k.yas);
}
