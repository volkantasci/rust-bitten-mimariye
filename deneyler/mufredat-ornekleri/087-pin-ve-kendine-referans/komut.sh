#@ video: 087-pin-ve-kendine-referans
#@ degisken: adresler (ASLR); miri'nin bellek bloğu numarası
cd tasima-sorunu
k cargo run -q
# Eski yeri okumak: miri tanımsız davranışı yakalar
ks 'cargo +nightly miri run -q -- oku 2>&1 | head -n 12'
# Kendin dene: aynı program miri'de oku kipi olmadan (işaretçi yalnız yazdırılır, okunmaz)
ks 'cargo +nightly miri run -q 2>&1 | head -n 6'
cd ..
cd sabitlenmis && k cargo build -q; cd ..
