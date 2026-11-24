#@ video: 092-guvenli-soyutlama-yazmak
#@ degisken: Miri'nin etiket (<…>) ve ayırma (alloc…) numaraları
#@ goster: Undefined Behavior|error: |test result|---- |hatali_surum|dogru_surum
# Normal testler: ikisi de "geçer"; hatalı sürümün tanımsız davranışı görünmez
k cargo test -q
# Miri altında: doğru sürüm temiz
ks 'cargo +nightly miri test -q dogru_surum 2>&1 | grep -E "test result|running"'
# Hatalı sürüm, varsayılan ödünç modeli (Stacked Borrows): çakışan &mut daha oluşurken yakalanır
ks 'cargo +nightly miri test -q hatali_surum 2>&1 | sed -n "/^error: Undefined Behavior/,/this is on thread/p"'
# Kendin dene: Tree Borrows modeliyle aynı test; ihlal ilk çakışan yazmada yakalanır
ks 'MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri test -q hatali_surum 2>&1 | sed -n "/^error: Undefined Behavior/,/this is on thread/p"'
# Kendin dene: hatalı sürümün testinden iki yazmayı ve assert_eq! satırını silmek; yalnız dilimler oluşturuluyor
ks 'rm -rf /tmp/bolme-kopya && mkdir /tmp/bolme-kopya && cp -r Cargo.toml Cargo.lock src /tmp/bolme-kopya/ && cd /tmp/bolme-kopya && sed -i "/a\[1\] = 20;/d; /b\[0\] = 99;/d; /assert_eq!(v\[1\], 99);/d" src/lib.rs && cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "^error: Undefined Behavior|^test result"'
ks 'cd /tmp/bolme-kopya && MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "^error: Undefined Behavior|^test result"'
