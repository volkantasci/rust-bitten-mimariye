#@ video: 085-future-durum-makinesi
k cargo build -q
k ./target/debug/durum_makinesi
# Derleyicinin ürettiği durum makinesi: varyantlar ve her varyantta saklanan yerel değişkenler (nightly 1.102.0 -Zprint-type-sizes; artımlı derleme kapalı ki tablo her seferinde yazılsın)
ks 'touch src/main.rs && CARGO_INCREMENTAL=0 cargo +nightly rustc -q -- -Zprint-type-sizes > target/tip-boyutlari.txt 2>&1; grep -A7 "async fn body of tampon_await_boyunca()}" target/tip-boyutlari.txt'
ks 'grep -A8 "async fn body of tampon_await_oncesi()}" target/tip-boyutlari.txt'
# Kendin dene: ikinci bir await (yeni varyant Suspend1) ve 64 baytlık tampon
k ./target/debug/durum_makinesi kendin-dene
ks 'grep -A13 "async fn body of tampon_iki_await()}" target/tip-boyutlari.txt'
