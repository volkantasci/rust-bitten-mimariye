#@ video: 042-agaclar-ve-btreemap
#@ degisken: süreler
k cargo run -q --release
# Kaynak: B-ağacının dallanma sabiti
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^const B: usize\|^pub const CAPACITY" $S/alloc/src/collections/btree/node.rs'
