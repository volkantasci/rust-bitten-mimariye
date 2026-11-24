#@ video: 034-oruntu-esleme
cd eksik-kol && k cargo build -q; cd ..
cd dilim-oruntuleri && k cargo run -q; cd ..
# Karar ağacı: derleyici dilim örüntülerini uzunluk üzerinde dallanmaya çevirir (MIR'daki switchInt)
mkdir -p target
k rustc --edition 2024 -C opt-level=0 --emit=mir -o target/dilim.mir dilim-oruntuleri/src/main.rs
ks 'awk "/^fn siniflandir/,/^}/" target/dilim.mir | grep -E "switchInt|Ge\(|Eq\(|PtrMetadata|Len\("'
