use std::future::Future;
use std::pin::{Pin, pin};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant};

// Uyandırıcı: beklemedeki yürütücü iş parçacığını dürter
struct Durtucu(thread::Thread);
impl Wake for Durtucu {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(gelecek: F) -> F::Output {
    let mut gelecek = pin!(gelecek);
    let uyandirici = Waker::from(Arc::new(Durtucu(thread::current())));
    let mut cx = Context::from_waker(&uyandirici);
    let mut yoklama = 0;
    loop {
        yoklama += 1;
        if let Poll::Ready(sonuc) = gelecek.as_mut().poll(&mut cx) {
            println!("[yürütücü] {yoklama} yoklamada tamamlandı");
            return sonuc;
        }
        thread::park(); // CPU yakmadan, uyandırılana kadar uyu
    }
}

// Zamanlayıcı future: ayrı bir iş parçacığı süre dolunca uyandırıcıyı çağırır
struct Zamanlayici(Arc<Mutex<(bool, Option<Waker>)>>);
impl Zamanlayici {
    fn yeni(sure: Duration) -> Self {
        let durum = Arc::new(Mutex::new((false, None::<Waker>)));
        let d2 = Arc::clone(&durum);
        thread::spawn(move || {
            thread::sleep(sure);
            let mut d = d2.lock().unwrap();
            d.0 = true;
            if let Some(w) = d.1.take() {
                w.wake();
            }
        });
        Zamanlayici(durum)
    }
}
impl Future for Zamanlayici {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut d = self.0.lock().unwrap();
        if d.0 {
            Poll::Ready(())
        } else {
            d.1 = Some(cx.waker().clone()); // hazır olunca beni uyandır
            Poll::Pending
        }
    }
}

async fn gorev(t0: Instant) -> u32 {
    println!("[görev] başladı          ({:>3} ms)", t0.elapsed().as_millis());
    Zamanlayici::yeni(Duration::from_millis(200)).await;
    println!("[görev] 1. bekleme bitti ({:>3} ms)", t0.elapsed().as_millis());
    Zamanlayici::yeni(Duration::from_millis(300)).await;
    println!("[görev] 2. bekleme bitti ({:>3} ms)", t0.elapsed().as_millis());
    42
}

fn main() {
    let t0 = Instant::now();
    let sonuc = block_on(gorev(t0));
    println!("sonuç = {sonuc}");
}
