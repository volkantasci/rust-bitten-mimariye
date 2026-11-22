#@ video: 119-ambar-ag-protokolu
#@ ortak: ortak-projeler/ambar
cd ../ortak-projeler/ambar
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
./target/release/ambar sunucu 127.0.0.1:7001 calisma/veri her 2> calisma/sunucu.log &
SP=$!
sleep 0.5
k ./target/release/ambar istemci 127.0.0.1:7001 "SET dil rust" "GET dil" "SET selam merhaba dünya" "GET selam" "DEL dil" "GET dil" "SAY" "UCUR hepsini"
# Protokol düz metin: istemci olmadan, ham TCP ile de konuşulabilir
ks 'exec 3<>/dev/tcp/127.0.0.1/7001; printf "GET selam\nSAY\n" >&3; head -n 2 <&3'
kill $SP
k cat calisma/sunucu.log
rm -rf calisma
