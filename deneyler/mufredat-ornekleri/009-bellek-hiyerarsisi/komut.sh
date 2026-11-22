#@ video: 009-bellek-hiyerarsisi
#@ degisken: süreler donanıma ve o anki yüke bağlı (AMD Ryzen 5 5600: L1d 32 KiB, L2 512 KiB/çekirdek, L3 32 MiB)
ks 'lscpu | grep -E "^L1d|^L2|^L3"'
k cargo run -q --release
