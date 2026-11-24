use std::mem::size_of;

fn main() {
    println!("size_of::<Vec<u64>>() = {} bayt: işaretçi + kapasite + uzunluk", size_of::<Vec<u64>>());
    let mut v: Vec<u64> = Vec::new();
    println!("başlangıç: len {}, cap {}", v.len(), v.capacity());
    let mut onceki_cap = v.capacity();
    let mut onceki_adres = v.as_ptr();
    let mut ust_sinir = 0; // yeniden ayırmada taşınması gerekebilecek toplam eleman (üst sınır)
    let mut gercek = 0; // blok gerçekten yeni bir adrese taşındığında kopyalanan eleman
    for i in 0..1000u64 {
        let dolu = v.len();
        if dolu == v.capacity() {
            ust_sinir += dolu;
        }
        v.push(i);
        if v.capacity() != onceki_cap {
            let durum = if onceki_cap == 0 {
                "ilk ayırma"
            } else if v.as_ptr() != onceki_adres {
                gercek += dolu; // ayırıcı yeni bir yer verdi: eski elemanlar kopyalandı
                "taşındı"
            } else {
                "yerinde büyüdü" // arkasındaki alan boştu: kopya yok
            };
            println!("len {:>4} → cap {:>4}  ({durum})", v.len(), v.capacity());
            onceki_cap = v.capacity();
            onceki_adres = v.as_ptr();
        }
    }
    println!("1000 push: üst sınır {ust_sinir} eleman (≈ push başına {:.2}), bu çalıştırmada gerçekte taşınan {gercek}",
             ust_sinir as f64 / 1000.0);
    let hazir: Vec<u64> = Vec::with_capacity(1000);
    println!("Vec::with_capacity(1000): cap {}", hazir.capacity());
    // İlk ayırmadaki en küçük kapasite eleman boyutuna bağlıdır (std: min_non_zero_cap)
    let mut a: Vec<u8> = Vec::new();
    a.push(1);
    let mut b: Vec<u64> = Vec::new();
    b.push(1);
    let mut c: Vec<[u8; 2048]> = Vec::new();
    c.push([0; 2048]);
    println!("ilk push sonrası cap: u8 → {}, u64 → {}, [u8; 2048] → {}", a.capacity(), b.capacity(), c.capacity());
}
