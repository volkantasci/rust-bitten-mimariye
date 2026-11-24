use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn temel() {
    // 1) Boru: alt sürecin stdin'ine yaz, stdout'unu oku
    let mut sirala = Command::new("sort").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();
    sirala.stdin.take().unwrap().write_all("zeytin\nelma\nkiraz\n".as_bytes()).unwrap(); // yazma ucu düşer: sort dosya sonunu görür
    let cikti = sirala.wait_with_output().unwrap();
    print!("sort çıktısı:\n{}", String::from_utf8_lossy(&cikti.stdout));

    // 2) Sinyaller: alt sürece SIGTERM ve SIGKILL
    let mut uyuyan = Command::new("sleep").arg("10").spawn().unwrap();
    thread::sleep(Duration::from_millis(100));
    Command::new("kill").arg("-TERM").arg(uyuyan.id().to_string()).status().unwrap();
    println!("SIGTERM sonrası: {}", uyuyan.wait().unwrap());
    let mut ikinci = Command::new("sleep").arg("10").spawn().unwrap();
    ikinci.kill().unwrap(); // Child::kill → SIGKILL
    println!("kill() sonrası  : {}", ikinci.wait().unwrap());
}

fn kapasite() {
    // Çekirdeğin boru tamponu sınırlıdır: okuyan okumazsa yazan bir noktada bekler
    let (mut okuyucu, mut yazici) = io::pipe().unwrap();
    let yazilan = Arc::new(AtomicUsize::new(0));
    let sayac = Arc::clone(&yazilan);
    let yazan = thread::spawn(move || {
        loop {
            match yazici.write(b"x") {
                Ok(_) => sayac.fetch_add(1, Ordering::Relaxed),
                Err(e) => return e, // okuma ucu kapanınca
            };
        }
    });
    thread::sleep(Duration::from_millis(300));
    println!("kimse okumazken yazılan: {} bayt (yazan şimdi bekliyor)", yazilan.load(Ordering::Relaxed));
    let mut tampon = [0u8; 4096];
    okuyucu.read_exact(&mut tampon).unwrap();
    thread::sleep(Duration::from_millis(100));
    println!("4096 bayt okununca     : {} bayt", yazilan.load(Ordering::Relaxed));
    drop(okuyucu); // okuma ucunu kapat
    let hata = yazan.join().unwrap();
    println!("okuma ucu kapanınca yazma: {hata} ({:?})", hata.kind());
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("cok-yaz") => {
            for i in 0..100_000 {
                println!("satır {i}");
            }
        }
        Some("dikkatli-yaz") => {
            let mut cikis = io::stdout().lock();
            for i in 0..100_000 {
                if let Err(e) = writeln!(cikis, "satır {i}") {
                    if e.kind() == io::ErrorKind::BrokenPipe {
                        return; // okuyan gitti: sessizce bitir
                    }
                    panic!("{e}");
                }
            }
        }
        Some("kapasite") => kapasite(),
        _ => temel(),
    }
}
