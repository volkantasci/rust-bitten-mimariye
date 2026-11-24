# Bekleyen bağlantı kuyruğu taşma sayacı (/proc/net/netstat, TcpExt: ListenOverflows).
# Sayaç bu ağ ad alanında çekirdek açıldığından beri toplanır. $1: satırın başına yazılacak etiket.
awk -v etiket="$1" '/^TcpExt:/ { if (!h) { split($0, ad); h = 1 } else { for (i = 2; i <= NF; i++) if (ad[i] == "ListenOverflows") print etiket ": ListenOverflows " $i } }' /proc/net/netstat
