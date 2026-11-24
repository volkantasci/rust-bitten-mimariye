#@ video: 026-odunc-alma
#@ degisken: adresler her çalıştırmada değişir (ASLR); bloğun yerinde büyüyüp büyümediği ayırıcıya bağlıdır; miri alloc numaraları
# Makine düzeyi: Vec dolunca ayırıcı bloğu bazen yerinde büyütür, bazen taşır
cd neden && k cargo run -q; cd ..
# unsafe Rust: eski adresi tutan ham işaretçi; bu çalıştırmada "çalışıyor" görünebilir, miri kuralın çiğnendiğini gösterir
cd sarkan-isaretci
k cargo run -q
k cargo +nightly miri run -q
cd ..
# Güvenli Rust: bir elemana referans yaşarken push reddedilir (E0502)
cd kural && k cargo build -q; cd ..
