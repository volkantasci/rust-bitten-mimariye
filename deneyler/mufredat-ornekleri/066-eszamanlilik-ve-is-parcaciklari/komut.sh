#@ video: 066-eszamanlilik-ve-is-parcaciklari
#@ degisken: görülen sıralar, süreler, hızlanma oranları ve bağlam değiştirme sayıları her çalıştırmada ve her makinede değişir
# Bu makinenin işlemcisi: kaç çekirdek, çekirdek başına kaç donanım iş parçacığı?
ks 'lscpu | grep -E "^(CPU\(s\)|Thread\(s\) per core|Core\(s\) per socket):"'
k cargo build -q --release
# 1) Sıra garantisi yok
k ./target/release/is_parcacigi sira
# 2) İşi bölmek: blok, döngüsel ve dinamik dağıtım (son satır: çekirdeğin bu süreçte yaptığı bağlam değiştirmeler)
ks '/usr/bin/time -f "bağlam değiştirme: gönüllü %w, zorunlu %c" ./target/release/is_parcacigi hizlanma 2>&1'
# Kendin dene: programı tek bir mantıksal işlemciye kısıtla (eşzamanlı ama paralel değil)
ks '/usr/bin/time -f "bağlam değiştirme: gönüllü %w, zorunlu %c" taskset -c 0 ./target/release/is_parcacigi hizlanma 2>&1'
