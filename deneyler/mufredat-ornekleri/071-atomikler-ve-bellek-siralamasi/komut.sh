#@ video: 071-atomikler-ve-bellek-siralamasi
#@ degisken: Relaxed sayımı her çalıştırmada değişir (SeqCst her zaman 0 olmalı)
k cargo build -q --release
k ./target/release/siralama_testi
# Makine kodundaki fark: SeqCst yazma xchg (tam bariyer) olarak derleniyor
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_oku_gevsek>:"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A5 "<siralama_testi::yaz_oku_sirali>:"'
