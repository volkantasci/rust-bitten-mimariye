#[unsafe(no_mangle)]
pub fn elle_dongu(v: &[u32]) -> u32 {
    let mut toplam = 0u32;
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 == 0 {
            toplam = toplam.wrapping_add(v[i].wrapping_mul(v[i]));
        }
        i += 1;
    }
    toplam
}

#[unsafe(no_mangle)]
pub fn iterator_zinciri(v: &[u32]) -> u32 {
    v.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x.wrapping_mul(x))
        .fold(0u32, |a, b| a.wrapping_add(b))
}
