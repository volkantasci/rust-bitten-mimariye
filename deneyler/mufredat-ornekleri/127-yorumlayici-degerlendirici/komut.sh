#@ video: 127-yorumlayici-degerlendirici
#@ degisken: yığın taşması satırlarındaki iş parçacığı kimlikleri (parantez içindeki sayılar); koruma sayfası satırlarındaki adresler
# Kontrol noktası: bu videonun sonundaki kod minik/ dizininde (126'ya göre farkı: diff -ru).
cd minik
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
k ./target/release/minik calistir ornekler/fib.mk
k cat ornekler/sifira_bolme.mk
k ./target/release/minik calistir ornekler/sifira_bolme.mk
# Kısa devre: && ve || sonuç belliyse sağ tarafı hiç değerlendirmez
k cat ornekler/kisa_devre.mk
k ./target/release/minik calistir ornekler/kisa_devre.mk
# Sonsuz özyineleme: dilin kendi derinlik sınırı (2000) Rust'ın yığını dolmadan durdurur
k cat ornekler/sonsuz.mk
k ./target/release/minik calistir ornekler/sonsuz.mk
# Sınır yükseltilirse: 64 MiB'lık yorumlayıcı yığını 60 000 çağrıya yetiyor, 80 000'e ve 1 000 000'a yetmiyor
k ./target/release/minik calistir --derinlik 60000 ornekler/sonsuz.mk
k ./target/release/minik calistir --derinlik 80000 ornekler/sonsuz.mk
k ./target/release/minik calistir --derinlik 1000000 ornekler/sonsuz.mk
cd ..
# Koruma sayfası: bir iş parçacığı yığınının hemen altındaki eşleme (/proc/self/maps)
k rustc -O koruma_sayfasi.rs -o koruma_sayfasi
k ./koruma_sayfasi
rm -f koruma_sayfasi
