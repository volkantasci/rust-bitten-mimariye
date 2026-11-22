#@ video: 059-birim-ve-entegrasyon-testleri
#@ degisken: test süreleri, iş parçacığı kimliği
# Birim + entegrasyon + belge testleri (ilk hatada durmadan)
cd hatali && k cargo test -q --no-fail-fast; cd ..
# Düzeltme: fark
ks 'diff -u hatali/src/lib.rs duzeltilmis/src/lib.rs'
cd duzeltilmis && k cargo test -q; cd ..
