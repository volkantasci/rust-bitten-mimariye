#@ video: 058-tedarik-zinciri-guvenligi
# Aynı crate, farklı özellik (feature) seçimi: kaç paket koda girer?
cd tam
k grep tokio Cargo.toml
k cargo fetch -q
ks 'cargo tree --prefix none -e normal | sort -u | wc -l'
k cargo tree --depth 1 -e normal
cd ../asgari
k grep tokio Cargo.toml
k cargo fetch -q
k cargo tree -e normal
cd ..
# Denetim araçları crates.io'da yayımlanıyor (bu imajda kurulu değil)
ks 'for a in cargo-audit cargo-deny cargo-vet; do cargo search --limit 1 $a | head -n 1; done'
