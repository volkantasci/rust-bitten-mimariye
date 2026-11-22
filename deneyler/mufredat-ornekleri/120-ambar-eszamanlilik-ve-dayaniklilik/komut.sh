#@ video: 120-ambar-eszamanlilik-ve-dayaniklilik
#@ ortak: ortak-projeler/ambar
#@ degisken: süre, onaylanan yazma sayısı (sunucunun öldürüldüğü ana bağlı)
cd ../ortak-projeler/ambar
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
# 1) 8 istemci aynı anda yazıyor: tek bir yazma bile kayboluyor mu?
./target/release/ambar sunucu 127.0.0.1:7002 calisma/eszamanli yok 2>/dev/null &
SP=$!
sleep 0.5
k ./target/release/ambar yuk 127.0.0.1:7002 8 2000
kill $SP
# 2) Çökme deneyi: sunucu "her yazmada fsync" ile çalışırken kill -9
./target/release/ambar sunucu 127.0.0.1:7003 calisma/cokme her 2>/dev/null &
SP=$!
sleep 0.5
./target/release/ambar yazici 127.0.0.1:7003 100000000 > calisma/yazici.txt &
YP=$!
sleep 2
{ kill -9 $SP; wait $SP; } 2>/dev/null
wait $YP
k cat calisma/yazici.txt
k ./target/release/ambar kurtar calisma/cokme
# 3) Aynı deney fsync olmadan: süreç ölse de çekirdeğin sayfa önbelleği yaşıyor
./target/release/ambar sunucu 127.0.0.1:7004 calisma/cokme_yok yok 2>/dev/null &
SP=$!
sleep 0.5
./target/release/ambar yazici 127.0.0.1:7004 100000000 > calisma/yazici_yok.txt &
YP=$!
sleep 2
{ kill -9 $SP; wait $SP; } 2>/dev/null
wait $YP
k cat calisma/yazici_yok.txt
ks './target/release/ambar kurtar calisma/cokme_yok | grep "geçerli kayıt"'
rm -rf calisma
