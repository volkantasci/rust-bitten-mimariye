#@ video: 120-ambar-eszamanlilik-ve-dayaniklilik
#@ degisken: süre ve yazma/s; onaylanan ve kurtarılan kayıt sayıları (sunucunun öldürüldüğü ana bağlı)
# Kontrol noktası: bu videonun sonundaki kod ambar/ dizininde (119'a göre farkı: diff -ru).
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
# 1) 8 istemci aynı anda yazıyor: tek bir yazma bile kayboluyor mu?
./target/release/ambar sunucu calisma/eszamanli 127.0.0.1:7002 yok 2>/dev/null &
SP=$!
sleep 0.5
k ./target/release/ambar yuk 127.0.0.1:7002 8 2000
kill $SP
k ./target/release/ambar durum calisma/eszamanli
# 2) Çökme deneyi: sunucu "her yazmada fsync" ile çalışırken SIGKILL (kill -9)
./target/release/ambar sunucu calisma/cokme 127.0.0.1:7003 her 2>/dev/null &
SP=$!
sleep 0.5
./target/release/ambar yazici 127.0.0.1:7003 100000000 > calisma/yazici.txt &
YP=$!
sleep 2
{ kill -9 $SP; wait $SP; } 2>/dev/null
wait $YP
k cat calisma/yazici.txt
k ./target/release/ambar durum calisma/cokme
# 3) Aynı deney fsync olmadan: süreç ölür, çekirdeğin sayfa önbelleği yaşar
./target/release/ambar sunucu calisma/cokme_yok 127.0.0.1:7004 yok 2>/dev/null &
SP=$!
sleep 0.5
./target/release/ambar yazici 127.0.0.1:7004 100000000 > calisma/yazici_yok.txt &
YP=$!
sleep 2
{ kill -9 $SP; wait $SP; } 2>/dev/null
wait $YP
k cat calisma/yazici_yok.txt
k ./target/release/ambar durum calisma/cokme_yok
rm -rf calisma
