#@ video: 118-ambar-depolama-motoru
# Kontrol noktası: bu videonun sonundaki kod ambar/ dizininde (117'ye göre farkı: diff -ru).
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep "^test "'
k cargo build -q --release
# CRC-32'nin yayımlanmış denetim değeri ve bağımsız bir araçla (GNU cksum) karşılaştırma
k ./target/release/ambar crc 123456789
ks 'printf 123456789 | cksum -a crc32b'
k ./target/release/ambar crc 123456788
rm -rf calisma
k ./target/release/ambar doldur calisma 1000
ks 'stat -c "%s bayt" calisma/gunluk.ambar'
# 501. kaydın başlığı (18 890. bayttan 16 bayt): başlık CRC'si, anahtar uzunluğu (12), değer uzunluğu (10), veri CRC'si
k od -A d -t x1 -j 18890 -N 16 calisma/gunluk.ambar
# 1) Yarım yazma (torn write): son kaydın son 5 baytı diske hiç ulaşmamış gibi
k truncate -s -5 calisma/gunluk.ambar
k ./target/release/ambar durum calisma
ks 'stat -c "%s bayt" calisma/gunluk.ambar'
# 2) Sessiz bozulma: 501. kaydın anahtarında tek bir bayt değişsin
ks 'rm -rf calisma && ./target/release/ambar doldur calisma 1000'
ks 'printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18914 conv=notrunc status=none'
k ./target/release/ambar durum calisma
ks 'stat -c "%s bayt" calisma/gunluk.ambar'
# 3) Aynı bozulma 501. kaydın değer uzunluğu alanında: önce ikinci, sonra üçüncü baytı
ks 'rm -rf calisma && ./target/release/ambar doldur calisma 1000'
ks 'printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18899 conv=notrunc status=none'
k od -A d -t x1 -j 18890 -N 16 calisma/gunluk.ambar
k ./target/release/ambar durum calisma
ks 'rm -rf calisma && ./target/release/ambar doldur calisma 1000'
ks 'printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18900 conv=notrunc status=none'
k ./target/release/ambar durum calisma
ks 'stat -c "%s bayt" calisma/gunluk.ambar'
# Bozuk kayıttan itibaren her şeyi atmak operatörün açık kararı olmalı:
k ./target/release/ambar kirp calisma
k ./target/release/ambar durum calisma
rm -rf calisma
