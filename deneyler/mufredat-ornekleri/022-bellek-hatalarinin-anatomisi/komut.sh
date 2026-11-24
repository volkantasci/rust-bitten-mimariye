#@ video: 022-bellek-hatalarinin-anatomisi
#@ degisken: AddressSanitizer adresleri ve süreç kimlikleri; panik satırındaki iş parçacığı kimliği; miri alloc numaraları; ayırma dışı okumada kullanılmayan baytların içeriği (bu kayıtta sıfır)
#@ goster: ERROR: AddressSanitizer|SUMMARY|freed by thread|previously allocated|READ of size|is located|asan::main|Shadow byte legend|Freed heap region|Heap left redzone|Addressable|=>|error\[E0382\]|move occurs because|value moved here|value used here after move|^ilk bayt|double free|SIGABRT|dangling box|beyond the end
# 1) Sınır dışı erişim. Güvenli Rust: her indeks denetlenir, dışarısı tanımlı bir paniktir
cd sinir && k cargo run -q; cd ..
# unsafe Rust: 8 baytlık bir öbek bloğunun ötesini okumak (tanımsız davranış); makinede komşu blok görünür
cd sinir-disi
k cargo run -q
# miri iki deneysel ödünç modelinde de aynı hatayı verir: erişim ayırmanın sonunun ötesinde
k cargo +nightly miri run -q
ks 'MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri run -q'
cd ..
# 2) Çifte iade. unsafe Rust: glibc'nin ayırıcısı bu durumda ikinci iadeyi fark eder ve süreci durdurur
cd cifte-iade
k cargo run -q
k cargo +nightly miri run -q
cd ..
# Güvenli Rust: değer taşındıktan sonra bir daha kullanılamaz (E0382)
cd cifte-iade-guvenli && k cargo build -q; cd ..
# 3) İade sonrası kullanım ve gölge bellek: AddressSanitizer (nightly)
cd asan
k cargo run -q
ks 'ASAN_SYMBOLIZER_PATH=/usr/bin/addr2line RUSTFLAGS="-Zsanitizer=address" cargo +nightly run -q --target x86_64-unknown-linux-gnu 2>&1'
cd ..
