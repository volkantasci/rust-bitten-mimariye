#@ video: 080-http-elle
k cargo build -q
./target/debug/http_elle 2 2> calisma_sunucu.log &
sleep 0.5
# 1) Elle yazılmış ham istek; yanıtın baytları od -c ile (\r ve \n görünür)
ks './target/debug/http_elle istemci | od -c | head -n 12'
# 2) curl ile: > giden, < gelen satırlar
ks 'curl -sv http://127.0.0.1:8099/ 2>&1 | grep -E "^[<>] |^Merhaba"'
wait
k cat calisma_sunucu.log
rm -f calisma_sunucu.log
# Kendin dene: Content-Length 15 yerine 10
./target/debug/http_elle 1 10 2> /dev/null &
sleep 0.5
ks 'curl -s http://127.0.0.1:8099/; echo " ← curl çıktısının sonu"'
wait
