#@ video: 057-crateler-ve-bagimliliklar
#@ degisken: cargo update --dry-run çıktısı crates.io'da yayımlanan yeni sürümlere göre değişir (ağ gerekir)
k cat Cargo.toml
k cargo run -q
# "1" gereksinimi hangi sürüme çözüldü? Bağımlılık ağacı:
k cargo tree
# Geçişli bağımlılık: memchr'i projeye kim getirdi?
k cargo tree -e normal -i memchr
# Cargo.lock: tam sürüm + .crate dosyasının SHA-256 özeti (checksum alanı)
ks 'grep -A3 "name = \"serde_json\"" Cargo.lock'
# Özeti kendimiz hesaplayalım: cargo'nun önbelleğindeki .crate dosyasının SHA-256'sı
ks 'sha256sum /usr/local/cargo/registry/cache/*/serde_json-1.0.154.crate | cut -d" " -f1'
# Kilit dosyası olmasaydı bugün hangi sürümler seçilirdi? (--dry-run: dosyaya yazmaz)
k cargo update --dry-run
