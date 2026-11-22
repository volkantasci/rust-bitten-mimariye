#@ video: 022-bellek-hatalarinin-anatomisi
#@ degisken: AddressSanitizer adresleri ve süreç kimlikleri
#@ goster: ERROR: AddressSanitizer|SUMMARY|freed by thread|previously allocated|READ of size
mkdir -p target
# 1) Tampon taşması: yığın koruyucusu (canary) yakalar ve süreci durdurur
k gcc -O0 -fstack-protector-strong -o target/tasma c/tasma.c
k ./target/tasma Ferris
k ./target/tasma "bu metin sekiz bayttan cok daha uzun"
# 2) Çift iade: glibc'nin ayırıcısı fark eder
k gcc -O0 -o target/cift_birak c/cift_birak.c
k ./target/cift_birak
# 3) İade-sonrası-kullanım: AddressSanitizer ile derleyince yakalanır
k gcc -O0 -g -fsanitize=address -o target/uaf_asan c/kullan_sonra_birak.c
ks './target/uaf_asan 2>&1 | head -n 30'
# 4) Rust: dizinin dışına erişim tanımlı bir panikle durur
cd rust && k cargo run -q
