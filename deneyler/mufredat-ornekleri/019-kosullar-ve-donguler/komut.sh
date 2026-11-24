#@ video: 019-kosullar-ve-donguler
k cargo run -q --release
# Derleyicinin gördüğü kontrol akışı: adimlar fonksiyonunun MIR'ındaki temel bloklar ve atlamalar
mkdir -p target
k rustc --edition 2024 -C opt-level=2 --emit=mir -o target/collatz.mir src/main.rs
ks 'awk "/^fn adimlar/,/^}/" target/collatz.mir | grep -E "^    bb[0-9]+|switchInt|goto|return"'
