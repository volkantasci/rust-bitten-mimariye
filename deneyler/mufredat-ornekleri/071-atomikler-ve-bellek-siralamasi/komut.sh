#@ video: 071-atomikler-ve-bellek-siralamasi
#@ degisken: Relaxed, Release/Acquire ve fence(AcqRel) sayımları her çalıştırmada ve makinede değişir (SeqCst ve fence(SeqCst) sürümlerinde 0 olmalı); fonksiyon adresleri
k cargo build -q --release
k ./target/release/siralama_testi
# Makine kodundaki fark: yalnız yazma komutu ve bariyer değişiyor
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_oku_gevsek>:"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_oku_edin_birak>:"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_oku_sirali>:"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_bariyer_oku>:"'
# Yalnız 2 mantıksal işlemcide: program (bekleme döngüsündeki yield sayesinde) biter, ama iki işçi ve ana iş parçacığı 2 işlemciyi paylaştığı için işçiler nadiren aynı anda koşar
k taskset -c 0,1 ./target/release/siralama_testi gevsek
# Kendin dene: SeqCst yerine daha zayıf bir bariyer (AcqRel)
k ./target/release/siralama_testi zayif-bariyer
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_zayif_bariyer_oku>:"'
