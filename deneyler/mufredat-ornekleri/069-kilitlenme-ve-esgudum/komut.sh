#@ video: 069-kilitlenme-ve-esgudum
#@ degisken: iki iş parçacığının yazdırma sırası
cd kilitlenme
k cargo build -q
# Ters sıralı kilitler: sonsuza dek bekler; 3 saniye sonra timeout öldürür (124)
k timeout 3 ./target/debug/kilitlenme
# Tek bir kilit sırası: kilitlenme yok
k timeout 3 ./target/debug/kilitlenme sirali
