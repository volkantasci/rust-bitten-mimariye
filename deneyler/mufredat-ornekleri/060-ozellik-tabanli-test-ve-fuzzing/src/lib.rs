/// İki sayının ortası (aşağı yuvarlanmış). Saf ve hatalı sürüm.
pub fn orta_saf(a: u32, b: u32) -> u32 {
    (a + b) / 2 // a + b taşabilir!
}

/// Taşmayan sürüm: ortak bitler + farklı bitlerin yarısı.
pub fn orta_guvenli(a: u32, b: u32) -> u32 {
    (a & b) + ((a ^ b) >> 1)
}

#[cfg(test)]
mod testler {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn elle_secilmis_ornekler() {
        assert_eq!(orta_saf(2, 4), 3); // birkaç örnek: geçer, hata gizli kalır
        assert_eq!(orta_saf(10, 20), 15);
    }

    proptest! {
        // Özellik: sonuç her zaman min(a,b) ile max(a,b) arasında olmalı
        #[test]
        fn saf_surum_aralikta(a: u32, b: u32) {
            let m = orta_saf(a, b);
            prop_assert!(a.min(b) <= m && m <= a.max(b));
        }

        #[test]
        fn guvenli_surum_aralikta(a: u32, b: u32) {
            let m = orta_guvenli(a, b);
            prop_assert!(a.min(b) <= m && m <= a.max(b));
        }
    }
}
