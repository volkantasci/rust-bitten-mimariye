#@ video: 101-llvm-ve-optimizasyon
mkdir -p target
k rustc --edition 2024 --crate-type=lib -C opt-level=0 -C debug-assertions=off -C overflow-checks=off --emit=llvm-ir -o target/O0.ll dongu.rs
k rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/O3.ll dongu.rs
ks 'echo "O0 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O0.ll | wc -l)  O3 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll | wc -l)"'
# O3: döngü (br/phi) kalmadı mı? Hangi komutlar var?
ks 'awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll'
