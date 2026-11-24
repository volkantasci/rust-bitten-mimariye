#@ video: 038-hata-tipleri-tasarlamak
#@ degisken: panik satırındaki iş parçacığı kimliği
k cargo build -q
k ./target/debug/ayar ayarlar/iyi.conf
k ./target/debug/ayar ayarlar/bozuk.conf
k ./target/debug/ayar ayarlar/yok.conf
# Ajan Masası (K8): unwrap() ile ayrıştıran ajan kodu; bozuk girdi ve u16 taşması panik verir
cd ajan-masasi && k cargo build -q --release
k ./target/release/ajan 8080
k ./target/release/ajan abc
k ./target/release/ajan 70000
# Düzeltilmiş sürüm: aynı girdiler, panik yerine bağlamlı hata ve çıkış kodu 1
k ./target/release/duzeltilmis abc
k ./target/release/duzeltilmis 70000
cd ..
