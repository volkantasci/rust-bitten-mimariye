#@ video: 088-tokio-ile-async
#@ degisken: süreler, kovalara düşen istemci sayıları ve taşma sayaçları
k cargo build -q --release
k bash sayac.sh başta
k ./target/release/tokio_sunucu 100
k ./target/release/tokio_sunucu 1000
k bash sayac.sh "1000 istemciden sonra"
# tokio'nun bind() ile kurduğu kuyruk uzunluğu ve çekirdeğin üst sınırı
ks 'strace -f -e trace=listen ./target/release/tokio_sunucu 10 2>&1 | grep "listen("'
k cat /proc/sys/net/core/somaxconn
# Kuyruk doluyken çekirdek ret mi gönderir (1), adımı sessizce mi düşürür (0)?
k cat /proc/sys/net/ipv4/tcp_abort_on_overflow
# Kendin dene: kuyruğu 1024 yap
k ./target/release/tokio_sunucu 1000 1024
k bash sayac.sh "kuyruk 1024 ile sonra"
