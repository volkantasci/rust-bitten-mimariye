#@ video: 075-dayaniklilik-ve-fsync
#@ degisken: süreler diske ve dosya sistemine bağlı
mkdir -p calisma
k cargo build -q --release
# Ana makinenin diskine bağlı dizin (btrfs)
k ./target/release/dayaniklilik olc calisma
# Konteynerin kendi kök dosya sistemi (/tmp burada)
k ./target/release/dayaniklilik olc /tmp
# Neden bu kadar hızlı? Kök dizinin bağlama seçeneklerine bakalım
ks 'grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"'
ks 'grep " /work " /proc/mounts | cut -d" " -f1-3'
# Atomik değiştirme kalıbının sistem çağrıları
ks 'strace -e trace=openat,write,fsync,fdatasync,rename,renameat2 ./target/release/dayaniklilik atomik calisma 2>&1 | grep -v "/lib\|/etc\|/proc\|/sys"'
k cat calisma/ayar.json
