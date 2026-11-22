struct KendineBakan {
    veri: [u8; 4],
    isaretci: *const u8, // kendi 'veri' alanını göstermesi amaçlanıyor
}

fn main() {
    let mut a = KendineBakan { veri: *b"abcd", isaretci: std::ptr::null() };
    a.isaretci = a.veri.as_ptr();
    println!("a.veri @ {:p}   a.isaretci = {:p}", a.veri.as_ptr(), a.isaretci);
    let b = a; // taşıma: bitler yeni bir yere kopyalandı
    println!("b.veri @ {:p}   b.isaretci = {:p}  ← hâlâ eski yeri gösteriyor", b.veri.as_ptr(), b.isaretci);
}
