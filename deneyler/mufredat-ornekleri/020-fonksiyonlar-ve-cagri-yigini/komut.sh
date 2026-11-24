#@ video: 020-fonksiyonlar-ve-cagri-yigini
#@ degisken: adresler (ASLR); çerçeve boyutu derleme ayarına bağlı
k cargo run -q
k cargo run -q --release
# Dönüş adresleri: gdb seviye 3'te durur; çağrı zinciri ve kaydedilmiş dönüş adresi (gdb ASLR'yi kapatır, adresler sabit görünür)
ks 'gdb -q -batch -ex "break cagri_yigini::asagi if seviye == 3" -ex run -ex bt -ex "info frame" ./target/debug/cagri_yigini 2>&1 | grep -E "^#|saved rip|called by"'
