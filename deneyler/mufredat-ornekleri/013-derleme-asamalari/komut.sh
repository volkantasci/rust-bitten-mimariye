#@ video: 013-derleme-asamalari
#@ degisken: derleme ve çalışma süreleri
#@ goster: Running `rustc
# cargo aslında rustc'yi çağırır: -v ile tam komutu görelim
k cargo build -v
mkdir -p target
# Aynı fonksiyonun LLVM ara temsili (IR): optimizasyonsuz ve optimizasyonlu
k rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=llvm-ir -o target/kare_O0.ll kare.rs
ks "awk '/^define.*@kare/,/^}/' target/kare_O0.ll"
k rustc --edition 2024 --crate-type=lib -C opt-level=2 --emit=llvm-ir -o target/kare_O2.ll kare.rs
ks "awk '/^define.*@kare/,/^}/' target/kare_O2.ll"
# debug ve release ikililerinin hızı
k cargo build -q --release
k hyperfine -N --warmup 1 --runs 5 --style basic ./target/debug/elek ./target/release/elek
