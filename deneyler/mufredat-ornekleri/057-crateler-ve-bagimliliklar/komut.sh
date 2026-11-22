#@ video: 057-crateler-ve-bagimliliklar
k cat Cargo.toml
k cargo run -q
# "1" gereksinimi hangi sürüme çözüldü? Bağımlılık ağacı:
k cargo tree
# Cargo.lock: tam sürüm + içerik özeti (checksum)
ks 'grep -A3 "name = \"serde_json\"" Cargo.lock'
