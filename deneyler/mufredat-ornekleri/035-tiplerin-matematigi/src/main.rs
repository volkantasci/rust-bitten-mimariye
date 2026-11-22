use std::convert::Infallible;
use std::mem::size_of;

#[allow(dead_code)]
enum Hic {} // hiç değeri olmayan tip: 0

fn main() {
    println!("()                      {} bayt  1 değer", size_of::<()>());
    println!("bool                    {} bayt  2 değer", size_of::<bool>());
    println!("(bool, bool)            {} bayt  2 × 2 = 4 değer", size_of::<(bool, bool)>());
    println!("Option<bool>            {} bayt  2 + 1 = 3 değer", size_of::<Option<bool>>());
    println!("Result<bool, ()>        {} bayt  2 + 1 = 3 değer", size_of::<Result<bool, ()>>());
    println!("Hic                     {} bayt  0 değer", size_of::<Hic>());
    println!("Option<Hic>             {} bayt  0 + 1 = 1 değer", size_of::<Option<Hic>>());
    println!("Result<u32, Infallible> {} bayt  (u32 ile aynı: hata kolu boş)", size_of::<Result<u32, Infallible>>());
}
