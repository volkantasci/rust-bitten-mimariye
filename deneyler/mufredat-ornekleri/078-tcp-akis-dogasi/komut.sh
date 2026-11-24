#@ video: 078-tcp-akis-dogasi
#@ degisken: süreler; dosya tanımlayıcı numaraları
k cargo run -q --release
# set_nodelay hangi sistem çağrısına dönüşüyor?
ks 'strace -f -e trace=setsockopt ./target/release/tcp_akis 2>&1 | grep TCP_NODELAY'
# Kendin dene: NODELAY'i istemci yerine sunucuda açmak
k ./target/release/tcp_akis sunucu-nodelay
