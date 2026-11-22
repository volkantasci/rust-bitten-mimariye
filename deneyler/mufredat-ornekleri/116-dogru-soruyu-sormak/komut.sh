#@ video: 116-dogru-soruyu-sormak
#@ degisken: iş parçacığı kimlikleri, test süreleri
# Önce sözleşme: imza + belge + testler; gövde todo!()
cd 1-sozlesme && ks 'cargo test -q 2>&1 | grep -E "not yet implemented|test result" | sort | uniq -c'; cd ..
# Sonra uygulama (ajan ya da insan): aynı testler
cd 2-uygulama && ks 'cargo test -q 2>&1 | grep "test result" | head -n 1'; cd ..
