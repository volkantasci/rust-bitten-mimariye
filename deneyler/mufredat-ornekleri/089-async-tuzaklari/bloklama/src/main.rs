use std::time::{Duration, Instant};

struct Iz(&'static str);
impl Drop for Iz {
    fn drop(&mut self) {
        println!("  [{}] future düşürüldü (iptal)", self.0);
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 1) async içinde bloklayan uyku: tek iş parçacığı kilitlenir, görevler sıraya girer
    let t = Instant::now();
    let a = tokio::spawn(async { std::thread::sleep(Duration::from_millis(500)) });
    let b = tokio::spawn(async { std::thread::sleep(Duration::from_millis(500)) });
    let _ = tokio::join!(a, b);
    println!("std::thread::sleep ile iki görev : {:>4} ms", t.elapsed().as_millis());

    let t = Instant::now();
    let a = tokio::spawn(tokio::time::sleep(Duration::from_millis(500)));
    let b = tokio::spawn(tokio::time::sleep(Duration::from_millis(500)));
    let _ = tokio::join!(a, b);
    println!("tokio::time::sleep ile iki görev : {:>4} ms", t.elapsed().as_millis());

    // 2) İptal: select! kaybeden dalı düşürür; yarım kalan iş geri alınmaz
    tokio::select! {
        _ = async {
            let _iz = Iz("uzun iş");
            println!("  [uzun iş] 1. adım yapıldı");
            tokio::time::sleep(Duration::from_millis(200)).await;
            println!("  [uzun iş] 2. adım yapıldı (buraya hiç gelinmez)");
        } => {}
        _ = tokio::time::sleep(Duration::from_millis(50)) => println!("zaman aşımı kazandı"),
    }
}
