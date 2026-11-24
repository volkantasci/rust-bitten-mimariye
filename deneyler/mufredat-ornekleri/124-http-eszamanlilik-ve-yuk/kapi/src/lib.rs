//! Kapı — bitirme projesi, 124. adım (bitmiş hâl): sıfırdan bir HTTP/1.1 sunucusu.
//! Artımlı ayrıştırıcı, güvenli yol çözümü ile statik dosya sunumu; aynı ayrıştırıcıyla
//! iki sunucu: iş parçacığı havuzu ve tokio; isteğe bağlı istek son süresi (408).
pub mod ayristirici;
pub mod dosya;
pub mod havuz;
pub mod zaman_uyumsuz;
