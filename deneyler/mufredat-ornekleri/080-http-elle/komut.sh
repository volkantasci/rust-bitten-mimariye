#@ video: 080-http-elle
k cargo build -q
./target/debug/http_elle 2 2> calisma_sunucu.log &
sleep 0.5
# 1) Elle yazılmış ham istek (bash'in /dev/tcp yönlendirmesiyle)
ks 'exec 3<>/dev/tcp/127.0.0.1/8099; printf "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n" >&3; cat <&3 | od -c | head -n 12'
# 2) curl ile: > giden, < gelen satırlar
ks 'curl -sv http://127.0.0.1:8099/ 2>&1 | grep -E "^[<>] |^Merhaba"'
wait
k cat calisma_sunucu.log
rm -f calisma_sunucu.log
