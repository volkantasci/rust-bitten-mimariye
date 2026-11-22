use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::fd::AsRawFd;

fn kucuk_hata() -> i64 {
    // SAFETY: getrusage'a geçerli bir rusage yapısı veriyoruz.
    unsafe {
        let mut r: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut r);
        r.ru_minflt
    }
}

fn main() {
    let yol = "/tmp/esleme.bin";
    let boyut: usize = 64 * 1024 * 1024; // 64 MiB
    {
        let mut f = OpenOptions::new().create(true).write(true).truncate(true).open(yol).unwrap();
        f.write_all(b"MERHABA").unwrap();
        f.set_len(boyut as u64).unwrap(); // geri kalanı sıfır (seyrek dosya)
    }
    let f = File::open(yol).unwrap();
    // SAFETY: dosya açık, boyut doğru; yalnızca okuma izniyle, özel eşleme.
    let p = unsafe {
        libc::mmap(std::ptr::null_mut(), boyut, libc::PROT_READ, libc::MAP_PRIVATE, f.as_raw_fd(), 0)
    };
    assert_ne!(p, libc::MAP_FAILED);
    // SAFETY: eşleme boyut bayt boyunca okunabilir; bu dilim munmap'e kadar kullanılıyor.
    let veri: &[u8] = unsafe { std::slice::from_raw_parts(p as *const u8, boyut) };
    println!("eşlendi: {:p}, ilk 7 bayt: {:?}", p, std::str::from_utf8(&veri[..7]).unwrap());
    let once = kucuk_hata();
    let mut toplam = 0u64;
    for i in (0..boyut).step_by(4096) {
        toplam += veri[i] as u64; // her sayfanın ilk baytı: ilk dokunuşta sayfa hatası
    }
    let sonra = kucuk_hata();
    println!("{} sayfaya dokunuldu → {} küçük sayfa hatası (toplam = {toplam})", boyut / 4096, sonra - once);
    // SAFETY: p ve boyut mmap'ten geldi; veri artık kullanılmıyor.
    unsafe { libc::munmap(p, boyut) };
}
