use std::collections::HashMap;

fn fib(n: u64, cagri: &mut u64) -> u64 {
    *cagri += 1;
    if n < 2 { n } else { fib(n - 1, cagri) + fib(n - 2, cagri) }
}

fn fib_hafiza(n: u64, hafiza: &mut HashMap<u64, u64>, cagri: &mut u64) -> u64 {
    *cagri += 1;
    if n < 2 {
        return n;
    }
    if let Some(&v) = hafiza.get(&n) {
        return v;
    }
    let v = fib_hafiza(n - 1, hafiza, cagri) + fib_hafiza(n - 2, hafiza, cagri);
    hafiza.insert(n, v);
    v
}

fn derin(n: u64) -> u64 {
    let tampon = [n as u8; 1024]; // her çerçeve en az 1 KiB yer kaplasın
    if n == 0 { 0 } else { tampon[(n % 1024) as usize] as u64 + derin(n - 1) }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();
    if arg == "derin" {
        let n: u64 = std::env::args().nth(2).unwrap().parse().unwrap();
        println!("derin({n}) = {}", derin(n));
        return;
    }
    if arg == "buyuk-yigin" {
        let n: u64 = std::env::args().nth(2).unwrap().parse().unwrap();
        let t = std::thread::Builder::new().stack_size(512 * 1024 * 1024);
        let sonuc = t.spawn(move || derin(n)).unwrap().join().unwrap();
        println!("512 MiB yığınlı iş parçacığında derin({n}) = {sonuc}");
        return;
    }
    for n in [5, 10, 20, 30] {
        let mut c1 = 0;
        let mut c2 = 0;
        let a = fib(n, &mut c1);
        let b = fib_hafiza(n, &mut HashMap::new(), &mut c2);
        println!("fib({n}) = {a:>6}  saf: {c1:>9} çağrı   memoization ile: {c2:>3} çağrı  (aynı mı: {})", a == b);
    }
}
