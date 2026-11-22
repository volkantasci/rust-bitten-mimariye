/// Metni en fazla `genislik` karakterlik satırlara böler.
/// Sözleşme: kelimeler bölünmez; genişlik karakter (char) sayısıdır;
/// genişlikten uzun kelime kendi satırına konur; boş metin boş liste verir.
pub fn sarmala(metin: &str, genislik: usize) -> Vec<String> {
    let mut satirlar = Vec::new();
    let mut satir = String::new();
    let mut satir_uzunlugu = 0; // karakter sayısı (bayt değil)
    for kelime in metin.split_whitespace() {
        let n = kelime.chars().count();
        if satir_uzunlugu > 0 && satir_uzunlugu + 1 + n > genislik {
            satirlar.push(std::mem::take(&mut satir));
            satir_uzunlugu = 0;
        }
        if satir_uzunlugu > 0 {
            satir.push(' ');
            satir_uzunlugu += 1;
        }
        satir.push_str(kelime);
        satir_uzunlugu += n;
    }
    if !satir.is_empty() {
        satirlar.push(satir);
    }
    satirlar
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
