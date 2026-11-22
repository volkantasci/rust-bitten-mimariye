#@ video: 088-tokio-ile-async
#@ degisken: süreler ve kovalara düşen istemci sayıları
k cargo build -q --release
k ./target/release/tokio_sunucu 100
# Dinleme kuyruğu taşma sayaçları (bu konteynerin ağ ad alanı) — önce
ks "awk '/^TcpExt:/ { if (!h) { split(\$0, ad); h=1 } else { for (i=2;i<=NF;i++) if (ad[i] ~ /^Listen(Overflows|Drops)\$/) print ad[i], \$i } }' /proc/net/netstat"
k ./target/release/tokio_sunucu 1000
# ... ve sonra
ks "awk '/^TcpExt:/ { if (!h) { split(\$0, ad); h=1 } else { for (i=2;i<=NF;i++) if (ad[i] ~ /^Listen(Overflows|Drops)\$/) print ad[i], \$i } }' /proc/net/netstat"
# tokio'nun bind() ile kurduğu dinleme kuyruğu uzunluğu
ks 'strace -f -e trace=listen ./target/release/tokio_sunucu 10 2>&1 | grep "listen("'
