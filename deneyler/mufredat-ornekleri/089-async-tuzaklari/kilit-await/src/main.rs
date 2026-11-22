use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let sayac = Arc::new(Mutex::new(0));
    let s = Arc::clone(&sayac);
    tokio::spawn(async move {
        let mut kilit = s.lock().unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await; // kilit elde tutulurken bekleme
        *kilit += 1;
    })
    .await
    .unwrap();
    println!("{}", sayac.lock().unwrap());
}
