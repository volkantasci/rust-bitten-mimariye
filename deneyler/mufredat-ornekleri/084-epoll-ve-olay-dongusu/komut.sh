#@ video: 084-epoll-ve-olay-dongusu
#@ degisken: dosya tanımlayıcı numaraları, port numaraları, data= alanındaki adresler
k cargo build -q
# std ile engellemeyen soket (ve aynı çağrının çekirdekten dönen makbuzu)
k ./target/debug/olay_dongusu std
ks 'strace -e trace=accept4 ./target/debug/olay_dongusu std 2>&1 | grep accept4'
k ./target/debug/olay_dongusu
# Olay döngüsünün çekirdekle konuşması: epoll (fcntl satırı: epoll tanımlayıcısının kopyası)
ks 'strace -f -e trace=epoll_create1,epoll_ctl,epoll_wait,epoll_pwait,epoll_pwait2,accept4,connect,fcntl ./target/debug/olay_dongusu 2>&1 | grep -v "^yankı" | grep -vE "fcntl\([0-9]+, F_(GETFL|SETFL|GETFD|SETFD)"'
# Kendin dene: aynı accept engelleyen sokette (bağlantı gelmediği için bekler; 2 saniye sonra timeout)
k timeout 2 ./target/debug/olay_dongusu std-engelleyen
