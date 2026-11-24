// Elle yönetim: yer istemek ve iade etmek programcının işi (std::alloc, unsafe).
use std::alloc::{alloc, dealloc, Layout};

fn main() {
    let duzen = Layout::array::<u64>(4).unwrap(); // 4 × 8 = 32 bayt, 8 bayt hizalı
    println!("istenen: {} bayt, hizalama {}", duzen.size(), duzen.align());
    unsafe {
        let p = alloc(duzen) as *mut u64; // ayırıcıdan yer iste
        assert!(!p.is_null());
        for i in 0..4 {
            p.add(i).write(i as u64 * 10);
        }
        println!("blok {:p}: {:?}", p, std::slice::from_raw_parts(p, 4));
        dealloc(p as *mut u8, duzen); // iade: aynı düzenle geri ver
    }
    unsafe {
        let q = alloc(duzen); // ikinci blok: iade etmeyi "unutuyoruz"
        assert!(!q.is_null());
        println!("ikinci blok {:p} ayrıldı", q);
    }
    println!("main bitti; ikinci blok hiç iade edilmedi");
}
