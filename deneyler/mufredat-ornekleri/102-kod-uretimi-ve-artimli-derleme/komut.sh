#@ video: 102-kod-uretimi-ve-artimli-derleme
#@ degisken: derleme süreleri; artımlı önbellekteki .o dosyalarının adları
rm -rf calisma && mkdir -p calisma && cd calisma && bash ../uret.sh && cd buyuk
k wc -l src/main.rs
ks '/usr/bin/time -f "temiz derleme (debug): %e s" cargo build -q'
# Artımlı önbellekte her kod üretim birimi (CGU) bir .o dosyası
ks 'cd target/debug/incremental/buyuk-*/s-*/ && md5sum *.o > /tmp/buyuk-once.txt && echo "kod üretim birimi (.o): $(ls *.o | wc -l)" && ls -S *.o | head -n 1 | xargs stat -c "en büyüğü: %s bayt"'
ks 'cd target/debug/incremental/buyuk-*/s-*/ && ls -l *.o | awk "{print \$5}" | sort -n | tr "\n" " " | sed "s/^/bütün birimler (bayt): /"; echo'
ks '/usr/bin/time -f "değişiklik yok: %e s" cargo build -q'
ks 'sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "tek satır değişti (artımlı): %e s" cargo build -q'
ks 'cd target/debug/incremental/buyuk-*/s-*/ && echo "yeniden üretilen .o: $(md5sum *.o | grep -vxFf /tmp/buyuk-once.txt | wc -l)" && md5sum *.o | grep -vxFf /tmp/buyuk-once.txt | awk "{print \$2}" | xargs stat -c "yeniden üretilenin boyutu: %s bayt"'
ks 'CARGO_INCREMENTAL=0 /usr/bin/time -f "aynı değişiklik, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz'
# Aynı 20000 fonksiyon, 100 modüle bölünmüş
cd ../moduler
ks 'grep -c "^mod " src/main.rs'
ks '/usr/bin/time -f "100 modül, temiz derleme (debug): %e s" cargo build -q'
ks 'cd target/debug/incremental/moduler-*/s-*/ && md5sum *.o > /tmp/moduler-once.txt && echo "kod üretim birimi (.o): $(ls *.o | wc -l)" && ls -S *.o | head -n 1 | xargs stat -c "en büyüğü: %s bayt"'
ks 'cd target/debug/incremental/moduler-*/s-*/ && ls -l *.o | awk "{print \$5}" | sort -n | awk "{a[NR]=\$1} END {printf \"en küçük 7 birim (bayt): \"; for (i = 1; i <= 7; i++) printf \"%d \", a[i]; printf \"\\nsonraki %d birim: %d – %d bayt\\nen büyük birim: %d bayt\\n\", NR - 8, a[8], a[NR - 1], a[NR]}"'
ks 'sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "100 modül, tek satır değişti (artımlı): %e s" cargo build -q'
ks 'cd target/debug/incremental/moduler-*/s-*/ && echo "yeniden üretilen .o: $(md5sum *.o | grep -vxFf /tmp/moduler-once.txt | wc -l)" && md5sum *.o | grep -vxFf /tmp/moduler-once.txt | awk "{print \$2}" | xargs stat -c "yeniden üretilenin boyutu: %s bayt"'
cd ../buyuk
# Kod üretim birimleri (codegen-units): paralel kod üretimi ↔ optimizasyon fırsatı
ks 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 /usr/bin/time -f "release, codegen-units=16: %e s" cargo build -q --release --target-dir target/cgu16'
ks 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 /usr/bin/time -f "release, codegen-units=1 : %e s" cargo build -q --release --target-dir target/cgu1'
ks './target/cgu16/release/buyuk; ./target/cgu1/release/buyuk'
# Zaman nereye gidiyor? Artımlı yeniden derlemede derleyici aşamaları (nightly, -Ztime-passes), en uzun 8 aşama
k cargo +nightly build -q --target-dir target/nightly
ks 'sed -i "s/(x ^ 15001)/(x ^ 15002)/" src/main.rs && cargo +nightly rustc -q --target-dir target/nightly -- -Ztime-passes 2>&1 | grep -E "time: +[0-9.]+;" | sort -k2 -n -r | head -n 8'
