use std::mem::size_of_val;

fn main() {
    let hic = || 1;
    let x = 10_u64;
    let oku = || x + 1; // x'i paylaşılan referansla yakalar
    let mut sayac = 0_u32;
    let mut artir = || {
        sayac += 1; // sayac'ı değiştirilebilir referansla yakalar
        sayac
    };
    let ad = String::from("Ferris");
    let tuket = move || ad; // ad'ı içine taşır

    println!("hiçbir şey yakalamayan : {:>2} bayt", size_of_val(&hic));
    println!("&u64 yakalayan         : {:>2} bayt", size_of_val(&oku));
    println!("&mut u32 yakalayan     : {:>2} bayt", size_of_val(&artir));
    println!("String taşıyan (move)  : {:>2} bayt", size_of_val(&tuket));
    println!("{} {} {} {}", hic(), oku(), artir(), artir());
    println!("{}", tuket());
}
