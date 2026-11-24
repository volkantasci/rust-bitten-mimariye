#@ video: 090-unsafe-ne-demek
# Ham işaretçi oluşturmak güvenli; gösterdiği yere erişmek unsafe ister (E0133)
cd ham-isaretci && k cargo build -q; cd ..
# Hangi işlemler unsafe gerektirir? Derleyicinin kendi açıklamasındaki örnekler
ks 'rustc --explain E0133 | sed -n "/^Using unsafe/,/^\* Calling functions marked/p"'
cd sozlesme && k cargo run -q; cd ..
# 2024 sürümünün kuralları: no_mangle ve extern blokları da "unsafe" diye işaretlenmeli
cd surum-2024 && k cargo build -q; cd ..
# Kendin dene: aynı dosya 2021 baskısıyla derlenir; bu iki kural 2024 baskısıyla geldi
ks 'rustc --edition 2021 -o /tmp/surum2021 surum-2024/src/main.rs && /tmp/surum2021'
# Kendin dene: sozlesme'de *p'yi unsafe { *p } içine almak; 2024 uyarısı kaybolur
ks 'rm -rf /tmp/sozlesme-kopya && mkdir /tmp/sozlesme-kopya && cp -r sozlesme/Cargo.toml sozlesme/Cargo.lock sozlesme/src /tmp/sozlesme-kopya/ && cd /tmp/sozlesme-kopya && sed -i "s/^    \*p \/\/.*/    unsafe { *p }/" src/main.rs && grep -n "unsafe { \*p }" src/main.rs && cargo run -q 2>&1'
