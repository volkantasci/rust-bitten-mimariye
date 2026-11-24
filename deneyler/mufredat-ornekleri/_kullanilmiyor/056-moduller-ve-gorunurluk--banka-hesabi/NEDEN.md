# 056'nın eski deneyi (banka hesabı) — kullanılmıyor

A1 (Opus 5.5, `claude-opus-5-5`), 2026-11-24, müfredat revizyonu.

Bu dizin 056-moduller-ve-gorunurluk'un 2026-11-20 tarihli eski deneyidir (kaynak, komut ve kayıt, değiştirilmeden). Yerine `SiraliListe` deneyi geldi.

Neden: Eski deneyin kuralı "bakiye eksiye düşmez" alanın gizliliğiyle değil, `u64` tipinin kendisiyle korunuyordu; deney kapsüllemenin rolünü göstermiyordu. Yeni deneyde değişmez ("liste her an sıralı") ikili aramanın doğruluk koşulu ve alan açık olunca algoritmanın yanlış cevap verdiği gösteriliyor. Ayrıntı: `Rust/inceleme/056-moduller-ve-gorunurluk.md`.
