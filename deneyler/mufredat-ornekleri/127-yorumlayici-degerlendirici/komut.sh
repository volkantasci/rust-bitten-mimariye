#@ video: 127-yorumlayici-degerlendirici
#@ ortak: ortak-projeler/minik
cd ../ortak-projeler/minik
k cargo build -q --release
k ./target/release/minik calistir ornekler/fib.mk
k cat ornekler/sifira_bolme.mk
k ./target/release/minik calistir ornekler/sifira_bolme.mk
k ./target/release/minik calistir ornekler/sonsuz.mk
