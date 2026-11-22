#@ video: 100-mir-ve-odunc-denetimi
mkdir -p target
# MIR: temel bloklardan (bb0, bb1...) oluşan kontrol akış grafiği; ödünç denetimi burada yapılır
k rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=mir -o target/mir.mir mir.rs
ks 'sed -n "/fn uzunluk/,/^}/p" target/mir.mir'
