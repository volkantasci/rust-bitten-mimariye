// unsafe Rust: push'tan önce alınan ham işaretçiyi push'tan sonra kullanmak.
fn main() {
    let mut v = vec![1, 2, 3];
    let ilk: *const i32 = &v[0]; // ham işaretçi: o anki bloğun adresi
    println!("önce : kapasite {}, veri @ {:p}", v.capacity(), v.as_ptr());
    v.push(4); // kapasite doldu: ayırıcı bloğu büyütür ya da taşır
    println!("sonra: kapasite {}, veri @ {:p}", v.capacity(), v.as_ptr());
    let deger = unsafe { *ilk }; // blok taşındıysa iade edilmiş belleği okur: tanımsız davranış
    println!("ilk = {deger}");
}
