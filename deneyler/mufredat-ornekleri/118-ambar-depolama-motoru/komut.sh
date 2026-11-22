#@ video: 118-ambar-depolama-motoru
#@ ortak: ortak-projeler/ambar
cd ../ortak-projeler/ambar
ks 'cargo test -q --lib 2>&1 | grep "test result"'
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
k ./target/release/ambar doldur calisma/veri 1000
ks 'stat -c "%s bayt" calisma/veri/gunluk.ambar'
# 1) Yarım yazma: son kaydın son 5 baytı diske hiç ulaşmamış gibi
k truncate -s -5 calisma/veri/gunluk.ambar
k ./target/release/ambar kurtar calisma/veri
# 2) Sessiz bozulma, kaydın verisinde: tek bir bayt değişsin
rm -rf calisma/veri && ./target/release/ambar doldur calisma/veri 1000 > /dev/null
ks 'printf "X" | dd of=calisma/veri/gunluk.ambar bs=1 seek=16910 conv=notrunc status=none'
k ./target/release/ambar kurtar calisma/veri
# 3) Sessiz bozulma, kaydın başlığında (uzunluk alanı): kayıt sınırı kaybolur
rm -rf calisma/veri && ./target/release/ambar doldur calisma/veri 1000 > /dev/null
ks 'printf "X" | dd of=calisma/veri/gunluk.ambar bs=1 seek=16900 conv=notrunc status=none'
k ./target/release/ambar kurtar calisma/veri
rm -rf calisma
