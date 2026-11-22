fn main() {
    let p = {
        let kutu = Box::new(5u32);
        &*kutu as *const u32
    }; // kutu burada bırakıldı
    println!("değer = {}", unsafe { *p }); // bırakılmış bellek: tanımsız davranış
}
