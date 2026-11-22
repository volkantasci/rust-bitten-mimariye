/// Bir dilimi `orta` noktasından iki ayrık, değiştirilebilir parçaya böler.
/// (std'deki `split_at_mut`'ın küçük bir eşi.)
pub fn ikiye_bol(v: &mut [i32], orta: usize) -> (&mut [i32], &mut [i32]) {
    assert!(orta <= v.len()); // güvenli API'nin sözleşmesi: denetim burada
    let n = v.len();
    let p = v.as_mut_ptr();
    // SAFETY: orta <= n olduğu denetlendi; [0, orta) ve [orta, n) çakışmaz,
    // ikisi de v'nin içinde ve v'nin ödünç süresi boyunca geçerli.
    unsafe {
        (
            std::slice::from_raw_parts_mut(p, orta),
            std::slice::from_raw_parts_mut(p.add(orta), n - orta),
        )
    }
}

/// HATALI sürüm: ikinci parça bir eleman erken başlıyor, parçalar çakışıyor.
pub fn ikiye_bol_hatali(v: &mut [i32], orta: usize) -> (&mut [i32], &mut [i32]) {
    assert!(orta >= 1 && orta <= v.len());
    let n = v.len();
    let p = v.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(p, orta),
            std::slice::from_raw_parts_mut(p.add(orta - 1), n - orta + 1),
        )
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn dogru_surum() {
        let mut v = [1, 2, 3, 4, 5];
        let (a, b) = ikiye_bol(&mut v, 2);
        a[0] = 10;
        b[0] = 30;
        assert_eq!(v, [10, 2, 30, 4, 5]);
    }

    #[test]
    fn hatali_surum() {
        let mut v = [1, 2, 3, 4, 5];
        let (a, b) = ikiye_bol_hatali(&mut v, 2);
        a[1] = 20; // aynı elemana iki ayrı &mut üzerinden yazılıyor
        b[0] = 99;
        assert_eq!(v[1], 99);
    }
}
