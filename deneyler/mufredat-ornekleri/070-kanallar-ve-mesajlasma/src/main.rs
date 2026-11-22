use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // Boru hattı: üretici → işçi → tüketici, paylaşılan bellek yok, yalnızca mesaj
    let (ham_tx, ham_rx) = mpsc::channel::<u32>();
    let (sonuc_tx, sonuc_rx) = mpsc::channel::<(u32, u32)>();
    let uretici = thread::spawn(move || {
        for n in 1..=5 {
            ham_tx.send(n).unwrap();
        }
        // ham_tx burada düşer: kanal kapanır
    });
    let isci = thread::spawn(move || {
        for n in ham_rx {
            sonuc_tx.send((n, n * n)).unwrap();
        }
    });
    for (n, kare) in sonuc_rx {
        println!("{n}² = {kare}");
    }
    uretici.join().unwrap();
    isci.join().unwrap();

    // Sınırlı kanal: dolunca gönderen bekler (geri basınç)
    let (tx, rx) = mpsc::sync_channel::<u32>(2);
    let t0 = Instant::now();
    let hizli = thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
            println!("[{:>4} ms] üretici {i} gönderdi", t0.elapsed().as_millis());
        }
    });
    thread::sleep(Duration::from_millis(300));
    for i in rx {
        println!("[{:>4} ms] yavaş tüketici {i} aldı", t0.elapsed().as_millis());
        thread::sleep(Duration::from_millis(100));
    }
    hizli.join().unwrap();
}
