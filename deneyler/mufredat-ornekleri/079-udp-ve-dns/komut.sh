#@ video: 079-udp-ve-dns
#@ degisken: kaynak port numaraları
k cargo run -q
ks 'grep -v "^#" /etc/hosts | grep localhost'
