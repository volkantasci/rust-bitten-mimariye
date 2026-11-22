#@ video: 041-hash-tablolari
#@ degisken: HashMap sırası ve hash değeri her süreçte değişir (rastgele tohum); BTreeMap sırası sabittir
k cargo build -q
k ./target/debug/hash
k ./target/debug/hash
# Kaynak: std'nin HashMap'i hashbrown (SwissTable) üzerine kurulu
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "use hashbrown\|Rust port of Google" $S/std/src/collections/hash/map.rs'
