#@ video: 129-yolun-sonu
#@ ortak: ortak-projeler/ambar
#@ ortak: ortak-projeler/kapi
#@ ortak: ortak-projeler/minik
#@ degisken: strace satırlarının başındaki iş parçacığı kimlikleri, dosya tanımlayıcı numaraları, istemci portu (sin_port) ve "accept4 resumed" satırındaki adres
# 1) Bitirme projelerinin on iki kontrol noktasında testler
ks 'for c in ../11[7-9]-*/ambar/Cargo.toml ../12[01]-*/ambar/Cargo.toml ../12[2-4]-*/kapi/Cargo.toml ../12[5-8]-*/minik/Cargo.toml; do d=${c%/Cargo.toml}; printf "%-42s %s\n" "${d#../}" "$(cd $d && cargo test -q --lib 2>&1 | grep -o "[0-9]* passed; [0-9]* failed")"; done'
# ortak-projeler/ altındaki bitmiş projeler, son kontrol noktalarının aynısı mı?
ks 'diff -r -x target ../121-ambar-performans/ambar ../ortak-projeler/ambar && diff -r -x target ../124-http-eszamanlilik-ve-yuk/kapi ../ortak-projeler/kapi && diff -r -x target ../128-yorumlayici-bytecode-vm/minik ../ortak-projeler/minik && echo "ortak-projeler: son kontrol noktalarıyla aynı"'
# 2) Bir isteğin çekirdeğe dokunan adımları: Ambar'a tek bir "yaz" ve sistem çağrıları
cd ../ortak-projeler/ambar
cargo build -q --release
rm -rf calisma
echo "# her yazmada fsync kipi"
strace -f -o strace.txt -e trace=accept4,recvfrom,write,sendto,fdatasync ./target/release/ambar sunucu calisma 127.0.0.1:7029 her 2>/dev/null &
SP=$!
sleep 1
ks 'exec 3<>/dev/tcp/127.0.0.1/7029; printf "yaz dil Rust\n" >&3; head -n 1 <&3'
sleep 0.2
kill $(pgrep -P $SP)   # izlenen sunucuyu durdur; strace onunla birlikte çıkar
wait $SP 2>/dev/null
ks 'grep -v "write(2," strace.txt | grep -E "accept4|recvfrom|write|sendto|fdatasync"'
k od -A d -t x1z -v calisma/gunluk.ambar
rm -rf calisma strace.txt
echo "# toplu kesinleştirme kipi"
strace -f -o strace.txt -e trace=accept4,recvfrom,write,sendto,fdatasync ./target/release/ambar sunucu calisma 127.0.0.1:7030 toplu 2>/dev/null &
SP=$!
sleep 1
ks 'exec 3<>/dev/tcp/127.0.0.1/7030; printf "yaz dil Rust\n" >&3; head -n 1 <&3'
sleep 0.2
kill $(pgrep -P $SP)
wait $SP 2>/dev/null
ks 'grep -v "write(2," strace.txt | grep -E "recvfrom\([0-9]+, \"yaz|write\(|sendto|fdatasync"'
rm -rf calisma strace.txt
cd ../../129-yolun-sonu
# 3) Üç bitmiş projenin Rust satır sayısı ve bu müfredatın deney dizinleri
ks 'wc -l ../ortak-projeler/*/src/*.rs ../ortak-projeler/*/src/bin/*.rs | tail -n 1'
ks 'printf "deney dizini: %s\n" "$(ls -d ../[0-9][0-9][0-9]-*/ | wc -l)"'
