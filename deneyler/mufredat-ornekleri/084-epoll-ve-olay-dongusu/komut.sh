#@ video: 084-epoll-ve-olay-dongusu
#@ degisken: dosya tanımlayıcı numaraları
k cargo build -q
k ./target/debug/olay_dongusu
# Olay döngüsünün çekirdekle konuşması: epoll
ks 'strace -f -e trace=epoll_create1,epoll_ctl,epoll_wait,epoll_pwait,epoll_pwait2,accept4,connect ./target/debug/olay_dongusu 2>&1 | grep -v "^yankı"'
