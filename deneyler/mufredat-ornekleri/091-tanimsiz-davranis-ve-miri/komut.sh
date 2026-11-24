#@ video: 091-tanimsiz-davranis-ve-miri
#@ degisken: sınır dışı ve bırakılmış bellekten okunan değerler tanımsızdır, her derlemede/çalıştırmada değişebilir; Miri'nin ayırma numaraları (alloc…)
#@ goster: Undefined Behavior|error: |-->|^\s+= help
# Normal çalıştırma: "çalışıyor gibi" görünür
k cargo run -q --bin sinir_disi
k cargo run -q --release --bin birak_sonra_kullan
# Aynı program, iki derleme, iki farklı sonuç: tanımsız davranışlı programın anlamı yoktur
k cargo run -q --bin gecersiz_bool
k cargo run -q --release --bin gecersiz_bool
# İki ikilide de karşılaştırma komutu yok: hangi kolun çalışacağına derleme anında karar verilmiş
ks 'objdump -d -M intel --no-show-raw-insn -C target/debug/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/ret/"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/jmp/"'
# Seçilen metin: lea'nın gösterdiği adreste duran sabit (debug 9 bayt "yanlış\n", release 7 bayt "doğru\n")
ks 'a=$(objdump -d -M intel --no-show-raw-insn -C target/debug/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/ret/" | grep -oE "# [0-9a-f]+ <" | head -n 1 | grep -oE "[0-9a-f]+"); objdump -s -j .rodata --start-address=0x$a --stop-address=$((0x$a + 9)) target/debug/gecersiz_bool | tail -n 1'
ks 'a=$(objdump -d -M intel --no-show-raw-insn -C target/release/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/jmp/" | grep -oE "# [0-9a-f]+ <" | head -n 1 | grep -oE "[0-9a-f]+"); objdump -s -j .rodata --start-address=0x$a --stop-address=$((0x$a + 7)) target/release/gecersiz_bool | tail -n 1'
# Miri: programı soyut makine üzerinde yorumlayarak her bellek erişimini denetler
ks 'cargo +nightly miri run -q --bin sinir_disi 2>&1 | head -n 12'
ks 'cargo +nightly miri run -q --bin birak_sonra_kullan 2>&1 | head -n 12'
ks 'cargo +nightly miri run -q --bin gecersiz_bool 2>&1 | head -n 10'
# Kendin dene 1: p.add(3) yerine p.add(2) (etiket de v[2]); tanımlı okuma, Miri sessiz (çıkış kodu 0)
ks 'rm -rf /tmp/ub-kopya && mkdir /tmp/ub-kopya && cp -r Cargo.toml Cargo.lock src /tmp/ub-kopya/ && cd /tmp/ub-kopya && sed -i "s/p.add(3)/p.add(2)/; s/v\[3\] = /v[2] = /" src/bin/sinir_disi.rs && cargo run -q --bin sinir_disi && cargo +nightly miri run -q --bin sinir_disi && echo "Miri: tanımsız davranış yok"'
# Kendin dene 2: transmute(2u8) yerine transmute(1u8); iki derleme aynı sonucu verir
ks 'cd /tmp/ub-kopya && sed -i "s/transmute(2u8)/transmute(1u8)/" src/bin/gecersiz_bool.rs && cargo run -q --bin gecersiz_bool && cargo run -q --release --bin gecersiz_bool'
