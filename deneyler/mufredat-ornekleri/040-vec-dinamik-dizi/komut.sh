#@ video: 040-vec-dinamik-dizi
#@ degisken: "(taşındı)" / "(yerinde büyüdü)" işaretleri ve "gerçekte taşınan" sayısı ayırıcıya ve süreçteki öteki ayırmalara bağlıdır
k cargo run -q
# Kaynak (nightly rust-src): büyütme kapasiteyi ikiye katlar; ilk ayırmanın en küçük kapasitesi eleman boyutuna bağlı
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "let cap = cmp::max(self.cap.as_inner() \* 2, required_cap)" $S/alloc/src/raw_vec/mod.rs; grep -n -A8 "^const fn min_non_zero_cap" $S/alloc/src/raw_vec/mod.rs'
