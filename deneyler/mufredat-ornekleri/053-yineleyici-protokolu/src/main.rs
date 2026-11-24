struct Fibonacci {
    a: u64,
    b: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let r = self.a;
        self.a = self.b;
        self.b += r;
        Some(r) // hiç bitmeyen bir dizi
    }
}

fn main() {
    let v = vec![1, 2, 3];
    v.iter().map(|x| {
        println!("işleniyor {x}");
        x * 2
    }); // tembel: kimse sonuç istemedi, hiçbir şey çalışmaz
    println!("--- collect ile:");
    let ikiler: Vec<i32> = v.iter().map(|x| {
        println!("işleniyor {x}");
        x * 2
    }).collect();
    println!("{ikiler:?}");

    let ilk10: Vec<u64> = Fibonacci { a: 0, b: 1 }.take(10).collect();
    println!("{ilk10:?}");
    let tek_kareler: Vec<u64> = (1..).map(|x: u64| x * x).filter(|k| k % 2 == 1).take(4).collect();
    println!("{tek_kareler:?}");
}
