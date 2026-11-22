#@ video: 098-lexer-ve-parser
# Ayrıştırıcının ürettiği soyut sözdizimi ağacı (AST): öncelik ağacın biçiminde
ks 'rustc +nightly --edition 2024 -Zunpretty=ast-tree ifade.rs | grep -E "kind: (Binary|Lit)|Add|Mul|symbol" '
# Sözdizimi hatası: ayrıştırıcı nerede takıldığını söyler
ks 'echo "fn main() { let x = (1 + 2; }" > /tmp/bozuk.rs && rustc --edition 2024 /tmp/bozuk.rs'
