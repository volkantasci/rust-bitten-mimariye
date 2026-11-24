use std::time::Instant;

// xorshift: bağımlılıksız basit sözde-rastgele üreteç
fn uretec() -> impl FnMut() -> u64 {
    let mut durum: u64 = 0x2545_F491_4F6C_DD1D;
    move || {
        durum ^= durum << 13;
        durum ^= durum >> 7;
        durum ^= durum << 17;
        durum
    }
}

// Sattolo algoritması: tek halkalı rastgele permütasyon. sonraki[i] bir sonraki indisi tutar.
fn sattolo(n: usize, rastgele: &mut impl FnMut() -> u64) -> Vec<usize> {
    let mut sonraki: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        let j = (rastgele() % i as u64) as usize;
        sonraki.swap(i, j);
    }
    sonraki
}

fn main() {
    let sirali = std::env::args().any(|a| a == "sirali");
    if !sirali {
        // Küçük bir örnek: 8 düğümlük halkayı 0'dan başlayıp izle
        let ornek = sattolo(8, &mut uretec());
        let mut yol = vec![0];
        let mut p = ornek[0];
        while p != 0 {
            yol.push(p);
            p = ornek[p];
        }
        yol.push(0);
        let yazi: Vec<String> = yol.iter().map(|i| i.to_string()).collect();
        println!("örnek halka (n = 8): {}", yazi.join(" → "));
    }
    let mut rastgele = uretec();
    println!("erişim düzeni: {}", if sirali { "sıralı" } else { "rastgele (Sattolo)" });
    println!("{:>14} {:>12}", "çalışma kümesi", "ns/erişim");
    let boyutlar: &[usize] = if sirali {
        &[16, 1024, 32768, 262144]
    } else {
        &[16, 32, 64, 128, 256, 512, 1024, 2048, 8192, 16384, 32768, 65536, 262144]
    };
    for &kib in boyutlar {
        let n = kib * 1024 / std::mem::size_of::<usize>();
        let sonraki: Vec<usize> = if sirali { (0..n).map(|i| (i + 1) % n).collect() } else { sattolo(n, &mut rastgele) };
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
