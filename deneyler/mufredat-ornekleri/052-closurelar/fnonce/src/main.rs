fn main() {
    let ad = String::from("Deniz");
    let tuket = move || ad; // çağrılınca ad'ı dışarı verir: FnOnce
    println!("{}", tuket());
    println!("{}", tuket());
}
