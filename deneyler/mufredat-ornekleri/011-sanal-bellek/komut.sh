#@ video: 011-sanal-bellek
#@ degisken: adresler (ASLR), RSS ve sayfa hatası sayıları (THP ayarına bağlı)
k getconf PAGESIZE
k cat /sys/kernel/mm/transparent_hugepage/enabled
k cargo build -q --release
k ./target/release/sanal_bellek
k ./target/release/sanal_bellek
