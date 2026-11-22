use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const IS: u64 = 8;
const ADIM: u64 = 1_000_000;

fn main() {
    // 1) Mutex ile paylaşılan sayaç: doğru ama çekişmeli
    let t = Instant::now();
    let sayac = Arc::new(Mutex::new(0u64));
    let tutamaklar: Vec<_> = (0..IS)
        .map(|_| {
            let sayac = Arc::clone(&sayac);
            thread::spawn(move || {
                for _ in 0..ADIM {
                    *sayac.lock().unwrap() += 1; // kilit al, artır, kilit (guard) düşünce bırak
                }
            })
        })
        .collect();
    for h in tutamaklar { h.join().unwrap(); }
    println!("Mutex           : {} ({:>6.1} ms)", *sayac.lock().unwrap(), t.elapsed().as_secs_f64() * 1e3);

    // 2) Atomik sayaç
    let t = Instant::now();
    let atom = AtomicU64::new(0);
    thread::scope(|s| {
        for _ in 0..IS {
            s.spawn(|| for _ in 0..ADIM { atom.fetch_add(1, Ordering::Relaxed); });
        }
    });
    println!("AtomicU64       : {} ({:>6.1} ms)", atom.load(Ordering::Relaxed), t.elapsed().as_secs_f64() * 1e3);

    // 3) Paylaşma! Her iş parçacığı kendi sayacını tutsun, sonunda topla
    let t = Instant::now();
    let toplam: u64 = thread::scope(|s| {
        let h: Vec<_> = (0..IS).map(|_| s.spawn(|| { let mut yerel = 0u64; for _ in 0..ADIM { yerel = std::hint::black_box(yerel + 1); } yerel })).collect();
        h.into_iter().map(|x| x.join().unwrap()).sum()
    });
    println!("yerel + toplama : {} ({:>6.1} ms)", toplam, t.elapsed().as_secs_f64() * 1e3);

    // 4) Zehirlenme: kilidi tutarken panikleyen iş parçacığı
    let veri = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v2 = Arc::clone(&veri);
    let _ = thread::spawn(move || {
        let mut kilitli = v2.lock().unwrap();
        kilitli.push(4);
        panic!("kilit elimdeyken çöktüm");
    })
    .join();
    match veri.lock() {
        Ok(v) => println!("kilit temiz: {v:?}"),
        Err(zehir) => println!("kilit zehirlenmiş! içerik yine de okunabilir: {:?}", zehir.into_inner()),
    }
}
