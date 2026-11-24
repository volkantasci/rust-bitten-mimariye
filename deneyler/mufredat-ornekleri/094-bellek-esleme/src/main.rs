use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::fd::AsRawFd;

const YOL: &str = "/tmp/esleme.bin";
const BOYUT: usize = 64 * 1024 * 1024; // 64 MiB
const SAYFA: usize = 4096;

/// (hafif, ağır) sayfa hatası sayaçları: çekirdeğin bu süreç için tuttuğu sayılar.
fn hatalar() -> (i64, i64) {
    // SAFETY: getrusage'a geçerli, sıfırlanmış bir rusage yapısının adresini veriyoruz.
    unsafe {
        let mut r: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut r);
        (r.ru_minflt, r.ru_majflt)
    }
}

/// Bu sürecin depolama aygıtından okuttuğu bayt sayısı (/proc/self/io, read_bytes).
fn diskten_okunan() -> u64 {
    let io = std::fs::read_to_string("/proc/self/io").unwrap();
    io.lines()
        .find_map(|satir| satir.strip_prefix("read_bytes: "))
        .and_then(|deger| deger.trim().parse().ok())
        .unwrap_or(0)
}

/// /proc/self/smaps'te `adres`te başlayan eşlemenin `alan` satırı (ör. "Rss", "FilePmdMapped").
fn smaps_alani(adres: usize, alan: &str) -> String {
    let smaps = std::fs::read_to_string("/proc/self/smaps").unwrap();
    let baslik = format!("{adres:x}-");
    let mut icinde = false;
    for satir in smaps.lines() {
        let ilk = satir.split_whitespace().next().unwrap_or("");
        let yeni_esleme = ilk.contains('-') && ilk.chars().all(|c| c.is_ascii_hexdigit() || c == '-');
        if yeni_esleme {
            icinde = satir.starts_with(&baslik);
        } else if icinde && satir.starts_with(alan) {
            return satir.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
        }
    }
    String::from("?")
}

/// Dosyayı baştan yazar: ilk 7 bayt "MERHABA", gerisi delik (seyrek dosya).
/// Kesme (truncate) eski sayfaları sayfa önbelleğinden de atar: her deneme soğuk başlar.
fn dosya_hazirla() -> File {
    let mut f = OpenOptions::new().create(true).write(true).truncate(true).open(YOL).unwrap();
    f.write_all(b"MERHABA").unwrap();
    f.set_len(BOYUT as u64).unwrap();
    File::open(YOL).unwrap()
}

/// Dosyayı eşler, her sayfanın ilk baytına bir kez dokunur, hata sayısını yazar.
fn dene(ad: &str, ek_bayrak: i32, tavsiye: Option<i32>) {
    let f = dosya_hazirla();
    let d0 = diskten_okunan(); // eşleme ve dokunuşlar boyunca diskten okunan (MAP_POPULATE'in okumaları dahil)
    // SAFETY: dosya açık ve BOYUT bayt uzunluğunda; yalnız okuma izniyle özel eşleme istiyoruz.
    let p = unsafe {
        libc::mmap(std::ptr::null_mut(), BOYUT, libc::PROT_READ, libc::MAP_PRIVATE | ek_bayrak, f.as_raw_fd(), 0)
    };
    assert_ne!(p, libc::MAP_FAILED);
    if let Some(t) = tavsiye {
        // SAFETY: p ve BOYUT az önce mmap'ten geldi.
        assert_eq!(unsafe { libc::madvise(p, BOYUT, t) }, 0);
    }
    // SAFETY: eşleme BOYUT bayt boyunca okunabilir ve dilim munmap'ten önce son kez kullanılıyor.
    // &[u8] içeriğin dilim yaşadıkça değişmemesini de ister: bu süre içinde dosyaya kimse yazmıyor
    // (MAP_PRIVATE'te dosyaya sonradan yapılan değişikliklerin eşlemede görünüp görünmeyeceği belirsizdir).
    let veri: &[u8] = unsafe { std::slice::from_raw_parts(p as *const u8, BOYUT) };
    let (h0, a0) = hatalar();
    let mut toplam = 0u64;
    for i in (0..BOYUT).step_by(SAYFA) {
        toplam += veri[i] as u64; // her sayfanın ilk baytı: sayfa eşli değilse sayfa hatası
    }
    let (h1, a1) = hatalar();
    let d1 = diskten_okunan();
    println!(
        "{ad:<13} {} sayfaya dokunuldu: {:>5} hafif, {} ağır sayfa hatası (ilk 7 bayt {:?}, toplam {toplam})",
        BOYUT / SAYFA,
        h1 - h0,
        a1 - a0,
        std::str::from_utf8(&veri[..7]).unwrap()
    );
    println!(
        "              smaps: Rss {}, FilePmdMapped {}; diskten okunan: {} bayt",
        smaps_alani(p as usize, "Rss:"),
        smaps_alani(p as usize, "FilePmdMapped:"),
        d1 - d0
    );
    // SAFETY: p ve BOYUT mmap'ten geldi; veri artık kullanılmıyor.
    unsafe { libc::munmap(p, BOYUT) };
}

fn main() {
    dene("varsayılan", 0, None); // çekirdek kendi seçimiyle (önden okuma dahil)
    dene("MADV_RANDOM", 0, Some(libc::MADV_RANDOM)); // "rastgele okuyacağım": önden okuma kapalı
    dene("MAP_POPULATE", libc::MAP_POPULATE, None); // bütün sayfalar mmap sırasında önceden eşlenir
}
