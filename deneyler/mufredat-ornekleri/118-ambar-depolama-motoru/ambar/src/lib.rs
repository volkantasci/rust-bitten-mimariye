//! Ambar — bitirme projesi, 118. adım: günlük yapılı (log-structured) anahtar-değer deposu.
//! Bellekte bir indeks (HashMap), diskte yalnızca sonuna eklenen kayıtlardan oluşan günlük; her kaydın
//! uzunluk alanlarını bir başlık CRC'si, verisini bir veri CRC'si korur.
pub mod crc;
pub mod gunluk;
pub mod motor;
