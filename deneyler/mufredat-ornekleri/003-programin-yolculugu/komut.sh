#@ video: 003-programin-yolculugu
#@ degisken: strace -c süreleri ve satır sırası (tablo sürelere göre sıralanır; sayılar aynı kalır); ikili dosya boyutu derleyici sürümüne bağlı
k cargo build --release -q
ks 'stat -c "%s bayt  %n" target/release/merhaba'
k file target/release/merhaba
# Dosyanın ilk 16 baytı: ELF imzası
k od -A x -t x1z -N 16 target/release/merhaba
k ./target/release/merhaba
# Ekrana yazmak için çekirdekten yardım isteniyor: write sistem çağrısı
ks 'strace -e trace=write ./target/release/merhaba'
# Tek satırlık bir program kaç sistem çağrısı yapıyor?
ks 'strace -c ./target/release/merhaba > /dev/null'
# main fonksiyonu makine kodunda
ks 'nm target/release/merhaba | grep -i "merhaba.*main"'
# Kendin dene: iki println! kaç write çağrısı yapar?
cd iki-satir && k cargo build --release -q && ks 'strace -e trace=write ./target/release/iki_satir > /dev/null'; cd ..
# Çözülmüş sembol adı
ks 'nm -C target/release/merhaba | grep "merhaba::main"'
# Çağrıların kaçı yükleyiciye ve C kütüphanesine ait? Rust'ın kendi hazırlığı poll ile başlar
ks 'strace ./target/release/merhaba 2>&1 >/dev/null | grep -c ""'
ks 'strace ./target/release/merhaba 2>&1 >/dev/null | grep -n -m1 "^poll"'
