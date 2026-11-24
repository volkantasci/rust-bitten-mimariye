// unsafe Rust: aynı bloğu iki kez iade etmek (tanımsız davranış).
fn main() {
    let kutu = Box::new(42u64);
    let p = Box::into_raw(kutu); // sahiplikten çık, yalnızca adresi tut
    unsafe {
        drop(Box::from_raw(p)); // birinci iade
        println!("birinci iade tamam");
        drop(Box::from_raw(p)); // ikinci iade: aynı blok
    }
    println!("buraya gelinmez");
}
