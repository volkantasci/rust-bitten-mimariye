#[unsafe(no_mangle)]
pub fn kareler_toplami(n: u64) -> u64 {
    let mut s = 0u64;
    for i in 0..n {
        s = s.wrapping_add(i * i);
    }
    s
}
