/// Ajanın ödünç denetleyicisini "aşmak" için yazdığı yardımcı
fn yerinde_artir(p: *const u32) {
    unsafe { *(p as *mut u32) += 1 } // salt-okunur ödünçten türeyen işaretçiyle yazma
}

/// Paylaşılan (&) referans üzerinden yazıyor: tanımsız davranış!
pub fn say_ve_artir(sayac: &u32) -> u32 {
    yerinde_artir(sayac);
    *sayac
}

/// Ajanın yazdığı "hızlı" medyan: sıralı girdide doğru, sırasızda yanlış
pub fn medyan_hizli(v: &[i32]) -> Option<i32> {
    if v.is_empty() { None } else { Some(v[v.len() / 2]) }
}

/// Başvuru (referans) uygulaması: yavaş ama açıkça doğru
pub fn medyan_basvuru(v: &[i32]) -> Option<i32> {
    let mut s = v.to_vec();
    s.sort();
    if s.is_empty() { None } else { Some(s[s.len() / 2]) }
}

#[cfg(test)]
mod testler {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn sayac_artiyor() {
        let x = 41;
        assert_eq!(say_ve_artir(&x), 42);
    }

    #[test]
    fn medyan_ornek() {
        assert_eq!(medyan_hizli(&[1, 2, 3]), Some(2)); // ajanın tek örneği: sıralı!
    }

    proptest! {
        // Farklılık testi: hızlı sürüm, başvuru sürümüyle her girdide aynı mı?
        #[test]
        fn medyan_basvuruyla_ayni(v in proptest::collection::vec(-100i32..100, 0..20)) {
            prop_assert_eq!(medyan_hizli(&v), medyan_basvuru(&v));
        }
    }
}
