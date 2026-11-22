fn main() {
    let b: bool = unsafe { std::mem::transmute(2u8) }; // bool yalnızca 0 ya da 1 olabilir
    if b { println!("doğru") } else { println!("yanlış") }
}
