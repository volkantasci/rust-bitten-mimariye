/// Esnek girdi: &str, String, &String... hepsini kabul eder
pub fn selamla(ad: impl AsRef<str>) -> String {
    format!("Merhaba, {}!", ad.as_ref())
}

/// Sahipliği gerçekten gerekiyorsa açıkça iste
pub struct Kullanici {
    pub ad: String,
}
impl Kullanici {
    pub fn yeni(ad: impl Into<String>) -> Self {
        Kullanici { ad: ad.into() }
    }
    #[must_use = "yeni bir Kullanici döndürür, mevcut olanı değiştirmez"]
    pub fn yeniden_adlandir(&self, ad: &str) -> Kullanici {
        Kullanici { ad: ad.to_string() }
    }
}

/// İleride yeni hata türleri eklenebilir: dışarıdaki kod buna hazırlıklı olmalı
#[non_exhaustive]
#[derive(Debug)]
pub enum Hata {
    Bulunamadi,
    YetkiYok,
}
