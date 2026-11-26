#@ video: 004-bit-ve-bayt (ek deney: kanıtı ekrana taşıyan adımlar; ana deney mufredat-ornekleri/004-bit-ve-bayt)
#@ degisken: 3. adımdaki bellek adresleri (yığın adresi; her çalıştırmada değişir); 4. ve 11. adımlardaki strace satırlarında dosya tanımlayıcı numarası ortama bağlı olabilir. Öbür değerler aynı imajda ve x86-64 bir makinede değişmez.
rm -rf calisma && mkdir -p calisma/bit-bayt
cp -r ../mufredat-ornekleri/004-bit-ve-bayt/Cargo.toml ../mufredat-ornekleri/004-bit-ve-bayt/Cargo.lock ../mufredat-ornekleri/004-bit-ve-bayt/src calisma/bit-bayt/
cp -r calisma/bit-bayt calisma/kendin-dene
cp -r ek calisma/
cd calisma/ek
# 1) Tam sayı tipleri: kaç bayt, kaç bit; bayt değişmezi b'A' bir u8
k cargo run -q --bin tipler
# 2) Konumsal sayı sistemi: 1_000_000'un onluk, ikilik ve onaltılık basamaklarının ağırlıkları
k cargo run -q --bin basamaklar
# 3) Bellekteki dört bayt ve adresleri (adresler değişken; baytlar ana deneyin "bellekte" satırıyla aynı)
k cargo run -q --bin bellek
# 4) Bu makinenin bayt sırasını kim biliyor? lscpu çekirdeğe soruyor; derleyici hedefin özelliği olarak biliyor
ks 'lscpu | grep -E "^(Architecture|Byte Order)"'
ks 'strace -e trace=openat lscpu 2>&1 >/dev/null | grep byteorder'
k cat /sys/kernel/cpu_byteorder
ks 'rustc --print cfg | grep -E "target_(arch|endian)"'
# Rust'ın derleyebildiği bir big-endian hedef (bu makinede çalışmaz; yalnız hedefin özellikleri yazdırılır)
ks 'rustc --print cfg --target s390x-unknown-linux-gnu | grep -E "target_(arch|endian)"'
# 5) Standart kütüphanenin kaynağı (nightly-2026-11-20 ile gelen rust-src; kararlı araç zincirinde rust-src yok, 1.100 ile birebir aynılığı doğrulanmadı)
R="$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library"
# to_le_bytes ve to_be_bytes: önce sırayı hedefe göre çevir, sonra bellekteki hâli ver
k sed -n 4105,4107p "$R/core/src/num/uint_macros.rs"
k sed -n 4085,4087p "$R/core/src/num/uint_macros.rs"
# to_ne_bytes: sayının bellekteki hâli, olduğu gibi
k sed -n 4142,4146p "$R/core/src/num/uint_macros.rs"
# to_le: little-endian hedefte hiçbir şey yapmaz, big-endian hedefte baytları ters çevirir (seçim derleme zamanında)
k sed -n 986,989p "$R/core/src/num/uint_macros.rs"
k sed -n 1007,1012p "$R/core/src/num/uint_macros.rs"
k sed -n 979,984p "$R/core/src/num/uint_macros.rs"
# 6) Aynı dört bayt ters sırayla okunursa
k cargo run -q --bin ters_okuma
# 7) ELF başlığı (003'teki bağ): ilk 20 bayt, file ve readelf'in çözümü, elf.h'deki tanımlar, Rust ile çözüm
cd ../bit-bayt
k cargo build -q
ks 'od -A d -t x1 -N 20 target/debug/bit_bayt'
ks 'file target/debug/bit_bayt | cut -d, -f1-2'
ks 'readelf -h target/debug/bit_bayt | grep -E "Data|Type|Machine"'
ks 'grep -n -E "define (EI_NIDENT|EI_DATA|ELFDATA2LSB|ELFDATA2MSB|ET_DYN|EM_X86_64)[[:space:]]" /usr/include/elf.h'
ks 'grep -n "typedef uint16_t Elf64_Half" /usr/include/elf.h'
k sed -n 81,85p /usr/include/elf.h
cd ../ek
k cargo run -q --bin elf_makine -- ../bit-bayt/target/debug/bit_bayt
# 7b) Dosyanın bir kopyasında yalnız 6. baytı 01'den 02'ye (big-endian) çevir: araçlar aynı baytları büyüğü başta okur
cd ../bit-bayt
k cp target/debug/bit_bayt kopya
ks 'printf "\x02" | dd of=kopya bs=1 seek=5 conv=notrunc status=none'
ks 'od -A d -t x1 -N 20 kopya'
ks 'file kopya | cut -d, -f1-2'
ks 'readelf -h kopya | grep -E "Data|Type|Machine"'
# 8) Kendin dene: 1_000_000 yerine başka bir sayı (örnek: 2026); baytları önce kâğıtta tahmin edin
cd ../kendin-dene
k sed -i "s/1_000_000/2026/" src/main.rs
ks 'grep -n "let n" src/main.rs'
k cargo run -q
# 9) Ekrandaki alıntı kartları için belge satırları (aynı nightly rust-src; önceki adımların satır numaraları değişmesin diye sona eklendi)
# to_be_bytes ve to_le_bytes belgesi: büyüğü başta sıraya "ağ" bayt sırası da deniyor
k sed -n 4069,4070p "$R/core/src/num/uint_macros.rs"
k sed -n 4089,4090p "$R/core/src/num/uint_macros.rs"
# to_ne_bytes belgesi: makinenin kendi sırası; taşınabilir kod için to_be_bytes ya da to_le_bytes
k sed -n 4109,4114p "$R/core/src/num/uint_macros.rs"
# cfg_select! belgesi: seçim derleme zamanında yapılır, öbür dal derlenmez
k sed -n 199,203p "$R/core/src/macros/mod.rs"
# u8::is_ascii: ASCII aralığı 0..=127
k sed -n 639,641p "$R/core/src/num/mod.rs"
# std::fmt belgesi: # bayrağı 0x öneki ekler; önek toplam genişliğe dahildir
k sed -n 203p "$R/alloc/src/fmt.rs"
k sed -n 216,219p "$R/alloc/src/fmt.rs"
# 10) Kopyada readelf: uyarılar (stderr) atılınca kalan üç satır; ekrandaki readelf çıktısı bu kayıttan gelir (önceki satırların numarası değişmesin diye sona eklendi)
cd ../bit-bayt
ks 'readelf -h kopya 2>/dev/null | grep -E "Data|Type|Machine"'
# 11) lscpu bayt sırasını çekirdeğin dosyasından okuyor: dosyanın açılması ve okunan içerik
ks 'strace -e trace=openat,read lscpu 2>&1 >/dev/null | grep -A1 byteorder'
