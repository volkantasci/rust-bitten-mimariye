use rustix::pipe::{fcntl_getpipe_size, fcntl_setpipe_size};
use std::fs;
use std::io::{self, PipeReader, PipeWriter, Read, Write};
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

/// Kullanıcı başına boru sayfası yumuşak sınırı (pipe(7)); 0 ise sınır yok.
fn yumusak_sinir() -> usize {
    let sinir = fs::read_to_string("/proc/sys/fs/pipe-user-pages-soft").unwrap().trim().parse().unwrap();
    println!("yumuşak sınır (pipe-user-pages-soft): {sinir} sayfa");
    sinir
}

/// Kimse okumazken boruya bayt bayt yazar: tampon dolunca yazan bekler.
fn olc(mut okuyucu: PipeReader, mut yazici: PipeWriter) {
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

fn kapasite() {
    // Çekirdeğin boru tamponu sınırlıdır; büyüklüğü her makinede aynı olmayabilir,
    // bu yüzden önce çekirdeğe sorarız (fcntl F_GETPIPE_SZ)
    yumusak_sinir();
    let (okuyucu, yazici) = io::pipe().unwrap();
    println!("borunun kapasitesi (F_GETPIPE_SZ): {} bayt", fcntl_getpipe_size(&yazici).unwrap());
    olc(okuyucu, yazici);
}

fn sinirda() {
    // Bu kullanıcının borularındaki sayfaları yumuşak sınıra kadar doldururuz: her yeni boruyu
    // 1 MiB'a (256 sayfa) büyütürüz; çekirdek yeni bir boruya varsayılan 16 sayfadan
    // (65 536 bayt) azını verince dururuz
    const VARSAYILAN: usize = 16 * 4096;
    if yumusak_sinir() == 0 {
        return; // sınır kapalı
    }
    let mut tutulan = Vec::new();
    let (okuyucu, yazici) = loop {
        let (r, w) = io::pipe().unwrap();
        if fcntl_getpipe_size(&w).unwrap() < VARSAYILAN {
            break (r, w); // çekirdek bu boruya varsayılandan az verdi
        }
        let _ = fcntl_setpipe_size(&w, 1 << 20); // sınır aşılacaksa EPERM: boru olduğu gibi kalır
        tutulan.push((r, w));
    };
    let sayfa: usize = tutulan.iter().map(|(_, w)| fcntl_getpipe_size(w).unwrap() / 4096).sum();
    println!("tutulan borular: {} tane, toplam {sayfa} sayfa", tutulan.len());
    println!("yeni borunun kapasitesi (F_GETPIPE_SZ): {} bayt", fcntl_getpipe_size(&yazici).unwrap());
    // Varsayılanı geri istemek (fcntl F_SETPIPE_SZ)
    match fcntl_setpipe_size(&yazici, VARSAYILAN) {
        Ok(yeni) => println!("{VARSAYILAN} bayt isteği (F_SETPIPE_SZ): kabul edildi, {yeni} bayt"),
        Err(hata) => println!("{VARSAYILAN} bayt isteği (F_SETPIPE_SZ): {hata}"),
    }
    olc(okuyucu, yazici);
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
        Some("sinirda") => sinirda(),
        _ => temel(),
    }
}
