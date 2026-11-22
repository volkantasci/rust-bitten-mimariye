#@ video: 001-neden-rust-neden-bilim
#@ degisken: valgrind satırlarındaki süreç kimliği (==N==) ve adresler
#@ goster: ^isim =
# C: derlenir (en fazla bir uyarıyla), çalışır, yanlış veriyi sessizce basar
mkdir -p target
k gcc -Wall -Wextra -O0 -g -o target/kullan_sonra_birak c/kullan_sonra_birak.c
k ./target/kullan_sonra_birak
# İade başka fonksiyonda olunca uyarı da yok
k gcc -Wall -Wextra -O0 -g -o target/kullan_sonra_birak2 c/kullan_sonra_birak2.c
k ./target/kullan_sonra_birak2
# valgrind belleği izleyerek hatayı yakalar (ve davranış değişir!)
ks 'valgrind -q ./target/kullan_sonra_birak2 2>&1 | head -14'
k valgrind -q --error-exitcode=1 ./target/kullan_sonra_birak2
# Rust: aynı hata derleme aşamasında reddedilir
cd rust
k cargo build
