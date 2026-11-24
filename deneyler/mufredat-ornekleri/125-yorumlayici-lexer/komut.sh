#@ video: 125-yorumlayici-lexer
# Kontrol noktası: bu videonun sonundaki kod minik/ dizininde.
cd minik
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
k cat ornekler/ifade.mk
k ./target/release/minik token ornekler/ifade.mk
# En uzun eşleşme, Türkçe harfli ad, yorum ve karakter sayan sütunlar
k cat ornekler/esitlik.mk
k ./target/release/minik token ornekler/esitlik.mk
k ./target/release/minik token ornekler/garip_karakter.mk
