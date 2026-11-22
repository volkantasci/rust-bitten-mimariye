#@ video: 090-unsafe-ne-demek
cd ham-isaretci && k cargo build -q; cd ..
# Hangi işlemler unsafe gerektirir? Derleyicinin hata açıklaması listeler
ks 'rustc --explain E0133 | head -n 12'
cd sozlesme && k cargo run -q; cd ..
# 2024 sürümünün kuralları: no_mangle ve extern blokları da "unsafe" diye işaretlenmeli
cd surum-2024 && k cargo build -q; cd ..
