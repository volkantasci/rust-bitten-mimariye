#@ video: 045-onbellek-dostu-veri-yapilari
#@ degisken: süreler donanıma ve yüke bağlı
k cargo run -q --release
# Kaynak (nightly rust-src): std'nin kendi notu, bağlı liste yerine dizi tabanlı yapıları önerir
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A2 "^//! NOTE: It is almost always better to use" $S/alloc/src/collections/linked_list.rs'
