#@ video: 128-yorumlayici-bytecode-vm
#@ ortak: ortak-projeler/minik
#@ degisken: süreler ve hızlanma oranı
cd ../ortak-projeler/minik
k cargo build -q --release
k cat ornekler/kare.mk
k ./target/release/minik listele ornekler/kare.mk
k ./target/release/minik vm ornekler/kare.mk
# İki motor aynı programda aynı çıktıyı veriyor mu?
ks 'diff <(./target/release/minik calistir ornekler/fib.mk) <(./target/release/minik vm ornekler/fib.mk) && echo "fib.mk: iki motorun çıktısı aynı"'
k cat ornekler/kiyas.mk
k ./target/release/minik kiyasla ornekler/kiyas.mk
