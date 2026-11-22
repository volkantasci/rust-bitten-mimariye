use std::time::Instant;

fn main() {
    // xorshift: bağımlılıksız basit sözde-rastgele üreteç
    let mut durum: u64 = 0x2545_F491_4F6C_DD1D;
    let mut rastgele = move || {
        durum ^= durum << 13;
        durum ^= durum >> 7;
        durum ^= durum << 17;
        durum
    };
    println!("{:>14} {:>12}", "çalışma kümesi", "ns/erişim");
    for kib in [16usize, 32, 64, 128, 256, 512, 1024, 2048, 8192, 16384, 32768, 65536, 262144] {
        let n = kib * 1024 / std::mem::size_of::<usize>();
        // Sattolo algoritması: tek halkalı rastgele permütasyon.
        // sonraki[i] bir sonraki adresi tutar; donanım öngörücüsü tahmin edemez.
        let mut sonraki: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            let j = (rastgele() % i as u64) as usize;
            sonraki.swap(i, j);
        }
        let adim = 10_000_000;
        let mut p = 0;
        let t = Instant::now();
        for _ in 0..adim {
            p = sonraki[p]; // her okuma bir öncekine bağlı: gecikme ölçülür
        }
        let ns = t.elapsed().as_nanos() as f64 / adim as f64;
        println!("{:>10} KiB {:>12.2}   (son={p})", kib, ns);
    }
}
