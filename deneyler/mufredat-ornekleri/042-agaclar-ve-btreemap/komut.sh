#@ video: 042-agaclar-ve-btreemap
#@ degisken: süreler
k cargo run -q --release
# Kaynak (nightly rust-src): dallanma sabiti B = 6, düğüm en çok 2B − 1 = 11 anahtar tutar
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^const B: usize\|const CAPACITY: usize" $S/alloc/src/collections/btree/node.rs'
# Düğümün içinde anahtarlar soldan sağa tek tek karşılaştırılır (doğrusal arama)
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "for (offset, k) in\|Ordering::Greater => {}" $S/alloc/src/collections/btree/search.rs'
# std belgesinin kendi açıklaması: geniş düğüm daha az ayırma ve daha iyi önbellek kullanımı, ama ortalamada daha çok karşılaştırma
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A2 "A B-Tree instead makes each node contain" $S/alloc/src/collections/btree/map.rs'
