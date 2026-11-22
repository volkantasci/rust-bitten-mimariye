enum Liste {
    Bos,
    Eleman(i32, Liste),
}

fn main() {
    let _l = Liste::Eleman(1, Liste::Eleman(2, Liste::Bos));
}
