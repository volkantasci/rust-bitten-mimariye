/// Metni en fazla `genislik` karakterlik satırlara böler.
/// Sözleşme: kelimeler bölünmez; genişlik karakter (char) sayısıdır;
/// genişlikten uzun kelime kendi satırına konur; boş metin boş liste verir.
pub fn sarmala(metin: &str, genislik: usize) -> Vec<String> {
    let _ = (metin, genislik);
    todo!("henüz yazılmadı: önce sözleşme ve testler")
}

#[cfg(test)]
mod sozlesme {
    use super::*;

    #[test]
    fn kisa_metin_tek_satir() {
        assert_eq!(sarmala("merhaba dünya", 20), vec!["merhaba dünya"]);
    }
    #[test]
    fn kelimeler_bolunmez() {
        assert_eq!(sarmala("bir iki üç dört", 7), vec!["bir iki", "üç dört"]);
    }
    #[test]
    fn genislik_karakterle_olculur_bayt_degil() {
        assert_eq!(sarmala("ğğğ ççç", 3), vec!["ğğğ", "ççç"]);
    }
    #[test]
    fn uzun_kelime_kendi_satirinda() {
        assert_eq!(sarmala("a çokuzunkelime b", 5), vec!["a", "çokuzunkelime", "b"]);
    }
    #[test]
    fn bos_metin_bos_liste() {
        assert!(sarmala("   ", 10).is_empty());
    }
}
