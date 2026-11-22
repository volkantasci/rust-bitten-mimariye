#@ video: 001-neden-rust-neden-bilim
#@ degisken: adresler (0x...), valgrind satırlarındaki süreç kimliği (==N==), cargo'nun "Finished ... in N s" süresi
# Video 001 için ek deneyler. Ana deney: ../mufredat-ornekleri/001-neden-rust-neden-bilim/
# Çalıştırma (depo kökünden): laboratuvar/rr.sh video-001 bash calistir.sh

# Ortam: araç sürümleri (ayırıcının davranışı glibc sürümüne bağlıdır)
k rustc --version
k cargo --version
ks 'gcc --version | head -1'
k valgrind --version
ks 'ldd --version | head -1'

# Kopyalar ana deneyle bayt bayt aynı mı? (cmp çıktı vermezse aynıdır)
k cmp c/kullan_sonra_birak2.c ../mufredat-ornekleri/001-neden-rust-neden-bilim/c/kullan_sonra_birak2.c
k cmp rust/src/bin/orijinal.rs ../mufredat-ornekleri/001-neden-rust-neden-bilim/rust/src/main.rs
mkdir -p target

# 1) Sessizlik: uyarı yok, çökme yok, çıkış kodu 0
k gcc -Wall -Wextra -O0 -g -o target/kullan_sonra_birak2 c/kullan_sonra_birak2.c
ks './target/kullan_sonra_birak2; echo "çıkış kodu: $?"'

# 2) Tekrarlanabilirlik: 100 çalıştırmada kaç farklı çıktı var?
ks 'for i in $(seq 100); do ./target/kullan_sonra_birak2; done | sort | uniq -c'

# 3) Aynı raf mı? Adresler çalıştırmadan çalıştırmaya değişebilir; eşitlik değişmez
k gcc -Wall -Wextra -O0 -g -o target/adresler c/adresler.c
k ./target/adresler
k ./target/adresler
ks 'for i in $(seq 100); do ./target/adresler | grep "aynı blok"; done | sort | uniq -c'

# 4) valgrind neden davranışı değiştiriyor? Kendi ayırıcısı şifreye aynı bloğu veriyor mu?
ks 'valgrind -q ./target/adresler 2>/dev/null'
# İade edilen bloklar için bir bekletme kuyruğu da var (varsayılan boyutu)
ks 'valgrind --tool=memcheck --help 2>&1 | grep -A3 -- "--freelist-vol"'
# Kuyruk kapatılınca (0) ne oluyor? (İlk tahminimiz "blok yeniden verilir, hata kaçar" idi; çıktıya bakın)
k valgrind -q --error-exitcode=1 --freelist-vol=0 ./target/kullan_sonra_birak2

# 5) Optimizasyon açıkken GCC ne diyor? (videoda gösterilen derleme -O0)
k gcc -Wall -Wextra -O2 -g -o target/kullan_sonra_birak2_O2 c/kullan_sonra_birak2.c
k ./target/kullan_sonra_birak2_O2
# -O2 çıktısı her çalıştırmada aynı mı? (100 çalıştırmadaki farklı çıktı sayısı)
ks 'for i in $(seq 100); do ./target/kullan_sonra_birak2_O2; done | sort -u | wc -l'
# Makine kodunda kaç malloc/free çağrısı kaldı? (-O0 ve -O2)
ks 'objdump -d --no-show-raw-insn target/kullan_sonra_birak2 | grep -oE "call +[0-9a-f]+ <(malloc|free)@plt>" | grep -oE "<.*>" | sort | uniq -c'
ks 'objdump -d --no-show-raw-insn target/kullan_sonra_birak2_O2 | grep -oE "call +[0-9a-f]+ <(malloc|free)@plt>" | grep -oE "<.*>" | sort | uniq -c'
# -O2'de main'in yaptığı çağrılar (sırasıyla): şifre için malloc/strcpy/free kaldı mı?
ks 'objdump -d --no-show-raw-insn --disassemble=main target/kullan_sonra_birak2_O2 | grep -oE "(call|jmp) +[0-9a-f]+ <[^>]+>" | grep -oE "<[^>]+>"'

# 6) GCC'nin statik çözümleyicisi (-fanalyzer) bu örneği yakalıyor mu?
k gcc -Wall -Wextra -fanalyzer -O0 -g -o target/kullan_sonra_birak2_fa c/kullan_sonra_birak2.c

# 7) Derleyicinin çalışma anı denetleyicisi AddressSanitizer ne diyor? (ilk 3 satır)
k gcc -fsanitize=address -O0 -g -o target/kullan_sonra_birak2_asan c/kullan_sonra_birak2.c
ks './target/kullan_sonra_birak2_asan 2>&1 | grep -E "ERROR|READ of size|isim =" | sed -E "s/0x[0-9a-f]+/0x.../g"; echo "çıkış kodu: ${PIPESTATUS[0]}"'

# Rust varyantları
cd rust
# 8) Ana deneyin aynısı: derlenmez, çalıştırılacak dosya da oluşmaz
k cargo build --bin orijinal
k ls target/debug/orijinal
# 9) İade başka fonksiyonda (C'deki kullan_sonra_birak2 gibi): yine derlenmez
k cargo build --bin fonksiyon
# 10) Doğru sıra: önce kullan, sonra iade et
k cargo run --bin sira
# 11) Derleyicinin önerisi (clone) 3. satıra olduğu gibi uygulanınca
k cargo run --bin klon
# 12) Önerinin anlamı: referans artık isim'in belleğine mi, bir kopyaya mı bakıyor?
k cargo run --bin klon_adres
