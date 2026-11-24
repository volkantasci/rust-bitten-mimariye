use std::fs;

fn durum(alan: &str) -> String {
    let s = fs::read_to_string("/proc/self/status").unwrap();
    s.lines().find(|l| l.starts_with(alan)).unwrap().split_whitespace().skip(1).collect::<Vec<_>>().join(" ")
}

fn kucuk_hata() -> u64 {
    // /proc/self/stat 10. alan: minflt (diske gitmeyen, küçük sayfa hatası sayısı)
    let s = fs::read_to_string("/proc/self/stat").unwrap();
    let sonra = &s[s.rfind(')').unwrap() + 2..];
    sonra.split_whitespace().nth(7).unwrap().parse().unwrap()
}

fn main() {
    let yerel = 0u8;
    println!("yığın adresi: {:p}   öbek adresi: {:p}", &yerel, Box::new(0u8));
    println!("başlangıç        : VmRSS {:>12}  sayfa hatası {:>7}", durum("VmRSS:"), kucuk_hata());
    let mut v = vec![0u8; 1 << 30]; // 1 GiB, sıfırlarla
    println!("1 GiB ayrıldı    : VmRSS {:>12}  sayfa hatası {:>7}", durum("VmRSS:"), kucuk_hata());
    for i in (0..v.len()).step_by(4096) {
        v[i] = 1; // her 4 KiB'lik sayfaya bir kez dokun
    }
    println!("sayfalara dokundu: VmRSS {:>12}  sayfa hatası {:>7}", durum("VmRSS:"), kucuk_hata());
    let dev = fs::read_to_string("/proc/self/smaps_rollup").unwrap();
    let dev = dev.lines().find(|l| l.starts_with("AnonHugePages:")).unwrap();
    println!("{dev}   (2 MiB'lik büyük sayfalarla karşılanan kısım)");
    println!("toplam: {}", v.iter().step_by(4096).map(|&b| b as u64).sum::<u64>());
}
