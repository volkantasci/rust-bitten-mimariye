#@ video: 086-kendi-yurutucumuz
#@ degisken: milisaniye değerleri ±1-2 ms oynayabilir; işlemci süreleri ve meşgul sürümdeki yoklama sayısı
k cargo build -q
k ./target/debug/yurutucu
# İşlemci ne kadar çalıştı? Uyuyan yürütücü ile hiç uyumadan yoklayan (meşgul) yürütücü
ks '/usr/bin/time -f "uyuyan yürütücü: işlemci kullanıcı %U s, sistem %S s; geçen süre %e s" ./target/debug/yurutucu 2>&1 | tail -n 1'
ks '/usr/bin/time -f "meşgul yürütücü: işlemci kullanıcı %U s, sistem %S s; geçen süre %e s" ./target/debug/yurutucu mesgul 2>&1 | tail -n 3'
# Kendin dene: uyandırıcı unpark yapmazsa (2 saniye sonra timeout)
k timeout 2 ./target/debug/yurutucu unpark-yok
