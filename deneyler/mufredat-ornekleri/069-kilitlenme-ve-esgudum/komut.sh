#@ video: 069-kilitlenme-ve-esgudum
#@ degisken: iki iş parçacığının yazdırma sırası; döngülü sürümlerde kaç çalıştırmanın kilitlendiği (makineye ve yüke bağlı)
cd kilitlenme
k cargo build -q
# 1) Ters sıralı kilitler: sonsuza dek bekler; 3 saniye sonra timeout öldürür (124)
k timeout 3 ./target/debug/kilitlenme
# 2) Kilitlenmiş süreç ne yapıyor? Her iş parçacığının çekirdekte beklediği yer (wchan), durumu ve işlemci süresi (utime, stime; saat tıkı)
ks './target/debug/kilitlenme > /dev/null & P=$!; sleep 1; for t in /proc/$P/task/*; do echo "bekliyor: $(cat $t/wchan)  durum, utime, stime: $(cut -d" " -f3,14,15 $t/stat)"; done; kill $P'
# 3) Tek bir kilit sırası: kilitlenme yok
k timeout 3 ./target/debug/kilitlenme sirali
# 4) Beklemesiz döngü, her iş parçacığı 100, 300 ve 1000 transfer: kilitlenme yalnız bazen olur (her biri 20 çalıştırma; 0 = bitti, 124 = kilitlendi)
ks 'for n in 100 300 1000; do echo "$n transfer:"; for i in $(seq 20); do timeout 2 ./target/debug/kilitlenme dongu $n > /dev/null; echo "  çıkış kodu $?"; done | sort | uniq -c; done'
# Kendin dene: aynı döngü, tek kilit sırasıyla (1000 transfer, 20 çalıştırma)
ks 'for i in $(seq 20); do timeout 2 ./target/debug/kilitlenme dongu-sirali 1000 > /dev/null; echo "  sıralı, çıkış kodu $?"; done | sort | uniq -c'
