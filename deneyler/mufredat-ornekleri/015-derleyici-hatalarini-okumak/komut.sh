#@ video: 015-derleyici-hatalarini-okumak
cd tip-hatasi && k cargo build -q; cd ..
ks 'rustc --explain E0308 | head -n 16'
cd mut-eksik && k cargo build -q; cd ..
