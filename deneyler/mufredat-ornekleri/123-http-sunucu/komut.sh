#@ video: 123-http-sunucu
# Kontrol noktası: bu videonun sonundaki kod kapi/ dizininde (122'ye göre farkı: diff -ru).
cd kapi
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
./target/release/kapi havuz 127.0.0.1:8101 kok 8 2> sunucu.log &
SP=$!
sleep 0.5
ks 'curl -s -i http://127.0.0.1:8101/'
# HEAD: aynı başlıklar, gövde yok
ks 'curl -s -I http://127.0.0.1:8101/belgeler/not.txt'
ks 'curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/yok.html'
# Dizin geçişi denemesi: curl yolu "düzeltmesin" diye --path-as-is
ks 'curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/../../etc/passwd"'
# Yüzde kodlu ".." (%2e%2e): curl yolu kendisi düzeltip gönderir; --path-as-is ile olduğu gibi gider
ks 'curl -s -v -o /dev/null "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd" 2>&1 | grep "^> GET"'
ks 'curl -s -v --path-as-is -o /dev/null "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd" 2>&1 | grep "^> GET"'
ks 'curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd"'
# Bileşen denetiminin sınırı: kökün içindeki bir sembolik bağ dışarıyı gösterirse
k ln -s /etc kok/disari
ks 'curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/disari/os-release'
ks 'curl -s http://127.0.0.1:8101/disari/os-release | head -n 1'
k rm kok/disari
ks 'curl -s -i -X POST http://127.0.0.1:8101/'
# Kalıcı bağlantı: aynı TCP bağlantısında art arda iki istek, iki yanıt
ks 'exec 3<>/dev/tcp/127.0.0.1/8101; printf "GET /belgeler/not.txt HTTP/1.1\r\nHost: kapi\r\n\r\nGET /belgeler/not.txt HTTP/1.1\r\nHost: kapi\r\nConnection: close\r\n\r\n" >&3; cat <&3'
# Bozuk istek satırı
ks 'exec 3<>/dev/tcp/127.0.0.1/8101; printf "MERHABA\r\n\r\n" >&3; cat <&3'
kill $SP
k cat sunucu.log
rm -f sunucu.log kok/disari
