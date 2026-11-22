#@ video: 126-yorumlayici-parser
#@ ortak: ortak-projeler/minik
cd ../ortak-projeler/minik
k cargo build -q --release
# Öncelik ağacın biçiminde: 2 + (3 * (4 - 1))
k ./target/release/minik ast ornekler/ifade.mk
k cat ornekler/fib.mk
k ./target/release/minik ast ornekler/fib.mk
k ./target/release/minik ast ornekler/hatali.mk
