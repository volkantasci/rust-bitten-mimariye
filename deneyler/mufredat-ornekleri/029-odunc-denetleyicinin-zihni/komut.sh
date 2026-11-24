#@ video: 029-odunc-denetleyicinin-zihni
#@ degisken: valgrind satırlarındaki süreç kimliği (==N==)
cd nll && k cargo run -q; cd ..
cd temkinli && k cargo build -q
# Yeni nesil denetleyici Polonius (yalnızca nightly'de, deneysel bayrakla)
ks 'rustc +nightly -Z help | grep -i polonius'
ks 'RUSTFLAGS="-Zpolonius=next" cargo +nightly run -q --target-dir target/polonius'
cd ..
# Ajan Masası (K6): ajanın ilk sürümü derlenmez; ajan her yere .clone() ekler; valgrind ayırma sayısını gösterir
cd ajan-masasi/ilk-surum && k cargo build -q; cd ../..
cd ajan-masasi/kod && k cargo build -q --release
ks 'valgrind ./target/release/ajan 2>&1 | grep -E "toplam uzunluk|total heap usage"'
ks 'valgrind ./target/release/duzeltilmis 2>&1 | grep -E "toplam uzunluk|total heap usage"'
cd ../..
