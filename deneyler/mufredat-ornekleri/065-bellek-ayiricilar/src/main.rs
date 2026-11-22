use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

// Mikroskop: tüm ayırmaları sayan, işi sistemin ayırıcısına devreden bir ayırıcı
struct Sayan;
static AYIRMA: AtomicUsize = AtomicUsize::new(0);
static BUYUTME: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Sayan {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        AYIRMA.fetch_add(1, Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, yeni: usize) -> *mut u8 {
        BUYUTME.fetch_add(1, Relaxed);
        unsafe { System.realloc(p, l, yeni) }
    }
}

#[global_allocator]
static AYIRICI: Sayan = Sayan;

fn olc(ad: &str, f: impl FnOnce()) {
    let (a0, b0) = (AYIRMA.load(Relaxed), BUYUTME.load(Relaxed));
    f();
    println!("{ad:<42} ayırma {:>5}  büyütme {:>3}", AYIRMA.load(Relaxed) - a0, BUYUTME.load(Relaxed) - b0);
}

fn main() {
    olc("String::new + 1000 × push_str", || {
        let mut s = String::new();
        for _ in 0..1000 { s.push_str("ab"); }
        black_box(s);
    });
    olc("String::with_capacity(2000) + 1000 × push", || {
        let mut s = String::with_capacity(2000);
        for _ in 0..1000 { s.push_str("ab"); }
        black_box(s);
    });
    olc("1000 × format!(\"{i}\") → Vec<String>", || {
        let v: Vec<String> = (0..1000).map(|i| format!("{i}")).collect();
        black_box(v);
    });
    olc("1000 × Box::new(i)", || {
        let v: Vec<Box<u64>> = (0..1000).map(Box::new).collect();
        black_box(v);
    });
}
