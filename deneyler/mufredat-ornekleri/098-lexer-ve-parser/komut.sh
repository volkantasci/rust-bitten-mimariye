#@ video: 098-lexer-ve-parser
# Sözcüksel çözümleme: kaynak metin belirteçlere (token) bölünür; parantezli grup tek bir belirteç ağacıdır
ks 'rustc --edition 2024 -o /tmp/belirtecler belirtecler.rs && /tmp/belirtecler'
# Ayrıştırıcının ürettiği soyut söz dizimi ağacı (AST): öncelik ağacın biçiminde
ks 'rustc +nightly --edition 2024 -Zunpretty=ast-tree ifade.rs | grep -E "kind: (Binary|Lit)|Add|Mul|symbol" '
# Kendin dene: birleşme özelliği. 10 - 4 - 3 soldan birleşir: (10 - 4) - 3
ks 'rustc +nightly --edition 2024 -Zunpretty=ast-tree birlesme.rs | grep -E "kind: (Binary|Lit)|Sub|symbol" '
# Söz dizimi hatası: ayrıştırıcı nerede takıldığını söyler
ks 'echo "fn main() { let x = (1 + 2; }" > /tmp/bozuk.rs && rustc --edition 2024 /tmp/bozuk.rs'
