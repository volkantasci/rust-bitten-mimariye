//! Koruma sayfası: 1 MiB yığınla başlatılan bir iş parçacığı, kendi yığınının ve hemen
//! altındaki eşlemenin /proc/self/maps satırlarını yazar. Yığın aşağı doğru büyür; en
//! alttaki sayfaya erişim izni yoktur (---p): oraya dokunan bir erişim sayfa hatası verir.
use std::io::Read;

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(|| {
            let yerel = 0u8; // bu değişken iş parçacığının yığınında
            let adres = &yerel as *const u8 as usize;
            let mut harita = String::new();
            std::fs::File::open("/proc/self/maps").unwrap().read_to_string(&mut harita).unwrap();
            let satirlar: Vec<&str> = harita.lines().collect();
            for (i, satir) in satirlar.iter().enumerate() {
                let (bas, son) = satir.split(' ').next().unwrap().split_once('-').unwrap();
                let bas = usize::from_str_radix(bas, 16).unwrap();
                let son = usize::from_str_radix(son, 16).unwrap();
                if bas <= adres && adres < son {
                    println!("yığının altındaki eşleme: {}", satirlar[i - 1].trim_end());
                    println!("iş parçacığının yığını:   {}", satir.trim_end());
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}
