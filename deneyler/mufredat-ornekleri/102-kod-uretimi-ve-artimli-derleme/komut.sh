#@ video: 102-kod-uretimi-ve-artimli-derleme
#@ degisken: derleme süreleri
rm -rf calisma && mkdir -p calisma && cd calisma && bash ../uret.sh && cd buyuk
k wc -l src/main.rs
ks '/usr/bin/time -f "temiz derleme (debug): %e s" cargo build -q'
ks '/usr/bin/time -f "değişiklik yok: %e s" cargo build -q'
ks 'sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "tek satır değişti (artımlı): %e s" cargo build -q'
ks 'CARGO_INCREMENTAL=0 /usr/bin/time -f "aynı değişiklik, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz'
ks 'sed -i "s/(x ^ 15001)/(x ^ 15002)/" src/main.rs && CARGO_INCREMENTAL=0 /usr/bin/time -f "tek satır değişti, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz'
# Kod üretim birimleri (codegen-units): paralel kod üretimi ↔ optimizasyon fırsatı
ks 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 /usr/bin/time -f "release, codegen-units=16: %e s" cargo build -q --release --target-dir target/cgu16'
ks 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 /usr/bin/time -f "release, codegen-units=1 : %e s" cargo build -q --release --target-dir target/cgu1'
ks './target/cgu16/release/buyuk; ./target/cgu1/release/buyuk'
# Zaman nereye gidiyor? Artımlı yeniden derlemede derleyici aşamaları (nightly, -Ztime-passes), en uzun 8 aşama
k cargo +nightly build -q --target-dir target/nightly
ks 'sed -i "s/(x ^ 15002)/(x ^ 15003)/" src/main.rs && cargo +nightly rustc -q --target-dir target/nightly -- -Ztime-passes 2>&1 | grep -E "time: +[0-9.]+;" | sort -k2 -n -r | head -n 8'
