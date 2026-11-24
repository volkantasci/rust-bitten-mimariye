use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const ADIM: u64 = 1_000_000;

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}

// Mutex ile paylaşılan sayaç: doğru ama her artış kilidi almayı gerektirir
fn mutex_sayac(is: u64) -> (u64, f64) {
    let t = Instant::now();
    let sayac = Arc::new(Mutex::new(0u64));
    let tutamaklar: Vec<_> = (0..is)
        .map(|_| {
            let sayac = Arc::clone(&sayac);
            thread::spawn(move || {
                for _ in 0..ADIM {
                    *sayac.lock().unwrap() += 1; // kilit al, artır; koruyucu (guard) düşünce kilit bırakılır
                }
            })
        })
        .collect();
    for h in tutamaklar {
        h.join().unwrap();
    }
    let son = *sayac.lock().unwrap();
    (son, ms(t))
}

fn sayaclar() {
    let (m, mt) = mutex_sayac(8);
    println!("Mutex           : {m} ({mt:>6.1} ms)");

    let t = Instant::now();
    let atom = AtomicU64::new(0);
    thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                for _ in 0..ADIM {
                    atom.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    println!("AtomicU64       : {} ({:>6.1} ms)", atom.load(Ordering::Relaxed), ms(t));

    // Paylaşmamak: her iş parçacığı kendi sayacını tutar, sonda toplanır
    let t = Instant::now();
    let toplam: u64 = thread::scope(|s| {
        let h: Vec<_> = (0..8)
            .map(|_| {
                s.spawn(|| {
                    let mut yerel = 0u64;
                    for _ in 0..ADIM {
                        yerel = std::hint::black_box(yerel + 1);
                    }
                    yerel
                })
            })
            .collect();
        h.into_iter().map(|x| x.join().unwrap()).sum()
    });
    println!("yerel + toplama : {toplam} ({:>6.1} ms)", ms(t));
}

// Kilitte bekleme: 8 iş parçacığı, her biri 20 kez `uyku` ms süren bir iş yapar (G/Ç benzetimi)
fn kilitte_bekleme(uyku: u64) {
    let kilit = Mutex::new(0u64);
    for icerde in [true, false] {
        let t = Instant::now();
        thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..20 {
                        if icerde {
                            let mut g = kilit.lock().unwrap();
                            thread::sleep(Duration::from_millis(uyku)); // kilit elde tutulurken "G/Ç"
                            *g += 1;
                        } else {
                            thread::sleep(Duration::from_millis(uyku)); // "G/Ç" kilidin dışında
                            *kilit.lock().unwrap() += 1;
                        }
                    }
                });
            }
        });
        let ad = if icerde { "iş kilidin içinde " } else { "iş kilidin dışında" };
        println!("{ad}: 8 × 20 iş, iş başına {uyku} ms, {:>6.1} ms", ms(t));
    }
}

// Zehirlenme: kilidi tutarken panikleyen iş parçacığı
fn zehir() {
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

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();
    match arg.as_str() {
        "sayaclar" => sayaclar(),
        "kilitte-bekleme" => kilitte_bekleme(std::env::args().nth(2).map_or(1, |s| s.parse().unwrap())),
        "zehir" => zehir(),
        // "mutex N": yalnız Mutex'li sayaç, N iş parçacığıyla (futex çağrılarını saymak için)
        "mutex" => {
            let n = std::env::args().nth(2).map_or(8, |s| s.parse().unwrap());
            let (m, _) = mutex_sayac(n);
            println!("Mutex, {n} iş parçacığı: sayaç = {m}");
        }
        _ => eprintln!("kullanım: kilitler sayaclar | mutex N | kilitte-bekleme [ms] | zehir"),
    }
}
