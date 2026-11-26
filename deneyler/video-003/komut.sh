#@ video: 003-programin-yolculugu (ek deney: kanıtı ekrana taşıyan adımlar; ana deney mufredat-ornekleri/003-programin-yolculugu)
#@ degisken: metnin ikili dosyadaki konumu (grep -b ofseti), strace -k yığın izlerindeki adresler ve rt_sigaction satırlarındaki işleyici adresi derleyici ve kütüphane sürümüne bağlı; strace -P iletisindeki süreç kimliği; strace -c süreleri; çağrı sayıları ve sırası çekirdeğe ve C kütüphanesine bağlı
rm -rf calisma && mkdir -p calisma/merhaba
cp -r ../mufredat-ornekleri/003-programin-yolculugu/Cargo.toml ../mufredat-ornekleri/003-programin-yolculugu/Cargo.lock ../mufredat-ornekleri/003-programin-yolculugu/src calisma/merhaba/
cp -r satir-sonu-yok calisma/
cd calisma/merhaba
k cargo build --release -q
# 1) ELF başlığının ilk alanları: readelf ilk 16 baytı çözüyor (sihirli sayı, 64 bit, little-endian, sürüm, işletim sistemi ABI'si) ve dosya türünü yazıyor
ks 'readelf -h target/release/merhaba | head -n 8'
# 2) Programın yazdığı 17 bayt: onaltılık ve sekizlik (strace'in \303\274 yazımı sekizliktir)
ks './target/release/merhaba | od -A d -t x1z'
ks './target/release/merhaba | od -A d -c'
# 3) Aynı 17 bayt ikili dosyanın içinde (satır sonu dahil; konum derleyici sürümüne bağlı)
ks 'grep -boa "Merhaba" target/release/merhaba'
OFS=$(grep -boa "Merhaba" target/release/merhaba | head -n 1 | cut -d: -f1)
k od -A d -t x1z -j "$OFS" -N 17 target/release/merhaba
# 4) println!'in genişlediği standart kütüphane fonksiyonu dosyanın içinde
ks 'nm -C target/release/merhaba | grep " std::io::stdio::_print$"'
# 5) Hangi dosyalar açılıyor? İlk üçü yükleyiciden, dördüncüsü Rust'ın yığın taşması hazırlığından (8. adım)
ks 'strace -e trace=openat ./target/release/merhaba > /dev/null'
# 6) Tam döküm, yalnız çağrı adları ve sıra (adresler ve dönüş değerleri yok)
ks 'strace ./target/release/merhaba 2>&1 >/dev/null | cut -d"(" -f1 | cat -n'
# 7) Özet tablo exit_group'u saymaz (bu çağrı geri dönmez); tam dökümde ise bir kez geçer
ks 'strace -c ./target/release/merhaba 2>&1 >/dev/null | tail -n 1'
ks 'strace ./target/release/merhaba 2>&1 >/dev/null | grep -c "^exit_group"'
# 8) Çağrılar kimden geliyor? strace -k her çağrının yığın izini yazar
ks 'strace -k -e trace=poll ./target/release/merhaba > /dev/null'
ks 'strace -k -e trace=openat -P /proc/self/maps ./target/release/merhaba > /dev/null'
ks 'strace -k -e trace=write ./target/release/merhaba > /dev/null'
# Özet: her çağrı anında yığında ne var? main = Rust'ın çalışma zamanı ve bizim kodumuz; _start = main döndükten sonra; yükleyici = programın kendi kodu henüz başlamamış
ks 'strace -k -o /tmp/yigin-izi.txt ./target/release/merhaba > /dev/null; python3 ../../yigin_ozeti.py /tmp/yigin-izi.txt'
# 9) Kendin dene: satır sonu olmadan print! (write ne zaman ve kaç bayt?)
cd ../satir-sonu-yok
k cat src/main.rs
k cargo build --release -q
ks 'strace -e trace=write ./target/release/satir_sonu_yok > /dev/null'
# 10) Standart kütüphanenin kaynağından üç kesit (nightly-2026-11-20 ile gelen rust-src; kararlı araç zincirinde rust-src yok, 1.100 ile birebir aynılığı doğrulanmadı)
R="$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library"
# println! makrosu: standart kütüphanenin _print fonksiyonuna, sonuna satır sonu eklenmiş biçimlendirmeyle genişler
k sed -n 143,150p "$R/std/src/macros.rs"
# Rust'ın çalışma zamanı hazırlığı (Linux): 0-2 numaralı tanımlayıcıların denetimi (poll), SIGPIPE, yığın taşması işleyicisi
k sed -n 22,38p "$R/std/src/sys/pal/unix/mod.rs"
# Standart çıktı: LineWriter (satır tamponlu); FIXME notu terminale bağlı olup olmamaya göre seçim yapılmadığını söylüyor
k sed -n 610,615p "$R/std/src/io/stdio.rs"
# 11) Rust'ın sinyal ayarları: SIGPIPE yok sayılır; SIGSEGV ve SIGBUS için önce mevcut ayar sorulur (NULL), sonra işleyici kurulur (adres değişken)
cd ../merhaba
ks 'strace -e trace=rt_sigaction ./target/release/merhaba 2>&1 >/dev/null | cut -d"{" -f1-2 | cut -d, -f1-2'
# 12) Profillerin derleyiciye verdiği seçenekler: release optimizasyon düzeyi 3; dev hata ayıklama bilgisi 2 (derleme komutunu görmek için önce temizlenir)
cargo clean -q
ks 'cargo build --release -v 2>&1 | grep -o -- "-C opt-level=[0-9]"'
ks 'cargo build -v 2>&1 | grep -o -- "-C debuginfo=[0-9]"'
