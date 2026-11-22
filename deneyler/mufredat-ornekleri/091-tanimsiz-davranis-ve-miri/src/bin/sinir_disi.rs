fn main() {
    let v = vec![10u8, 20, 30];
    let p = v.as_ptr();
    let disari = unsafe { *p.add(3) }; // 4. eleman yok: tanımsız davranış
    println!("v[3] = {disari}");
}
