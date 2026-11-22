use std::mem::size_of;
use std::num::NonZeroU32;

fn yas_bul(ad: &str) -> Option<u32> {
    if ad == "Ferris" { Some(7) } else { None }
}

fn main() {
    println!("&u64               : {} bayt", size_of::<&u64>());
    println!("Option<&u64>       : {} bayt", size_of::<Option<&u64>>());
    println!("Box<u64>           : {} bayt", size_of::<Box<u64>>());
    println!("Option<Box<u64>>   : {} bayt", size_of::<Option<Box<u64>>>());
    println!("u32                : {} bayt", size_of::<u32>());
    println!("Option<u32>        : {} bayt", size_of::<Option<u32>>());
    println!("Option<NonZeroU32> : {} bayt", size_of::<Option<NonZeroU32>>());
    println!("Option<bool>       : {} bayt", size_of::<Option<bool>>());
    for ad in ["Ferris", "Ali"] {
        match yas_bul(ad) {
            Some(yas) => println!("{ad}: {yas} yaşında"),
            None => println!("{ad}: bulunamadı"),
        }
    }
}
