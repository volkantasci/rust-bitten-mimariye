#@ video: 119-ambar-ag-protokolu
# Kontrol noktası: bu videonun sonundaki kod ambar/ dizininde (118'e göre farkı: diff -ru).
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
# Dokümantasyon testi: motoru kilitsiz paylaşan kod derlenmemeli (compile_fail); derleyicinin sözü:
ks 'cargo test --doc -- --nocapture 2>&1 | grep -E "^error|^ *--> |^ *= help|^test "'
k cargo build -q --release
rm -rf calisma
./target/release/ambar sunucu calisma 127.0.0.1:7001 2> sunucu.log &
SP=$!
sleep 0.5
k ./target/release/ambar istemci 127.0.0.1:7001 "yaz dil rust" "oku dil" "yaz selam merhaba dünya" "oku selam" "sil dil" "oku dil" "say" "uçur hepsini"
# Protokol düz metin: özel bir istemci olmadan, bash'in /dev/tcp yönlendirmesiyle de konuşulur
ks 'exec 3<>/dev/tcp/127.0.0.1/7001; printf "oku selam\nsay\n" >&3; head -n 2 <&3'
# Çerçeveyi satır sonu belirler: değerin içindeki "\n" isteği ikiye böler
ks 'exec 3<>/dev/tcp/127.0.0.1/7001; printf "yaz not birinci satır\nikinci satır\n" >&3; head -n 2 <&3'
# Satır sınırı: hiç "\n" göndermeden tam 1 049 616 bayt (anahtar 1 KiB + değer 1 MiB + 16)
ks 'exec 3<>/dev/tcp/127.0.0.1/7001; head -c 1049616 /dev/zero | tr "\0" a >&3; head -n 1 <&3'
kill $SP
k cat sunucu.log
# Sunucu durdu; veri günlükte. Yeni bir süreç aynı durumu kurar:
k ./target/release/ambar listele calisma
rm -rf calisma sunucu.log
