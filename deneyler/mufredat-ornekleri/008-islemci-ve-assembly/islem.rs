#[unsafe(no_mangle)]
pub fn topla(a: i64, b: i64) -> i64 {
    a + b
}

#[unsafe(no_mangle)]
pub fn toplam(n: u64) -> u64 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += i;
        i += 1;
    }
    s
}
