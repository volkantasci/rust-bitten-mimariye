#@ video: 012-isletim-sistemi-ve-sistem-cagrilari
#@ degisken: süreler donanıma ve çekirdek ayarlarına bağlı
k cargo build -q --release
k ./target/release/sistem_cagrisi
# Gerçekten her seferinde çekirdeğe gidiyor mu? 1000 çağrıyı sayalım
ks 'strace -c -e trace=getpid ./target/release/sistem_cagrisi 1000 > /dev/null'
