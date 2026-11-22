#@ video: 117-ambar-tasarim
#@ ortak: ortak-projeler/ambar
cd ../ortak-projeler/ambar
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
k ./target/release/ambar demo calisma/veri
# Süreç bitti, bellek gitti. Yeni bir süreç günlüğü baştan okuyarak durumu yeniden kurar:
k ./target/release/ambar kurtar calisma/veri
# Günlük dosyasının baytları: 5 kayıt (SET, SET, üzerine yazma, silme, SET)
k od -A d -c calisma/veri/gunluk.ambar
rm -rf calisma
