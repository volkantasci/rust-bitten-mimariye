use std::time::{Duration, Instant};

fn bellek() -> (String, String) {
    let s = std::fs::read_to_string("/proc/self/status").unwrap();
    let al = |k: &str| s.lines().find(|l| l.starts_with(k)).unwrap().split_whitespace().nth(1).unwrap().to_string();
    (al("VmRSS:"), al("VmSize:"))
}

fn main() {
    let kip = std::env::args().nth(1).unwrap_or_default();
    let n: usize = std::env::args().nth(2).map_or(1000, |s| s.parse().unwrap());
    let t = Instant::now();
    if kip == "is-parcacigi" {
        let tutamaklar: Vec<_> = (0..n)
            .map(|_| std::thread::spawn(|| std::thread::sleep(Duration::from_secs(1))))
            .collect();
        let (rss, sanal) = bellek();
        println!("{n} iş parçacığı : başlatma {:>6.1} ms, RSS {rss} kB, sanal {sanal} kB", t.elapsed().as_secs_f64() * 1e3);
        for h in tutamaklar { h.join().unwrap(); }
    } else {
        let rt = tokio::runtime::Builder::new_multi_thread().enable_time().build().unwrap();
        rt.block_on(async {
            let gorevler: Vec<_> = (0..n)
                .map(|_| tokio::spawn(tokio::time::sleep(Duration::from_secs(1))))
                .collect();
            let (rss, sanal) = bellek();
            println!("{n} async görev   : başlatma {:>6.1} ms, RSS {rss} kB, sanal {sanal} kB", t.elapsed().as_secs_f64() * 1e3);
            for g in gorevler { g.await.unwrap(); }
        });
    }
    println!("    toplam süre {:.2} s", t.elapsed().as_secs_f64());
}
