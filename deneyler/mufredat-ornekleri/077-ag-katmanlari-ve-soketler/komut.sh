#@ video: 077-ag-katmanlari-ve-soketler
#@ degisken: port numaraları (onaltılık /proc/net/tcp satırlarında da), dosya tanımlayıcı numaraları, milisaniyeler
k cargo build -q
k ./target/debug/soketler
# Çekirdeğin istemci tarafına geçici port seçtiği aralık
k cat /proc/sys/net/ipv4/ip_local_port_range
# accept çağrılmadan önce (120. ms) çekirdeğin TCP tablosu: 0A = LISTEN, 01 = ESTABLISHED; tx_queue:rx_queue onaltılık bayt sayısı
ks './target/debug/soketler > /dev/null & sleep 0.12; awk "NR==1 || \$4==\"01\" || \$4==\"0A\"" /proc/net/tcp | cut -c1-60; wait'
# Aynı programın ağ sistem çağrıları
ks 'strace -f -e trace=socket,bind,listen,accept4,connect,sendto,recvfrom ./target/debug/soketler 2>&1 | grep -E "socket\(|bind\(|listen\(|accept4\(|connect\(|sendto\(|recvfrom\([0-9]+, \""'
# Kendin dene: accept 3 saniye geciktirilince
k ./target/debug/soketler 3000
