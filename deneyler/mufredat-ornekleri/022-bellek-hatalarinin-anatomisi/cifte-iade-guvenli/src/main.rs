// Güvenli Rust: değerin tek sahibi var; iade edilen değer bir daha kullanılamaz.
fn main() {
    let kutu = Box::new(42u64);
    drop(kutu); // birinci iade: sahiplik drop'a geçti
    drop(kutu); // ikinci iade denemesi
}
