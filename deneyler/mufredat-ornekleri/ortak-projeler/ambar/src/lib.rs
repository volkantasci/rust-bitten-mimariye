//! Ambar — bitirme projesi, 121. adım (bitmiş hâl): günlük yapılı (log-structured) anahtar-değer deposu
//! ve satır tabanlı metin protokolü (TCP); eşzamanlı istemciler; fsync politikaları ve toplu kesinleştirme (group commit).
pub mod crc;
pub mod gunluk;
pub mod motor;
pub mod protokol;
pub mod sunucu;
