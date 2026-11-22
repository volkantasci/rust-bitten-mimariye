#@ video: 061-clippy-rustfmt-ve-ci
k cargo run -q
# Derleyici sustu; clippy ne diyor?
k cargo clippy -q
# Sürekli entegrasyonda kapı: uyarı = hata
ks 'cargo clippy -q -- -D warnings 2>&1 | tail -n 2'
# Biçim denetimi: farkı göster, dosyaya dokunma
k cargo fmt --check
