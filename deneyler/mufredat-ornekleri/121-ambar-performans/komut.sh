#@ video: 121-ambar-performans
#@ ortak: ortak-projeler/ambar
#@ degisken: yazma/s değerleri diske ve yüke bağlı
cd ../ortak-projeler/ambar
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
port=7010
for kip in her toplu yok; do
  port=$((port + 1))
  ./target/release/ambar sunucu 127.0.0.1:$port calisma/p_$kip $kip 2>/dev/null &
  SP=$!
  sleep 0.5
  echo "# fsync kipi: $kip (veri: ana makinenin diski, btrfs)"
  k ./target/release/ambar yuk 127.0.0.1:$port 16 300
  kill $SP
done
# Tuzak: aynı "her" kipi konteynerin kendi /tmp'sinde (overlay, fsync=volatile)
./target/release/ambar sunucu 127.0.0.1:7019 /tmp/ambar_her her 2>/dev/null &
SP=$!
sleep 0.5
echo "# fsync kipi: her (veri: konteynerin /tmp'si)"
k ./target/release/ambar yuk 127.0.0.1:7019 16 300
kill $SP
rm -rf calisma /tmp/ambar_her
