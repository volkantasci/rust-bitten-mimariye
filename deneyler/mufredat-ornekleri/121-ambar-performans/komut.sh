#@ video: 121-ambar-performans
#@ degisken: yazma/s, gecikme ve fsync sayıları diske, yüke ve zamanlamaya bağlı
# Kontrol noktası: bu videonun sonundaki kod (Ambar'ın bitmiş hâli) ambar/ dizininde (120'ye göre farkı: diff -ru).
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
# Veri nerede? /work ana makinenin diskine bağlı (btrfs)
ks 'grep " /work " /proc/mounts | cut -d" " -f1-3'
port=7010
for kip in her toplu yok; do
  port=$((port + 1))
  ./target/release/ambar sunucu calisma/p_$kip 127.0.0.1:$port $kip 2>/dev/null &
  SP=$!
  sleep 0.5
  echo "# fsync kipi: $kip (veri: /work, ana makinenin diski)"
  k ./target/release/ambar yuk 127.0.0.1:$port 16 300
  kill $SP
done
# Tuzak: aynı "her" kipi konteynerin kendi kök dosya sisteminde (/tmp). Kök nasıl bağlı?
ks 'grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"'
./target/release/ambar sunucu /tmp/ambar_her 127.0.0.1:7019 her 2>/dev/null &
SP=$!
sleep 0.5
echo "# fsync kipi: her (veri: konteynerin /tmp'si)"
k ./target/release/ambar yuk 127.0.0.1:7019 16 300
kill $SP
rm -rf calisma /tmp/ambar_her
