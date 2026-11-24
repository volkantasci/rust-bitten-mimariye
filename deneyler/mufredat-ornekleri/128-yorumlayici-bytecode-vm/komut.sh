#@ video: 128-yorumlayici-bytecode-vm
#@ degisken: süreler ve hızlanma oranı; callgrind satırlarındaki süreç kimliği (==N==); komut sayıları küçük oranda değişebilir
# Kontrol noktası: bu videonun sonundaki kod (Minik'in bitmiş hâli) minik/ dizininde (127'ye göre farkı: diff -ru).
#@ goster: ^14$|iki motorun çıktısı aynı|^kare |tanimsiz\.mk|tanımsız değişken|^1$|^ağaç dolaşan|^bayt kod|^hızlanma|Collected|PROGRAM TOTALS|::ifade|hash_one|Sip13Rounds|malloc\.c|::calistir|::ikili
cd minik
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
k cat ornekler/kare.mk
k ./target/release/minik listele ornekler/kare.mk
k ./target/release/minik vm ornekler/kare.mk
# Fark testi: iki motor aynı programda aynı çıktıyı veriyor mu?
ks 'diff <(./target/release/minik calistir ornekler/fib.mk) <(./target/release/minik vm ornekler/fib.mk) && echo "fib.mk: iki motorun çıktısı aynı"'
# Gerçek bir fark: tanımsız ad ağaç dolaşanda çalışırken, derleyicide derlerken yakalanır
k cat ornekler/tanimsiz.mk
k ./target/release/minik calistir ornekler/tanimsiz.mk
k ./target/release/minik vm ornekler/tanimsiz.mk
k cat ornekler/kiyas.mk
k ./target/release/minik kiyasla ornekler/kiyas.mk
# Fark nereden geliyor? valgrind'in callgrind aracıyla yürütülen makine komutu sayısı, fib(20)
ks 'valgrind --tool=callgrind --callgrind-out-file=cg.agac ./target/release/minik calistir ornekler/fib20.mk 2>&1 | grep -E "^[0-9]+$|Collected"'
ks 'valgrind --tool=callgrind --callgrind-out-file=cg.vm ./target/release/minik vm ornekler/fib20.mk 2>&1 | grep -E "^[0-9]+$|Collected"'
# Ağaç dolaşanda komutların gittiği yerler: özet sayısı (hash), bellek ayırma, aritmetik
ks 'callgrind_annotate --inclusive=no cg.agac | grep -E "PROGRAM TOTALS|::ifade.2|hash_one|Sip13Rounds|malloc.c:(malloc|free) |::ikili " | sed -E "s/ \[.*//; s/[?]{3}://"'
ks 'callgrind_annotate --inclusive=no cg.vm | grep -E "PROGRAM TOTALS|::calistir |::ikili " | sed -E "s/ \[.*//; s/[?]{3}://"'
rm -f cg.agac cg.vm
