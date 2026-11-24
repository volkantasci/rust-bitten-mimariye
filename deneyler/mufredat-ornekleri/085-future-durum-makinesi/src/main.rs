use std::future::Future;
use std::mem::size_of_val;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker};

async fn bos() {}

async fn tampon_await_boyunca() -> usize {
    let tampon = [0u8; 4096];
    bos().await; // tampon bu noktadan sonra da kullanılıyor: future'ın içinde saklanmalı
    tampon.len()
}

async fn tampon_await_oncesi() -> usize {
    let n = {
        let tampon = [0u8; 4096];
        tampon.len()
    }; // tampon await'ten önce bitti: saklanmasına gerek yok
    bos().await;
    n
}

// Kendin dene: iki await noktası ve küçük (64 baytlık) tampon
async fn tampon_iki_await() -> usize {
    let n = {
        let tampon = [0u8; 4096];
        tampon.len()
    };
    bos().await;
    bos().await;
    n
}

async fn kucuk_tampon_await_boyunca() -> usize {
    let tampon = [0u8; 64];
    bos().await;
    tampon.len()
}

// Elle yazılmış bir future: n kez "henüz değil" der
struct GeriSayim(u32);
impl Future for GeriSayim {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 == 0 {
            Poll::Ready("bitti")
        } else {
            self.0 -= 1;
            cx.waker().wake_by_ref(); // "beni tekrar yokla"
            Poll::Pending
        }
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("kendin-dene") {
        println!("size_of_val(tampon_iki_await())          = {} bayt", size_of_val(&tampon_iki_await()));
        println!("size_of_val(kucuk_tampon_await_boyunca()) = {} bayt", size_of_val(&kucuk_tampon_await_boyunca()));
        return;
    }
    println!("size_of_val(bos())                  = {} bayt", size_of_val(&bos()));
    println!("size_of_val(tampon_await_boyunca()) = {} bayt", size_of_val(&tampon_await_boyunca()));
    println!("size_of_val(tampon_await_oncesi())  = {} bayt", size_of_val(&tampon_await_oncesi()));

    let tembel = async { println!("async blok çalıştı"); };
    println!("async blok oluşturuldu; henüz hiçbir şey olmadı");

    let mut cx = Context::from_waker(Waker::noop());
    let mut f = pin!(tembel);
    println!("ilk poll → {:?}", f.as_mut().poll(&mut cx));

    let mut g = pin!(GeriSayim(3));
    let mut sayac = 1;
    loop {
        match g.as_mut().poll(&mut cx) {
            Poll::Pending => println!("poll {sayac} → Pending"),
            Poll::Ready(v) => { println!("poll {sayac} → Ready({v:?})"); break; }
        }
        sayac += 1;
    }
}
