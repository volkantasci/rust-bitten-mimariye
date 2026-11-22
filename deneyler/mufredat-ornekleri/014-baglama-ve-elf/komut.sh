#@ video: 014-baglama-ve-elf
#@ degisken: ldd'nin yükleme adresleri (ASLR)
k cargo build -q --release
k ldd target/release/merhaba
ks 'readelf -h target/release/merhaba | grep -E "Class|Type|Machine|Entry"'
ks 'readelf -S -W target/release/merhaba | grep -E " \.(text|rodata|data|bss) "'
# Sembol adları: derleyici "isim karıştırma" (mangling) uygular
ks 'nm target/release/merhaba | grep "merhaba4main"'
ks 'nm -C target/release/merhaba | grep "merhaba::main"'
# Statik bağlama: C kütüphanesi de ikiliye gömülsün
ks 'RUSTFLAGS="-C target-feature=+crt-static" cargo build -q --release --target-dir target/statik'
k file target/statik/release/merhaba
k ldd target/statik/release/merhaba
ks 'stat -c "%s bayt  %n" target/release/merhaba target/statik/release/merhaba'
k ./target/statik/release/merhaba
