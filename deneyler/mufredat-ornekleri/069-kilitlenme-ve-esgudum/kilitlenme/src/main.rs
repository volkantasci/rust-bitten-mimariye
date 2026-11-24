use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static HESAP_A: Mutex<i64> = Mutex::new(100);
static HESAP_B: Mutex<i64> = Mutex::new(100);

// Hatalı sürüm: iki iş parçacığı iki kilidi ters sırayla alır.
// "sirali": düzeltilmiş sürüm, herkes önce A'yı alır.
// "dongu N": beklemesiz, her iş parçacığı N kez transfer yapar (kilitlenme yalnız bazen olur).
// "dongu-sirali N": aynı döngü, ama iki iş parçacığı da önce A'yı alır.
fn main() {
    let kip = std::env::args().nth(1).unwrap_or_default();
    let sirali = kip == "sirali" || kip == "dongu-sirali";
    let tur: u32 = if kip.starts_with("dongu") { std::env::args().nth(2).map_or(1000, |s| s.parse().unwrap()) } else { 1 };
    let konus = tur == 1; // tek turda adım adım yaz ve 100 ms bekle
    thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..tur {
                let mut a = HESAP_A.lock().unwrap(); // önce A
                if konus {
                    println!("1. iş parçacığı A'yı aldı, B'yi bekliyor");
                    thread::sleep(Duration::from_millis(100));
                }
                let mut b = HESAP_B.lock().unwrap();
                *a -= 1;
                *b += 1;
                if konus {
                    println!("1. iş parçacığı transferi yaptı (A = {}, B = {})", *a, *b);
                }
            }
        });
        s.spawn(|| {
            let (ilk, ikinci) = if sirali { (&HESAP_A, &HESAP_B) } else { (&HESAP_B, &HESAP_A) };
            for _ in 0..tur {
                let mut x = ilk.lock().unwrap();
                if konus {
                    println!("2. iş parçacığı ilk kilidi aldı, ikincisini bekliyor");
                    thread::sleep(Duration::from_millis(100));
                }
                let mut y = ikinci.lock().unwrap();
                *x += 1;
                *y -= 1;
                if konus {
                    println!("2. iş parçacığı transferi yaptı");
                }
            }
        });
    });
    println!("bitti: A = {}, B = {}", HESAP_A.lock().unwrap(), HESAP_B.lock().unwrap());
}
