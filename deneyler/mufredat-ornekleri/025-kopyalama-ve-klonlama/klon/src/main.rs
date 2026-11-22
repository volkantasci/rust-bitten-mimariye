use std::mem::size_of;

#[derive(Clone, Copy)]
struct Nokta {
    x: i32,
    y: i32,
}

fn main() {
    println!("size_of::<String>() = {} bayt (işaretçi + kapasite + uzunluk)", size_of::<String>());
    let a = 5;
    let b = a; // i32 Copy'dir: bitler kopyalanır, a geçerli kalır
    println!("a = {a}, b = {b}");
    let p = Nokta { x: 1, y: 2 };
    let q = p; // Copy türettik: p hâlâ geçerli
    println!("p = ({}, {}), q = ({}, {})", p.x, p.y, q.x, q.y);
    let s = String::from("derin kopya");
    let t = s.clone(); // öbekteki veri de kopyalanır
    println!("s verisi @ {:p}, t verisi @ {:p}", s.as_ptr(), t.as_ptr());
}
