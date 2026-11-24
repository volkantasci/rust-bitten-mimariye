#@ video: 051-standart-traitler-ve-donusumler
#@ degisken: 1000 kümenin kaçının 1 eleman tuttuğu (rastgele tohum; beklenen ≈ 1/128); panik satırındaki iş parçacığı kimliği
k cargo run -q
# Kaynak (nightly rust-src): Hash ile Eq birlikte uygulanınca sağlanması gereken özellik
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -B3 "^/// k1 == k2 -> hash(k1) == hash(k2)" $S/core/src/hash/mod.rs'
# Kaynak: bu özelliği bozmak bir "mantık hatası"dır, sonucu belirtilmemiştir
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A1 "^/// Violating this property is a logic error" $S/core/src/hash/mod.rs'
