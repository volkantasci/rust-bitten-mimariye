/// Sayıların aritmetik ortalaması. Boş girdide `None`.
///
/// ```
/// assert_eq!(istatistik::ortalama(&[2.0, 4.0]), Some(3.0));
/// assert_eq!(istatistik::ortalama(&[]), None);
/// ```
pub fn ortalama(v: &[f64]) -> Option<f64> {
    if v.is_empty() {
        None
    } else {
        Some(v.iter().sum::<f64>() / v.len() as f64)
    }
}

/// Ortanca (medyan).
pub fn medyan(v: &mut [f64]) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.total_cmp(b));
    let m = v.len() / 2;
    if v.len() % 2 == 0 {
        Some((v[m - 1] + v[m]) / 2.0) // çift uzunluk: ortadaki iki değerin ortalaması
    } else {
        Some(v[m])
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn bos_listenin_ortalamasi_yok() {
        assert_eq!(ortalama(&[]), None);
    }

    #[test]
    fn tek_uzunlukta_medyan() {
        assert_eq!(medyan(&mut [3.0, 1.0, 2.0]), Some(2.0));
    }

    #[test]
    fn cift_uzunlukta_medyan() {
        assert_eq!(medyan(&mut [4.0, 1.0, 3.0, 2.0]), Some(2.5));
    }
}
