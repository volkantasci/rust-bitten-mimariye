#@ video: 091-tanimsiz-davranis-ve-miri
#@ degisken: sınır dışı ve bırakılmış bellekten okunan değerler tanımsızdır, her derlemede/çalıştırmada değişebilir
#@ goster: Undefined Behavior|error: |-->|^\s+= help
# Normal çalıştırma: "çalışıyor gibi" görünür
k cargo run -q --bin sinir_disi
k cargo run -q --release --bin birak_sonra_kullan
k cargo run -q --release --bin gecersiz_bool
# miri: programı yorumlayarak her bellek erişimini denetler
ks 'cargo +nightly miri run -q --bin sinir_disi 2>&1 | head -n 12'
ks 'cargo +nightly miri run -q --bin birak_sonra_kullan 2>&1 | head -n 12'
ks 'cargo +nightly miri run -q --bin gecersiz_bool 2>&1 | head -n 10'
