static SELAM: &str = "merhaba";

fn main() {
    let yerel = 42u64; // yığında (stack)
    let kutu = Box::new(7u64); // öbekte (heap)
    let metin = String::from("öbekte bir metin");
    println!("kod    main()            : {:p}", main as fn());
    println!("statik SELAM metni       : {:p}", SELAM.as_ptr());
    println!("öbek   Box<u64>          : {:p}", kutu);
    println!("öbek   String verisi     : {:p}", metin.as_ptr());
    println!("yığın  yerel değişken    : {:p}", &yerel);
    // Çekirdeğin bu sürece verdiği bellek bölgeleri
    let haritalar = std::fs::read_to_string("/proc/self/maps").unwrap();
    for satir in haritalar.lines() {
        if satir.contains("yigin_obek") || satir.contains("[heap]") || satir.contains("[stack]") {
            println!("{satir}");
        }
    }
}
