use std::collections::BTreeMap;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

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

fn main() {
    // 1) Sıra garantisi yok: 4 iş parçacığını 1000 kez başlat, bitiş sıralarını say
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

    // 2) Aynı işi k parçaya böl: hızlanma ölçümü
    let ust = 3_000_000u64;
    let mut tek = 0.0;
    for k in [1u64, 2, 4, 6, 12] {
        let t = Instant::now();
        let toplam: usize = thread::scope(|s| {
            let parcalar: Vec<_> = (0..k)
                .map(|i| {
                    let (bas, son) = (ust * i / k, ust * (i + 1) / k);
                    s.spawn(move || (bas..son).filter(|&n| asal_mi(n)).count())
                })
                .collect();
            parcalar.into_iter().map(|p| p.join().unwrap()).sum()
        });
        let sure = t.elapsed().as_secs_f64();
        if k == 1 {
            tek = sure;
        }
        println!("{k:>2} iş parçacığı: {toplam} asal, {:>7.1} ms, hızlanma {:>4.2}×", sure * 1e3, tek / sure);
    }
}
