#@ video: 108-tip-gudumlu-tasarim
cd durum-tipleri && k cargo run -q; cd ..
cd yanlis-gecis && ks 'cargo build -q 2>&1 | head -n 16'; cd ..
