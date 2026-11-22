fn main() {
    let ad = String::from("Ferris");
    let tuket = move || ad; // çağrılınca ad'ı dışarı verir: FnOnce
    println!("{}", tuket());
    println!("{}", tuket());
}
