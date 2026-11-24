#@ video: 039-karmasiklik-analizi
k cargo run -q --release
# Kaynak (nightly rust-src): std'nin ikili araması eşitlikte erken çıkmaz; döngü sayısı yalnız uzunluğa bağlıdır
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "intentionally doesn.t have an early exit\|on the size of the input slice" $S/core/src/slice/mod.rs | head -2'
