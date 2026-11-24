#@ video: 100-mir-ve-odunc-denetimi
#@ goster: .
mkdir -p target
# MIR: temel bloklardan (bb0, bb1...) oluşan kontrol akış grafı
k rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=mir -o target/mir.mir mir.rs
ks 'sed -n "/fn uzunluk/,/^}/p" target/mir.mir'
# Ödünç denetimi akışa duyarlı: üç fonksiyondan yalnız sira_yanlis reddedilir
ks 'rustc --edition 2024 --crate-type=lib --out-dir target odunc.rs 2>&1'
# Görsel için: yola_gore'nin MIR'i. Reddedilen kodun MIR'i üretilmez; sira_yanlis çıkarılmış bir kopyadan
ks 'sed "/^pub fn sira_yanlis/,/^}/d" odunc.rs > target/odunc_mir.rs && rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=mir -o target/odunc.mir target/odunc_mir.rs && sed -n "/fn yola_gore/,/^}/p" target/odunc.mir'
