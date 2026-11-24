//! Ambar — bitirme projesi, 117. adım: günlük yapılı (log-structured) anahtar-değer deposu.
//! Bellekte bir indeks (HashMap), diskte yalnızca sonuna eklenen (append-only) bir günlük.
pub mod gunluk;
pub mod motor;
