#@ video: 079-udp-ve-dns
#@ degisken: kaynak port numaraları
k cargo build -q
k ./target/debug/udp_dns
# Ad çözümleme: kaynağı /etc/hosts; hangi sırayla bakılacağını /etc/nsswitch.conf söyler
k ./target/debug/udp_dns coz
ks 'grep -E "^hosts:" /etc/nsswitch.conf'
ks 'grep -v "^#" /etc/hosts | grep localhost'
# Çözümleme sırasında açılan dosyalar
ks 'strace -e trace=openat ./target/debug/udp_dns coz 2>&1 | grep -E "nsswitch|hosts|resolv"'
# Kendin dene: /etc/hosts'a yeni bir ad eklemek (yalnız bu geçici konteynerde); ağ çağrılarına da bakalım (53 numaralı porta gidilmemeli)
ks 'echo "127.0.0.1 laboratuvar.test" >> /etc/hosts && strace -e trace=openat,connect,sendto ./target/debug/udp_dns coz laboratuvar.test 2>&1 | grep -E "nsswitch|hosts|resolv|connect|sendto|laboratuvar"'
