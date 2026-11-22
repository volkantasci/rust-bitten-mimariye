#@ video: 094-bellek-esleme
#@ degisken: adres (ASLR) ve sayfa hatası sayısı (çekirdeğin bir hatada kaç sayfa eşlediğine bağlı)
k cargo run -q --release
ks 'strace -e trace=openat,mmap,munmap ./target/release/mmap 2>&1 | grep -E "esleme.bin|67108864"'
