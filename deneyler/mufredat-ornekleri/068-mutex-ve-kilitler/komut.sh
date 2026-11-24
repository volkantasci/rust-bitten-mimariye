#@ video: 068-mutex-ve-kilitler
#@ degisken: süreler, futex çağrı sayıları, adresler ve iş parçacığı kimlikleri
k cargo build -q --release
# 1) Aynı sayaç üç yolla: Mutex, atomik, paylaşmamak
k ./target/release/kilitler sayaclar
# 2) Çekişmenin çekirdekteki izi: futex sistem çağrıları (1 iş parçacığı ve 8 iş parçacığı)
ks 'strace -f -c -e trace=futex ./target/release/kilitler mutex 1 2>&1 | grep -E "^Mutex|futex$|total$"'
ks 'strace -f -c -e trace=futex ./target/release/kilitler mutex 8 2>&1 | grep -E "^Mutex|futex$|total$"'
# Tek iş parçacığındaki tek futex çağrısı nereden? Ana iş parçacığı, alt iş parçacığının bitmesini (join) bekliyor
ks 'strace -f -e trace=futex,clone3 ./target/release/kilitler mutex 1 2>&1 | grep -E "clone3|futex"'
# 3) Kritik bölgeyi kısa tutmak: yavaş işi kilidin dışına almak
k ./target/release/kilitler kilitte-bekleme
# 4) Zehirlenme
k ./target/release/kilitler zehir
# Kendin dene: futex çağrısı sayısı 2 ve 4 iş parçacığıyla; iş 5 ms sürünce kilitte bekleme
ks 'for n in 2 4; do strace -f -c -e trace=futex ./target/release/kilitler mutex $n 2>&1 | grep -E "^Mutex|futex$"; done'
k ./target/release/kilitler kilitte-bekleme 5
