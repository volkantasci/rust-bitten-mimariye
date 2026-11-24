#@ video: 124-http-eszamanlilik-ve-yuk
#@ degisken: istek/s, bağlantı süreleri ve yanıt süreleri (makineye ve zamanlamaya bağlı)
# Kontrol noktası: bu videonun sonundaki kod (Kapı'nın bitmiş hâli) kapi/ dizininde (123'e göre farkı: diff -ru).
cd kapi
ks 'cargo test --lib 2>&1 | grep -E "^test result"'
k cargo build -q --release
./target/release/kapi havuz 127.0.0.1:8101 kok 8 2>/dev/null &
P1=$!
./target/release/kapi async 127.0.0.1:8102 kok 2>/dev/null &
P2=$!
sleep 0.5
echo "# 8 işçili havuz:"
k ./target/release/kapi yuk 127.0.0.1:8101 100 200
echo "# tokio (her bağlantıya bir görev):"
k ./target/release/kapi yuk 127.0.0.1:8102 100 200
echo "# 8 yavaş istemci (başlığı bitirmeyen) varken; sunucularda son süre yok:"
k ./target/release/kapi yavas 127.0.0.1:8101 8
k ./target/release/kapi yavas 127.0.0.1:8102 8
kill $P1 $P2
echo "# aynı deneme, sunucularda 1000 ms istek son süresiyle:"
./target/release/kapi havuz 127.0.0.1:8103 kok 8 1000 2>/dev/null &
P1=$!
./target/release/kapi async 127.0.0.1:8104 kok 1000 2>/dev/null &
P2=$!
sleep 0.5
k ./target/release/kapi yavas 127.0.0.1:8103 8
k ./target/release/kapi yavas 127.0.0.1:8104 8
kill $P1 $P2
