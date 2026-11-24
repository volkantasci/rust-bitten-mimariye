#@ video: 041-hash-tablolari
#@ degisken: HashMap sırası ve hash değeri her süreçte değişir (rastgele tohum); BTreeMap ve sabit tohumlu HashMap sırası sabittir
k cargo build -q
k ./target/debug/hash
k ./target/debug/hash
# Kaynak (nightly rust-src): HashMap hashbrown (SwissTable) üzerine kurulu; varsayılan algoritma SipHash 1-3, rastgele tohumlu
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "use hashbrown\|Rust port of Google\|SipHash 1-3\|HashDoS attacks. The" $S/std/src/collections/hash/map.rs'
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "k0.wrapping_add(1)\|SipHasher13::new_with_keys(self" $S/std/src/hash/random.rs'
