#@ video: 059-birim-ve-entegrasyon-testleri
#@ degisken: test süreleri, iş parçacığı kimliği
# Birim + entegrasyon + dokümantasyon testleri (ilk hatada durmadan)
cd hatali && k cargo test -q --no-fail-fast; cd ..
# Düzeltme: fark (dosya tarihleri yazılmasın diye etiketle)
ks 'diff -u --label hatali/src/lib.rs --label duzeltilmis/src/lib.rs hatali/src/lib.rs duzeltilmis/src/lib.rs'
cd duzeltilmis && k cargo test -q; cd ..
