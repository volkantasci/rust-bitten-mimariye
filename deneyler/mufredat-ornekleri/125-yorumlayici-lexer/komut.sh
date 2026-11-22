#@ video: 125-yorumlayici-lexer
#@ ortak: ortak-projeler/minik
cd ../ortak-projeler/minik
k cargo build -q --release
k cat ornekler/ifade.mk
k ./target/release/minik token ornekler/ifade.mk
k ./target/release/minik token ornekler/garip_karakter.mk
