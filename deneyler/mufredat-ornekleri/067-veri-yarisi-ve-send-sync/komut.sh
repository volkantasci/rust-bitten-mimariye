#@ video: 067-veri-yarisi-ve-send-sync
#@ degisken: C yarışının sonucu her çalıştırmada farklıdır
mkdir -p target
k gcc -O2 -pthread -o target/yaris c/yaris.c
k ./target/yaris
k ./target/yaris
# ThreadSanitizer yarışı adıyla bildirir
k gcc -O1 -g -fsanitize=thread -pthread -o target/yaris_tsan c/yaris.c
ks './target/yaris_tsan 2>&1 | grep -E "WARNING|Write of size|Previous write|SUMMARY" | head -n 6'
# Rust: aynı paylaşım denemesi derlenmez
cd rc-gonderilemez && k cargo build -q; cd ..
cd atomik && k cargo run -q --release; cd ..
