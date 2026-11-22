fn main() {
    for s in ["A", "ç", "ğ", "€", "🦀"] {
        let c = s.chars().next().unwrap();
        println!("{s}  U+{:04X}  {} bayt  {:02x?}", c as u32, s.len(), s.as_bytes());
    }
    let kelime = "Iğdır";
    println!("{kelime:?}: len() = {} bayt, chars().count() = {} karakter",
             kelime.len(), kelime.chars().count());
    println!("\"istanbul\".to_uppercase() = {:?}", "istanbul".to_uppercase());
    let kucuk = "İ".to_lowercase();
    println!("\"İ\".to_lowercase()        = {:?} ({} karakter: {:?})",
             kucuk, kucuk.chars().count(), kucuk.chars().map(|c| format!("U+{:04X}", c as u32)).collect::<Vec<_>>());
    println!("size_of::<char>()          = {} bayt", std::mem::size_of::<char>());
}
