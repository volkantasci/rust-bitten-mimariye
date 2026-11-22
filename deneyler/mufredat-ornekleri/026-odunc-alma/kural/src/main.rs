fn main() {
    let mut v = vec![1, 2, 3];
    let ilk = &v[0]; // paylaşılan ödünç
    v.push(4); // değiştirmek için özel (mut) ödünç gerekir
    println!("ilk = {ilk}");
}
