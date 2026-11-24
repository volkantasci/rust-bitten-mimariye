//! Ambar: bitirme projesi — günlük yapılı (log-structured) anahtar-değer deposu.
//! Bellekte HashMap dizini + diskte yalnızca-ekle (append-only) günlük + TCP metin protokolü.
pub mod crc;
pub mod gunluk;
pub mod motor;
pub mod protokol;
pub mod sunucu;
