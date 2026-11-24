#@ video: 117-ambar-tasarim
#@ degisken: panik satırındaki iş parçacığı kimliği (parantez içindeki sayı) her çalıştırmada değişir
# Kontrol noktası: bu videonun sonundaki kod ambar/ dizininde.
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep "^test "'
k cargo build -q --release
rm -rf calisma
# Her komut ayrı bir süreç: açılışta günlüğü baştan okuyup dizini yeniden kurar.
k ./target/release/ambar yaz calisma dil rust
k ./target/release/ambar yaz calisma yil 2015
k ./target/release/ambar yaz calisma dil Rust
k ./target/release/ambar sil calisma yil
k ./target/release/ambar yaz calisma şehir İzmir
k ./target/release/ambar listele calisma
k ./target/release/ambar oku calisma yil
# Günlüğün baytları (onaltılık; sağda ASCII)
k od -A d -t x1z -v calisma/gunluk.ambar
# Aynı beş komut bir kez daha: canlı durum aynı kalır, günlük büyür
echo "# aynı beş komut bir kez daha çalıştırıldı"
for satir in "yaz dil rust" "yaz yil 2015" "yaz dil Rust" "sil yil" "yaz şehir İzmir"; do
  set -- $satir
  ./target/release/ambar "$1" calisma "${@:2}"
done
k ./target/release/ambar listele calisma
# Bu adımın okuyucusu günlüğün sağlam olduğunu varsayar: son kaydın 3 baytı eksik olursa?
k truncate -s -3 calisma/gunluk.ambar
k ./target/release/ambar listele calisma
rm -rf calisma
