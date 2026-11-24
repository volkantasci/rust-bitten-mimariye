//! Ambar — bitirme projesi, 120. adım: günlük yapılı (log-structured) anahtar-değer deposu
//! ve satır tabanlı metin protokolü (TCP); eşzamanlı istemciler ve fsync politikası.
pub mod crc;
pub mod gunluk;
pub mod motor;
pub mod protokol;
pub mod sunucu;
