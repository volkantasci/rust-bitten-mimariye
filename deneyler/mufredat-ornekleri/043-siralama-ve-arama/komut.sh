#@ video: 043-siralama-ve-arama
k cargo run -q --release
# Kaynak: std'nin sıralama algoritmaları
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "is based on \[driftsort\]" $S/alloc/src/slice.rs | head -1; grep -n "is based on \[ipnsort\]" $S/core/src/slice/mod.rs | head -1'
