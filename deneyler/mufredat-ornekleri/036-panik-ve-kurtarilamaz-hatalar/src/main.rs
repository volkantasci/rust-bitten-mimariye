fn bol(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("sıfıra bölme: {a} / {b}");
    }
    a / b
}

fn main() {
    let b: i32 = std::env::args().nth(1).map_or(0, |s| s.parse().unwrap());
    println!("10 / {b} = {}", bol(10, b));
}
