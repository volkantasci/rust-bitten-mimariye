#@ video: 038-hata-tipleri-tasarlamak
k cargo build -q
k ./target/debug/ayar ayarlar/iyi.conf
k ./target/debug/ayar ayarlar/bozuk.conf
k ./target/debug/ayar ayarlar/yok.conf
