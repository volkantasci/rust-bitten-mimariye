#@ video: 058-tedarik-zinciri-guvenligi
#@ degisken: cargo search sürümleri crates.io'ya göre değişir (ağ gerekir); build.rs'nin gördüğü girdi sayısı ortama bağlıdır
# 1) Aynı crate, farklı özellik (feature) seçimi: kaç paket derlemeye girer?
cd tam
k grep tokio Cargo.toml
k cargo fetch -q
# Farklı paket sayısı (kendi paketimiz dahil); --no-dedupe tekrar edenleri "(*)" ile kısaltmadan yazar
ks 'cargo tree --prefix none -e normal --no-dedupe | sort -u | wc -l'
k cargo tree --depth 1 -e normal
# Bunların bir kısmı prosedürel makro ve onun bağımlılıkları: ikiliye girmez, derleme anında çalışır
k cargo tree -e normal -i proc-macro2
cd ../asgari
k grep tokio Cargo.toml
k cargo fetch -q
ks 'cargo tree --prefix none -e normal --no-dedupe | sort -u | wc -l'
k cargo tree -e normal
cd ..
# 2) Bir bağımlılığın kodu derleme anında, bizim yetkilerimizle çalışabilir: derleme betiği (build.rs)
cd derleme-betigi
cargo clean -q 2>/dev/null
ks 'cargo build 2>&1 | grep -v "^ *Finished"'
k ./target/debug/uygulama
cd ..
# 3) Denetim araçları crates.io'da yayımlanıyor (bu imajda kurulu değil)
ks 'for a in cargo-audit cargo-deny cargo-vet; do cargo search --limit 1 $a | head -n 1; done'
