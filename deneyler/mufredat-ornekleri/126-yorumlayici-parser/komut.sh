#@ video: 126-yorumlayici-parser
# Kontrol noktası: bu videonun sonundaki kod minik/ dizininde (125'e göre farkı: diff -ru).
cd minik
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
# Öncelik ağacın biçiminde: 2 + (3 * (4 - 1))
k ./target/release/minik ast ornekler/ifade.mk
# Pratt ayrıştırmanın adımları: her girinti bir ifade(en_az) çağrısı
k cat ornekler/oncelik.mk
k ./target/release/minik iz ornekler/oncelik.mk
k cat ornekler/fib.mk
k ./target/release/minik ast ornekler/fib.mk
k ./target/release/minik ast ornekler/hatali.mk
