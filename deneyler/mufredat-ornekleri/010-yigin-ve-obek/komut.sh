#@ video: 010-yigin-ve-obek
#@ degisken: adresler her çalıştırmada değişir (ASLR); 100 çalıştırmadaki farklı yığın son hanesi sayısı da rastgeledir, öbekte hep 1
k cargo build -q
k ./target/debug/yigin_obek
# Kendin dene: ikinci çalıştırmada hangi haneler değişiyor?
k ./target/debug/yigin_obek
# 100 çalıştırmada son üç onaltılık hane kaç farklı değer alıyor? (yığın ve öbek)
ks 'for i in $(seq 100); do ./target/debug/yigin_obek | grep "^yığın"; done | sed -E "s/.*(...)$/\1/" | sort -u | wc -l'
ks 'for i in $(seq 100); do ./target/debug/yigin_obek | grep "Box<u64>"; done | sed -E "s/.*(...)$/\1/" | sort -u | wc -l'
