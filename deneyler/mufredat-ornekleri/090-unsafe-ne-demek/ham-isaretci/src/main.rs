fn main() {
    let x = 42;
    let p = &x as *const i32; // ham işaretçi oluşturmak güvenli
    println!("{}", *p); // içeriğine erişmek (dereference) ayrı iş
}
