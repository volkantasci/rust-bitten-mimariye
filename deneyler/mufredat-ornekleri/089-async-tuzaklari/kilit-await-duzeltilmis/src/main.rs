use std::sync::{Arc, Mutex};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let sayac = Arc::new(Mutex::new(0));
    let s = Arc::clone(&sayac);
    tokio::spawn(async move {
        {
            let mut kilit = s.lock().unwrap();
            *kilit += 1;
        } // koruyucu bu blokta düşer: kilit await'ten önce bırakılır
        tokio::time::sleep(Duration::from_millis(10)).await;
    })
    .await
    .unwrap();
    println!("sayaç = {}", sayac.lock().unwrap());
}
