use std::mem::size_of;

fn main() {
    println!("&u8    : {:>2} bayt", size_of::<&u8>());
    println!("&[u8]  : {:>2} bayt (işaretçi + uzunluk)", size_of::<&[u8]>());
    println!("&str   : {:>2} bayt (işaretçi + uzunluk)", size_of::<&str>());
    println!("String : {:>2} bayt (işaretçi + kapasite + uzunluk)", size_of::<String>());

    let s = String::from("ağaç");
    println!("{s:?} baytları: {:02x?}", s.as_bytes());
    println!("&s[0..1] = {:?}", &s[0..1]);
    println!("&s[1..3] = {:?}", &s[1..3]);
    let n = std::env::args().count() + 1; // çalışma anında 2
    println!("&s[0..{n}] = {:?}", &s[0..n]); // 'ğ' nin ortası
}
