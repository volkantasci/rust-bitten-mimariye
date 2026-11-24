# 067 — kaldırılan C deneyi (2026-11-24, Sezon 3 revizyonu)

`c/yaris.c` (iki pthread ile korumasız `volatile long` sayaç; gcc -O2 ve ThreadSanitizer) müfredattan çıkarıldı.
Neden: müfredat revizyon ilkeleri §1 (konu Rust'ın kendisidir; başka dil kodu yalnız FFI ya da kavramın
en doğal aracı Rust dışındaysa). Aynı kavram (okuma-değiştirme-yazma yarışı, kaybolan artışlar, tanımsız
davranışın araçla yakalanması) artık Rust'ta gösteriliyor: `unsafe` + `static mut` ile yarış, miri ile
"Data race detected" tanısı (`067-veri-yarisi-ve-send-sync/yaris/`).
