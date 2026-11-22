fn main() {
    let mut v = vec![1, 2, 3];
    println!("kapasite {}, veri @ {:p}", v.capacity(), v.as_ptr());
    v.push(4); // kapasite doldu: ayırıcı bloğu büyütür
    println!("kapasite {}, veri @ {:p}", v.capacity(), v.as_ptr());

    let _komsu = Box::new(0u64); // hemen arkasına başka bir ayırma
    v.extend([5, 6, 7]); // yine doldu: bu kez yerinde büyüyemez
    println!("kapasite {}, veri @ {:p}", v.capacity(), v.as_ptr());
}
