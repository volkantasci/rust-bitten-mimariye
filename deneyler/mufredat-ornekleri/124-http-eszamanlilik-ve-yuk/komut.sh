#@ video: 124-http-eszamanlilik-ve-yuk
#@ ortak: ortak-projeler/kapi
#@ degisken: istek/s ve süreler
cd ../ortak-projeler/kapi
k cargo build -q --release
./target/release/kapi havuz 127.0.0.1:8101 kok 8 2>/dev/null &
P1=$!
./target/release/kapi async 127.0.0.1:8102 kok 2>/dev/null &
P2=$!
sleep 0.5
echo "# 8 işçili havuz:"
k ./target/release/kapi yuk 127.0.0.1:8101 100 200
echo "# tokio (görev başına bağlantı):"
k ./target/release/kapi yuk 127.0.0.1:8102 100 200
echo "# 8 yavaş istemci (başlığı bitirmiyor) varken:"
k ./target/release/kapi yavas 127.0.0.1:8101 8
k ./target/release/kapi yavas 127.0.0.1:8102 8
kill $P1 $P2
