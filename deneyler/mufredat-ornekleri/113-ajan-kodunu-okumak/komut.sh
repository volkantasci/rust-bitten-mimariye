#@ video: 113-ajan-kodunu-okumak
#@ degisken: iş parçacığı kimlikleri, test süreleri
# Ajanın kendi testleri geçiyor:
cd ajanin-yazdigi
ks 'cargo test -q ajanin_testleri 2>&1 | grep "test result"'
# İncelemede eklediğimiz sınır testleri:
ks 'cargo test -q inceleme_testleri 2>&1 | grep -E "^---- |panicked|attempt|ParseIntError|test result"'
cd ../incelenmis
ks 'cargo test -q 2>&1 | grep "test result" | head -n 1'
