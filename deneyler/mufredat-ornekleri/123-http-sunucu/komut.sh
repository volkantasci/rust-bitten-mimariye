#@ video: 123-http-sunucu
#@ ortak: ortak-projeler/kapi
cd ../ortak-projeler/kapi
k cargo build -q --release
./target/release/kapi havuz 127.0.0.1:8101 kok 8 2> /tmp/kapi.log &
SP=$!
sleep 0.5
ks 'curl -s -i http://127.0.0.1:8101/'
ks 'curl -s http://127.0.0.1:8101/belgeler/not.txt'
ks 'curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/yok.html'
# Dizin geçişi saldırısı: curl yolu "düzeltmesin" diye --path-as-is
ks 'curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/../../etc/passwd"'
ks 'curl -s -o /dev/null -w "%{http_code}\n" -X POST http://127.0.0.1:8101/'
# Bozuk istek: ham TCP
ks 'exec 3<>/dev/tcp/127.0.0.1/8101; printf "MERHABA\r\n\r\n" >&3; cat <&3'
kill $SP
k cat /tmp/kapi.log
