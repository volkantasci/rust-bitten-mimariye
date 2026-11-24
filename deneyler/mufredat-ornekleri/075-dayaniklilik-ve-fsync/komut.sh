#@ video: 075-dayaniklilik-ve-fsync
#@ degisken: süreler diske ve dosya sistemine bağlı; inode numaraları
mkdir -p calisma
k cargo build -q --release
# Ana makinenin diskine bağlı dizin (btrfs)
k ./target/release/dayaniklilik olc calisma
# sync_data() hangi sistem çağrısı, kaç kez? (her kayıtta 200 + iki döngünün sonundaki 2)
ks 'strace -c -e trace=fsync,fdatasync ./target/release/dayaniklilik olc calisma 2>&1 | grep -E "fdatasync$|^her|^sonda"'
# Konteynerin kendi kök dosya sistemi (/tmp burada)
k ./target/release/dayaniklilik olc /tmp
# Neden bu kadar hızlı? Kök dizinin ve /work'ün bağlama bilgileri
ks 'grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"'
ks 'grep " /work " /proc/mounts | cut -d" " -f1-3'
# fsync çağrılmazsa: çekirdek kirli sayfaları kendisi yazar; bu kadar santisaniyeden eski kirli veri geri yazmaya uygun olur
k cat /proc/sys/vm/dirty_expire_centisecs
# Atomik değiştirme: önce eski sürüm (1), sonra kalıp; dosya adı aynı kalır, dosyanın kendisi (inode) değişir
ks 'echo "{\"surum\": 1}" > calisma/ayar.json && cat calisma/ayar.json && stat -c "inode %i" calisma/ayar.json'
ks 'strace -e trace=openat,write,fsync,fdatasync,rename,renameat2 ./target/release/dayaniklilik atomik calisma 2>&1 | grep -v "/lib\|/etc\|/proc\|/sys"'
ks 'cat calisma/ayar.json && stat -c "inode %i" calisma/ayar.json'
# Kendin dene: sync_data yerine sync_all (fsync)
ks 'strace -c -e trace=fsync,fdatasync ./target/release/dayaniklilik olc-tum calisma 2>&1 | grep -E "fsync$|^her|^sonda"'
k ./target/release/dayaniklilik olc-tum calisma
