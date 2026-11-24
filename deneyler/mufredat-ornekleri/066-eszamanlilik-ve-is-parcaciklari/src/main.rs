use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

const UST: u64 = 3_000_000; // [0, UST) aralığındaki asal sayılar sayılır

// Deneme bölmesi: n'yi 2'den √n'ye kadar her sayıya böl.
// Büyük sayılar daha çok bölme ister: iş yükü aralık boyunca eşit değildir.
fn asal_mi(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

#[derive(Clone, Copy)]
enum Dagitim {
    Blok,     // i. iş parçacığı ardışık bir dilim alır
    Dongusel, // i. iş parçacığı i, i+k, i+2k, ... sayılarını alır
    Dinamik,  // boşalan iş parçacığı ortak sayaçtan sıradaki 10 000'lik parçayı alır
}

const PARCA: u64 = 10_000;

// k iş parçacığıyla say. Dönüş: (asal sayısı, toplam süre ms, her iş parçacığının kendi süresi ms)
fn say(k: u64, dagitim: Dagitim) -> (usize, f64, Vec<f64>) {
    let t = Instant::now();
    let siradaki = AtomicU64::new(0); // dinamik dağıtımda sıradaki parçanın başı
    let sonuclar: Vec<(usize, f64)> = thread::scope(|s| {
        let parcalar: Vec<_> = (0..k)
            .map(|i| {
                let siradaki = &siradaki;
                s.spawn(move || {
                    let t0 = Instant::now();
                    let adet = match dagitim {
                        Dagitim::Blok => (UST * i / k..UST * (i + 1) / k).filter(|&n| asal_mi(n)).count(),
                        Dagitim::Dongusel => (i..UST).step_by(k as usize).filter(|&n| asal_mi(n)).count(),
                        Dagitim::Dinamik => {
                            let mut adet = 0;
                            loop {
                                let bas = siradaki.fetch_add(PARCA, Ordering::Relaxed);
                                if bas >= UST {
                                    break adet;
                                }
                                adet += (bas..(bas + PARCA).min(UST)).filter(|&n| asal_mi(n)).count();
                            }
                        }
                    };
                    (adet, t0.elapsed().as_secs_f64() * 1e3)
                })
            })
            .collect();
        parcalar.into_iter().map(|p| p.join().unwrap()).collect()
    });
    let toplam = sonuclar.iter().map(|s| s.0).sum();
    (toplam, t.elapsed().as_secs_f64() * 1e3, sonuclar.iter().map(|s| s.1).collect())
}

fn sira_deneyi() {
    // 4 iş parçacığını 1000 kez başlat; her seferinde kilide hangi sırayla girdiklerini kaydet
    let mut siralar: BTreeMap<String, u32> = BTreeMap::new();
    for _ in 0..1000 {
        let sira = Mutex::new(String::new());
        thread::scope(|s| {
            for i in 0..4 {
                let sira = &sira;
                s.spawn(move || sira.lock().unwrap().push_str(&i.to_string()));
            }
        });
        *siralar.entry(sira.into_inner().unwrap()).or_insert(0) += 1;
    }
    let mut liste: Vec<_> = siralar.into_iter().collect();
    liste.sort_by(|a, b| b.1.cmp(&a.1));
    println!("1000 denemede {} farklı sıra görüldü; en sık: {:?}", liste.len(), &liste[..liste.len().min(4)]);
}

fn hizlanma_deneyi() {
    println!("available_parallelism = {}", thread::available_parallelism().unwrap());
    for (ad, d) in [("blok", Dagitim::Blok), ("döngüsel", Dagitim::Dongusel), ("dinamik", Dagitim::Dinamik)] {
        let (_, _, sureler) = say(4, d);
        let yazi: Vec<String> = sureler.iter().map(|s| format!("{s:>3.0}")).collect();
        println!("4 iş parçacığının kendi süreleri (ms), {ad:<8}: {}", yazi.join(" "));
    }
    let (asal, tek, _) = say(1, Dagitim::Blok);
    println!("1 iş parçacığı: {asal} asal, {tek:.1} ms");
    // Denenecek iş parçacığı sayıları makineden türetilir: 2, 4, n/2 ve n (n = mantıksal işlemci sayısı)
    let n = thread::available_parallelism().unwrap().get() as u64;
    let mut sayilar = vec![2, 4, n / 2, n];
    sayilar.retain(|&k| k >= 2);
    sayilar.sort();
    sayilar.dedup();
    for k in sayilar {
        let olc = |d| {
            let (a, ms, _) = say(k, d);
            assert_eq!(a, asal);
            format!("{ms:>5.1} ms ({:>4.2}×)", tek / ms)
        };
        let (b, c, d) = (olc(Dagitim::Blok), olc(Dagitim::Dongusel), olc(Dagitim::Dinamik));
        println!("{k:>2} iş parçacığı | blok {b} | döngüsel {c} | dinamik {d}");
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("sira") => sira_deneyi(),
        Some("hizlanma") => hizlanma_deneyi(),
        _ => eprintln!("kullanım: is_parcacigi sira | hizlanma"),
    }
}
