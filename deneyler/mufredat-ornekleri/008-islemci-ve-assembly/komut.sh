#@ video: 008-islemci-ve-assembly
mkdir -p target
# Optimizasyonlu derleme: tek komut
k rustc --edition 2024 -C opt-level=3 --crate-type=lib --emit=obj -o target/islem.o islem.rs
k objdump -d -M intel --no-show-raw-insn --disassemble=topla target/islem.o
# Döngü mü? Derleyici formüle çevirdi mi?
k objdump -d -M intel --no-show-raw-insn --disassemble=toplam target/islem.o
# Optimizasyonsuz (debug) derleme: taşma denetimi görünür
k rustc --edition 2024 -C opt-level=0 --crate-type=lib --emit=obj -o target/islem_debug.o islem.rs
k objdump -d -M intel --no-show-raw-insn -r -C --disassemble=topla target/islem_debug.o
