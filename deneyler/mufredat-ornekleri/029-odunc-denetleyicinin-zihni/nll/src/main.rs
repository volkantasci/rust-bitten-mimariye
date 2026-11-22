fn main() {
    let mut v = vec![1, 2, 3];
    let ilk = &v[0];
    println!("ilk = {ilk}"); // ödüncün son kullanımı burası
    v.push(4); // NLL: ödünç bittiği için kabul edilir
    println!("v = {v:?}");
}
