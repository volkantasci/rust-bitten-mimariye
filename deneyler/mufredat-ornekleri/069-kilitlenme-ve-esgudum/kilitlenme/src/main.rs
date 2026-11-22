use std::sync::Mutex;
use std::thread;
use std::time::Duration;

static HESAP_A: Mutex<i64> = Mutex::new(100);
static HESAP_B: Mutex<i64> = Mutex::new(100);

fn main() {
    let sirali = std::env::args().any(|a| a == "sirali");
    thread::scope(|s| {
        s.spawn(|| {
            let a = HESAP_A.lock().unwrap(); // önce A
            println!("1. iş parçacığı A'yı aldı, B'yi bekliyor");
            thread::sleep(Duration::from_millis(100));
            let b = HESAP_B.lock().unwrap();
            println!("1. iş parçacığı transferi yaptı ({} / {})", *a, *b);
        });
        s.spawn(move || {
            // Hatalı sürüm: ters sırada (önce B). Düzeltilmiş sürüm: herkes önce A'yı alır.
            let (ilk, ikinci) = if sirali { (&HESAP_A, &HESAP_B) } else { (&HESAP_B, &HESAP_A) };
            let x = ilk.lock().unwrap();
            println!("2. iş parçacığı ilk kilidi aldı, ikincisini bekliyor");
            thread::sleep(Duration::from_millis(100));
            let y = ikinci.lock().unwrap();
            println!("2. iş parçacığı transferi yaptı ({} / {})", *x, *y);
        });
    });
    println!("bitti");
}
