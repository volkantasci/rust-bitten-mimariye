# Müfredat örnekleri — doğrulama kaydı

Bu dosya `_araclar/dogrulama_uret.py` tarafından üretilir; elle düzenlenmez.
Her bölüm: örnek yolu, kaynaklar, toolchain, çalıştırılan betik ve konteynerdeki gerçek çıktı.

Yeniden üretmek için (proje kökünden):

```bash
deneyler/mufredat-ornekleri/calistir.sh                 # tüm örnekler
deneyler/mufredat-ornekleri/calistir.sh 004-bit-ve-bayt # tek örnek
```

Adres, süre ve iş parçacığı sırası gibi değerler her çalıştırmada değişir; bu örnekler
aşağıda "Değişken çıktı" diye işaretlidir. Müfredatta gösterilen değerler bu kayıttaki
çalıştırmadan kopyalanmıştır: `Rust/mufredat.md`, `_araclar/mufredat_uret.py` ile bu
dizindeki `cikti.txt` dosyalarından üretilir ve `_araclar/kontrol.py` eşleşmeyi denetler.
Bilerek çöken deneyler ana makineye çekirdek dökümü göndermesin diye `RLIMIT_CORE=1` ile
çalışır (`_araclar/ortak.sh`). Ortak projeler (Ambar, Kapı, Minik) `ortak-projeler/` altındadır.

## Ortam (son çalıştırma)

```text
imaj: localhost/nkl-rust:1.100.0  kimlik: c34856ab12905ddd83303fd3d5db1900e10561c7326ff52151295e7243cb3f5f
tarih: 2026-11-20T16:38:17Z
rustc 1.100.0 (0ef1bc386 2026-11-09)
binary: rustc
commit-hash: 0ef1bc386059596dada8fb65b50425aab2ea637a
commit-date: 2026-11-09
host: x86_64-unknown-linux-gnu
release: 1.100.0
LLVM version: 23.1.5
cargo 1.100.0 (3acc79c0c 2026-11-05)
rustc 1.102.0-nightly (6905905bb 2026-11-19)
miri 0.1.0 (6905905bb7 2026-11-19)
gcc (Debian 14.2.0-19) 14.2.0
Model name:                              AMD Ryzen 5 5600 6-Core Processor
L1d cache:                               192 KiB (6 instances)
L1i cache:                               192 KiB (6 instances)
L2 cache:                                3 MiB (6 instances)
L3 cache:                                32 MiB (1 instance)
çekirdek: 7.2.15-1-omarchy
sayfa boyutu: 4096
```

## İçindekiler

- [001-neden-rust-neden-bilim](#001-neden-rust-neden-bilim)
- [002-laboratuvar-kurulumu](#002-laboratuvar-kurulumu)
- [003-programin-yolculugu](#003-programin-yolculugu)
- [004-bit-ve-bayt](#004-bit-ve-bayt)
- [005-tam-sayilar](#005-tam-sayilar)
- [006-kayan-nokta](#006-kayan-nokta)
- [007-metin-ve-utf8](#007-metin-ve-utf8)
- [008-islemci-ve-assembly](#008-islemci-ve-assembly)
- [009-bellek-hiyerarsisi](#009-bellek-hiyerarsisi)
- [010-yigin-ve-obek](#010-yigin-ve-obek)
- [011-sanal-bellek](#011-sanal-bellek)
- [012-isletim-sistemi-ve-sistem-cagrilari](#012-isletim-sistemi-ve-sistem-cagrilari)
- [013-derleme-asamalari](#013-derleme-asamalari)
- [014-baglama-ve-elf](#014-baglama-ve-elf)
- [015-derleyici-hatalarini-okumak](#015-derleyici-hatalarini-okumak)
- [016-degiskenler-ve-degistirilebilirlik](#016-degiskenler-ve-degistirilebilirlik)
- [017-skaler-ve-bilesik-tipler](#017-skaler-ve-bilesik-tipler)
- [018-ifadeler-ve-tip-cikarimi](#018-ifadeler-ve-tip-cikarimi)
- [019-kosullar-ve-donguler](#019-kosullar-ve-donguler)
- [020-fonksiyonlar-ve-cagri-yigini](#020-fonksiyonlar-ve-cagri-yigini)
- [021-ozyineleme](#021-ozyineleme)
- [022-bellek-hatalarinin-anatomisi](#022-bellek-hatalarinin-anatomisi)
- [023-bellek-yonetimi-stratejileri](#023-bellek-yonetimi-stratejileri)
- [024-sahiplik-kurallari](#024-sahiplik-kurallari)
- [025-kopyalama-ve-klonlama](#025-kopyalama-ve-klonlama)
- [026-odunc-alma](#026-odunc-alma)
- [027-dilimler-ve-str](#027-dilimler-ve-str)
- [028-yasam-sureleri](#028-yasam-sureleri)
- [029-odunc-denetleyicinin-zihni](#029-odunc-denetleyicinin-zihni)
- [030-structlar-ve-bellek-yerlesimi](#030-structlar-ve-bellek-yerlesimi)
- [031-metotlar-ve-impl](#031-metotlar-ve-impl)
- [032-enumlar-ve-toplam-tipler](#032-enumlar-ve-toplam-tipler)
- [033-option-ve-null](#033-option-ve-null)
- [034-oruntu-esleme](#034-oruntu-esleme)
- [035-tiplerin-matematigi](#035-tiplerin-matematigi)
- [036-panik-ve-kurtarilamaz-hatalar](#036-panik-ve-kurtarilamaz-hatalar)
- [037-result-ve-soru-isareti](#037-result-ve-soru-isareti)
- [038-hata-tipleri-tasarlamak](#038-hata-tipleri-tasarlamak)
- [039-karmasiklik-analizi](#039-karmasiklik-analizi)
- [040-vec-dinamik-dizi](#040-vec-dinamik-dizi)
- [041-hash-tablolari](#041-hash-tablolari)
- [042-agaclar-ve-btreemap](#042-agaclar-ve-btreemap)
- [043-siralama-ve-arama](#043-siralama-ve-arama)
- [044-kuyruk-yigin-ve-graflar](#044-kuyruk-yigin-ve-graflar)
- [045-onbellek-dostu-veri-yapilari](#045-onbellek-dostu-veri-yapilari)
- [046-traitler-davranis-sozlesmesi](#046-traitler-davranis-sozlesmesi)
- [047-jenerikler](#047-jenerikler)
- [048-monomorfizasyon](#048-monomorfizasyon)
- [049-trait-nesneleri-ve-vtable](#049-trait-nesneleri-ve-vtable)
- [050-statik-ve-dinamik-dagitim](#050-statik-ve-dinamik-dagitim)
- [051-standart-traitler-ve-donusumler](#051-standart-traitler-ve-donusumler)
- [052-closurelar](#052-closurelar)
- [053-iterator-protokolu](#053-iterator-protokolu)
- [054-sifir-maliyetli-soyutlama](#054-sifir-maliyetli-soyutlama)
- [055-fonksiyonel-desenler](#055-fonksiyonel-desenler)
- [056-moduller-ve-gorunurluk](#056-moduller-ve-gorunurluk)
- [057-crateler-ve-bagimliliklar](#057-crateler-ve-bagimliliklar)
- [058-tedarik-zinciri-guvenligi](#058-tedarik-zinciri-guvenligi)
- [059-birim-ve-entegrasyon-testleri](#059-birim-ve-entegrasyon-testleri)
- [060-ozellik-tabanli-test-ve-fuzzing](#060-ozellik-tabanli-test-ve-fuzzing)
- [061-clippy-rustfmt-ve-ci](#061-clippy-rustfmt-ve-ci)
- [062-box-ve-ozyinelemeli-tipler](#062-box-ve-ozyinelemeli-tipler)
- [063-rc-ve-arc-referans-sayma](#063-rc-ve-arc-referans-sayma)
- [064-ic-degistirilebilirlik](#064-ic-degistirilebilirlik)
- [065-bellek-ayiricilar](#065-bellek-ayiricilar)
- [066-eszamanlilik-ve-is-parcaciklari](#066-eszamanlilik-ve-is-parcaciklari)
- [067-veri-yarisi-ve-send-sync](#067-veri-yarisi-ve-send-sync)
- [068-mutex-ve-kilitler](#068-mutex-ve-kilitler)
- [069-kilitlenme-ve-esgudum](#069-kilitlenme-ve-esgudum)
- [070-kanallar-ve-mesajlasma](#070-kanallar-ve-mesajlasma)
- [071-atomikler-ve-bellek-siralamasi](#071-atomikler-ve-bellek-siralamasi)
- [072-veri-paralelligi](#072-veri-paralelligi)
- [073-surecler](#073-surecler)
- [074-dosyalar-ve-tamponlama](#074-dosyalar-ve-tamponlama)
- [075-dayaniklilik-ve-fsync](#075-dayaniklilik-ve-fsync)
- [076-borular-ve-sinyaller](#076-borular-ve-sinyaller)
- [077-ag-katmanlari-ve-soketler](#077-ag-katmanlari-ve-soketler)
- [078-tcp-akis-dogasi](#078-tcp-akis-dogasi)
- [079-udp-ve-dns](#079-udp-ve-dns)
- [080-http-elle](#080-http-elle)
- [081-protokol-tasarimi](#081-protokol-tasarimi)
- [082-serilestirme](#082-serilestirme)
- [083-neden-async](#083-neden-async)
- [084-epoll-ve-olay-dongusu](#084-epoll-ve-olay-dongusu)
- [085-future-durum-makinesi](#085-future-durum-makinesi)
- [086-kendi-yurutucumuz](#086-kendi-yurutucumuz)
- [087-pin-ve-kendine-referans](#087-pin-ve-kendine-referans)
- [088-tokio-ile-async](#088-tokio-ile-async)
- [089-async-tuzaklari](#089-async-tuzaklari)
- [090-unsafe-ne-demek](#090-unsafe-ne-demek)
- [091-tanimsiz-davranis-ve-miri](#091-tanimsiz-davranis-ve-miri)
- [092-guvenli-soyutlama-yazmak](#092-guvenli-soyutlama-yazmak)
- [093-ffi-c-ile-konusmak](#093-ffi-c-ile-konusmak)
- [094-bellek-esleme](#094-bellek-esleme)
- [095-bildirimsel-makrolar](#095-bildirimsel-makrolar)
- [096-makro-genisletme](#096-makro-genisletme)
- [097-prosedurel-makrolar](#097-prosedurel-makrolar)
- [098-lexer-ve-parser](#098-lexer-ve-parser)
- [099-hir-ve-tip-denetimi](#099-hir-ve-tip-denetimi)
- [100-mir-ve-odunc-denetimi](#100-mir-ve-odunc-denetimi)
- [101-llvm-ve-optimizasyon](#101-llvm-ve-optimizasyon)
- [102-kod-uretimi-ve-artimli-derleme](#102-kod-uretimi-ve-artimli-derleme)
- [103-olcum-bilimi](#103-olcum-bilimi)
- [104-profil-cikarma](#104-profil-cikarma)
- [105-dallanma-tahmini](#105-dallanma-tahmini)
- [106-simd-ve-vektorlestirme](#106-simd-ve-vektorlestirme)
- [107-derleyici-ayarlari-lto-pgo](#107-derleyici-ayarlari-lto-pgo)
- [108-tip-gudumlu-tasarim](#108-tip-gudumlu-tasarim)
- [109-hata-sinirlari-ve-katmanlar](#109-hata-sinirlari-ve-katmanlar)
- [110-api-tasarimi](#110-api-tasarimi)
- [111-test-edilebilir-mimari](#111-test-edilebilir-mimari)
- [112-gozlemlenebilirlik](#112-gozlemlenebilirlik)
- [113-ajan-kodunu-okumak](#113-ajan-kodunu-okumak)
- [114-inceleme-listesi](#114-inceleme-listesi)
- [115-araclarla-dogrulama](#115-araclarla-dogrulama)
- [116-dogru-soruyu-sormak](#116-dogru-soruyu-sormak)
- [117-ambar-tasarim](#117-ambar-tasarim)
- [118-ambar-depolama-motoru](#118-ambar-depolama-motoru)
- [119-ambar-ag-protokolu](#119-ambar-ag-protokolu)
- [120-ambar-eszamanlilik-ve-dayaniklilik](#120-ambar-eszamanlilik-ve-dayaniklilik)
- [121-ambar-performans](#121-ambar-performans)
- [122-http-ayristirici](#122-http-ayristirici)
- [123-http-sunucu](#123-http-sunucu)
- [124-http-eszamanlilik-ve-yuk](#124-http-eszamanlilik-ve-yuk)
- [125-yorumlayici-lexer](#125-yorumlayici-lexer)
- [126-yorumlayici-parser](#126-yorumlayici-parser)
- [127-yorumlayici-degerlendirici](#127-yorumlayici-degerlendirici)
- [128-yorumlayici-bytecode-vm](#128-yorumlayici-bytecode-vm)
- [129-yolun-sonu](#129-yolun-sonu)

## 001-neden-rust-neden-bilim

- Video: `001-neden-rust-neden-bilim`
- Örnek yolu: `deneyler/mufredat-ornekleri/001-neden-rust-neden-bilim/`
- Kaynaklar: `c/kullan_sonra_birak.c`, `c/kullan_sonra_birak2.c`, `rust/Cargo.toml`, `rust/src/main.rs`
- Toolchain: stable 1.100.0, gcc 14.2.0
- Son çalıştırma (UTC): 2026-11-20T16:03:27Z
- Değişken çıktı: valgrind satırlarındaki süreç kimliği (==N==) ve adresler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
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
```

- Gerçek çıktı (kısaltılmış; 113 satırın tamamı `cikti.txt` içinde):

```text
$ gcc -Wall -Wextra -O0 -g -o target/kullan_sonra_birak c/kullan_sonra_birak.c
c/kullan_sonra_birak.c: In function ‘main’:
c/kullan_sonra_birak.c:13:5: warning: pointer ‘isim’ used after ‘free’ [-Wuse-after-free]
   13 |     printf("isim = %s\n", isim);     /* iade edilmiş belleği okuyoruz */
      |     ^~~~~~~~~~~~~~~~~~~~~~~~~~~
c/kullan_sonra_birak.c:8:5: note: call to ‘free’ here
    8 |     free(isim);                      /* bellek iade edildi */
      |     ^~~~~~~~~~
$ ./target/kullan_sonra_birak
isim = gizli-123
$ gcc -Wall -Wextra -O0 -g -o target/kullan_sonra_birak2 c/kullan_sonra_birak2.c
$ ./target/kullan_sonra_birak2
isim = gizli-123
$ valgrind -q ./target/kullan_sonra_birak2 2>&1 | head -14
==78== Invalid read of size 1
==78==    at 0x484DC86: strlen (vg_replace_strmem.c:505)
==78==    by 0x48C627F: __printf_buffer (vfprintf-process-arg.c:435)
==78==    by 0x48C6F10: __vfprintf_internal (vfprintf-internal.c:1544)
==78==    by 0x48BB92A: printf (printf.c:33)
==78==    by 0x1091E6: main (kullan_sonra_birak2.c:16)
==78==  Address 0x4a59040 is 0 bytes inside a block of size 16 free'd
==78==    at 0x484787F: free (vg_replace_malloc.c:989)
==78==    by 0x109170: temizle (kullan_sonra_birak2.c:6)
==78==    by 0x1091A6: main (kullan_sonra_birak2.c:11)
==78==  Block was alloc'd at
==78==    at 0x4844818: malloc (vg_replace_malloc.c:446)
==78==    by 0x109185: main (kullan_sonra_birak2.c:9)
==78== 
$ valgrind -q --error-exitcode=1 ./target/kullan_sonra_birak2
==80== Invalid read of size 1
==80==    at 0x484DC86: strlen (vg_replace_strmem.c:505)
==80==    by 0x48C627F: __printf_buffer (vfprintf-process-arg.c:435)
==80==    by 0x48C6F10: __vfprintf_internal (vfprintf-internal.c:1544)
==80==    by 0x48BB92A: printf (printf.c:33)
==80==    by 0x1091E6: main (kullan_sonra_birak2.c:16)
==80==  Address 0x4a59040 is 0 bytes inside a block of size 16 free'd
==80==    at 0x484787F: free (vg_replace_malloc.c:989)
==80==    by 0x109170: temizle (kullan_sonra_birak2.c:6)
==80==    by 0x1091A6: main (kullan_sonra_birak2.c:11)
==80==  Block was alloc'd at
==80==    at 0x4844818: malloc (vg_replace_malloc.c:446)
==80==    by 0x109185: main (kullan_sonra_birak2.c:9)
==80== 
==80== Invalid read of size 1
==80==    at 0x484DC94: strlen (vg_replace_strmem.c:505)
… (44 satır atlandı)
isim = Ferris
… (3 satır atlandı)
error[E0505]: cannot move out of `isim` because it is borrowed
 --> src/main.rs:4:10
  |
2 |     let isim = String::from("Ferris");
  |         ---- binding `isim` declared here
3 |     let referans = &isim;            // isim'e bakan bir ödünç (referans)
  |                    ----- borrow of `isim` occurs here
4 |     drop(isim);                      // belleği iade etmeye çalış
  |          ^^^^ move out of `isim` occurs here
5 |     println!("isim = {referans}");   // iade edilmiş belleği okumak olurdu
  |                       -------- borrow later used here
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let referans = &isim.clone();            // isim'e bakan bir ödünç (referans)
  |                         ++++++++

For more information about this error, try `rustc --explain E0505`.
error: could not compile `kullan_sonra_birak` (bin "kullan_sonra_birak") due to 1 previous error
[çıkış kodu: 101]
```

## 002-laboratuvar-kurulumu

- Video: `002-laboratuvar-kurulumu`
- Örnek yolu: `deneyler/mufredat-ornekleri/002-laboratuvar-kurulumu/`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:27Z
- Değişken çıktı: cargo'nun "Finished ... in X s" süresi
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Konteynerin içindeyiz: araç sürümleri ve çekirdek
k rustc --version
k cargo --version
k uname -r
ks 'grep PRETTY_NAME /etc/os-release'
# Sıfırdan bir proje: cargo new hangi edition'ı seçiyor?
rm -rf calisma && mkdir -p calisma && cd calisma
k cargo new merhaba
k cat merhaba/Cargo.toml
k cat merhaba/src/main.rs
cd merhaba
k cargo run
# Hangi edition'lar var? (geçersiz bir değer verip listeyi cargo'ya söyletiyoruz)
cd ..
k cargo new --edition 2027 deneme
ks 'rustc --help | grep -A3 -- "--edition"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ rustc --version
rustc 1.100.0 (0ef1bc386 2026-11-09)
$ cargo --version
cargo 1.100.0 (3acc79c0c 2026-11-05)
$ uname -r
7.2.15-1-omarchy
$ grep PRETTY_NAME /etc/os-release
PRETTY_NAME="Debian GNU/Linux 13 (trixie)"
$ cargo new merhaba
    Creating binary (application) `merhaba` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ cat merhaba/Cargo.toml
[package]
name = "merhaba"
version = "0.1.0"
edition = "2024"

[dependencies]
$ cat merhaba/src/main.rs
fn main() {
    println!("Hello, world!");
}
$ cargo run
   Compiling merhaba v0.1.0 (/work/mufredat-ornekleri/002-laboratuvar-kurulumu/calisma/merhaba)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/merhaba`
Hello, world!
$ cargo new --edition 2027 deneme
error: invalid value '2027' for '--edition <YEAR>'
  [possible values: 2015, 2018, 2021, 2024]

  tip: a similar value exists: '2024'

For more information, try '--help'.
[çıkış kodu: 1]
$ rustc --help | grep -A3 -- "--edition"
        --edition <2015|2018|2021|2024|future>
                        Specify which edition of the compiler to use when
                        compiling code. The default is 2015 and the latest
                        stable edition is 2024.
```

## 003-programin-yolculugu

- Video: `003-programin-yolculugu`
- Örnek yolu: `deneyler/mufredat-ornekleri/003-programin-yolculugu/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:28Z
- Değişken çıktı: strace -c süreleri; ikili dosya boyutu derleyici sürümüne bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build --release -q
ks 'stat -c "%s bayt  %n" target/release/merhaba'
k file target/release/merhaba
# Dosyanın ilk 16 baytı: ELF imzası
k od -A x -t x1z -N 16 target/release/merhaba
k ./target/release/merhaba
# Ekrana yazmak için çekirdekten yardım isteniyor: write sistem çağrısı
ks 'strace -e trace=write ./target/release/merhaba'
# Tek satırlık bir program kaç sistem çağrısı yapıyor?
ks 'strace -c ./target/release/merhaba > /dev/null'
# main fonksiyonu makine kodunda
ks 'nm target/release/merhaba | grep -i "merhaba.*main"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build --release -q
$ stat -c "%s bayt  %n" target/release/merhaba
445024 bayt  target/release/merhaba
$ file target/release/merhaba
target/release/merhaba: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=d0cd1426a8c75ffeaaab228d564b6a1a6a7a19d9, not stripped
$ od -A x -t x1z -N 16 target/release/merhaba
000000 7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00  >.ELF............<
000010
$ ./target/release/merhaba
Merhaba, dünya!
$ strace -e trace=write ./target/release/merhaba
write(1, "Merhaba, d\303\274nya!\n", 17Merhaba, dünya!
) = 17
+++ exited with 0 +++
$ strace -c ./target/release/merhaba > /dev/null
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ------------------
 26.39    0.000071           5        13           mmap
 12.64    0.000034           8         4           openat
 12.27    0.000033           6         5           read
  9.29    0.000025           5         5           mprotect
  6.32    0.000017           3         5           rt_sigaction
  5.20    0.000014           3         4           close
  4.09    0.000011           3         3           brk
  3.72    0.000010           2         4           fstat
  2.23    0.000006           6         1           munmap
  2.23    0.000006           3         2           sigaltstack
  2.23    0.000006           3         2           prlimit64
  1.49    0.000004           4         1           write
  1.49    0.000004           2         2           pread64
  1.49    0.000004           4         1         1 access
  1.49    0.000004           4         1           gettid
  1.49    0.000004           4         1           sched_getaffinity
  1.49    0.000004           4         1           set_tid_address
  1.12    0.000003           3         1           poll
  1.12    0.000003           3         1           arch_prctl
  0.74    0.000002           2         1           set_robust_list
  0.74    0.000002           2         1           getrandom
  0.74    0.000002           2         1           rseq
  0.00    0.000000           0         1           execve
------ ----------- ----------- --------- --------- ------------------
100.00    0.000269           4        61         1 total
$ nm target/release/merhaba | grep -i "merhaba.*main"
0000000000014b30 t _RNvCsjEVuznjrSGG_7merhaba4main
```

## 004-bit-ve-bayt

- Video: `004-bit-ve-bayt`
- Örnek yolu: `deneyler/mufredat-ornekleri/004-bit-ve-bayt/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:28Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
ks 'lscpu | grep "Byte Order"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
onluk      : 1000000
ikilik     : 00000000000011110100001001000000
onaltılık  : 0x000f4240
bellekte   : [40, 42, 0f, 00]
küçük uçlu : [40, 42, 0f, 00]
büyük uçlu : [00, 0f, 42, 40]
'A' baytı  : 65 = 0x41 = 01000001
$ lscpu | grep "Byte Order"
Byte Order:                              Little Endian
```

## 005-tam-sayilar

- Video: `005-tam-sayilar`
- Örnek yolu: `deneyler/mufredat-ornekleri/005-tam-sayilar/`
- Kaynaklar: `Cargo.toml`, `sabit-tasma/Cargo.toml`, `sabit-tasma/src/main.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:28Z
- Değişken çıktı: panik satırındaki iş parçacığı kimliği (thread 'main' (N))
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Değer derleme anında biliniyorsa: derleyici reddeder
cd sabit-tasma && k cargo build -q; cd ..
# Değer çalışma anında: debug derlemede panik
k cargo run -q
# Release derlemede taşma denetimi kapalı: sessizce sarar
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error: this arithmetic operation will overflow
 --> src/main.rs:2:17
  |
2 |     let x: u8 = 255 + 1;
  |                 ^^^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default

error: could not compile `sabit_tasma` (bin "sabit_tasma") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q
checked_add(1)     = None
wrapping_add(1)    = 0
saturating_add(1)  = 255
overflowing_add(1) = (0, true)
-1i8 bitleri       = 11111111
i8 aralığı         = -128 ..= 127

thread 'main' (401) panicked at src/main.rs:12:41:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
$ cargo run -q --release
checked_add(1)     = None
wrapping_add(1)    = 0
saturating_add(1)  = 255
overflowing_add(1) = (0, true)
-1i8 bitleri       = 11111111
i8 aralığı         = -128 ..= 127
x + 1              = 0
```

## 006-kayan-nokta

- Video: `006-kayan-nokta`
- Örnek yolu: `deneyler/mufredat-ornekleri/006-kayan-nokta/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:28Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
warning: incorrect NaN comparison, NaN cannot be directly compared to itself
  --> src/main.rs:15:41
   |
15 |     println!("NaN == NaN         → {}", f64::NAN == f64::NAN);
   |                                         ^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(invalid_nan_comparisons)]` on by default
help: use `f32::is_nan()` or `f64::is_nan()` instead
   |
15 -     println!("NaN == NaN         → {}", f64::NAN == f64::NAN);
15 +     println!("NaN == NaN         → {}", f64::NAN.is_nan());
   |

0.1 + 0.2          = 0.30000000000000004
0.1 + 0.2 == 0.3   → false
0.1'in tam değeri  = 0.1000000000000000055511151231257827021181583404541015625
0.1 bitleri        = 0011111110111001100110011001100110011001100110011001100110011010
  işaret           = 0
  üs               = 01111111011 (1019 - 1023 = -4)
  kesir            = 1001100110011001100110011001100110011001100110011010
16777217 → f32     = 16777216
NaN == NaN         → false
1.0 / 0.0          = inf
(0.1+0.2-0.3).abs() < f64::EPSILON → true
```

## 007-metin-ve-utf8

- Video: `007-metin-ve-utf8`
- Örnek yolu: `deneyler/mufredat-ornekleri/007-metin-ve-utf8/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:28Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
A  U+0041  1 bayt  [41]
ç  U+00E7  2 bayt  [c3, a7]
ğ  U+011F  2 bayt  [c4, 9f]
€  U+20AC  3 bayt  [e2, 82, ac]
🦀  U+1F980  4 bayt  [f0, 9f, a6, 80]
"Iğdır": len() = 7 bayt, chars().count() = 5 karakter
"istanbul".to_uppercase() = "ISTANBUL"
"İ".to_lowercase()        = "i\u{307}" (2 karakter: ["U+0069", "U+0307"])
size_of::<char>()          = 4 bayt
```

## 008-islemci-ve-assembly

- Video: `008-islemci-ve-assembly`
- Örnek yolu: `deneyler/mufredat-ornekleri/008-islemci-ve-assembly/`
- Kaynaklar: `islem.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:28Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p target
# Optimizasyonlu derleme: tek komut
k rustc --edition 2024 -C opt-level=3 --crate-type=lib --emit=obj -o target/islem.o islem.rs
k objdump -d -M intel --no-show-raw-insn --disassemble=topla target/islem.o
# Döngü mü? Derleyici formüle çevirdi mi?
k objdump -d -M intel --no-show-raw-insn --disassemble=toplam target/islem.o
# Optimizasyonsuz (debug) derleme: taşma denetimi görünür
k rustc --edition 2024 -C opt-level=0 --crate-type=lib --emit=obj -o target/islem_debug.o islem.rs
k objdump -d -M intel --no-show-raw-insn -r -C --disassemble=topla target/islem_debug.o
```

- Gerçek çıktı (`cikti.txt`):

```text
$ rustc --edition 2024 -C opt-level=3 --crate-type=lib --emit=obj -o target/islem.o islem.rs
$ objdump -d -M intel --no-show-raw-insn --disassemble=topla target/islem.o

target/islem.o:     file format elf64-x86-64


Disassembly of section .text.topla:

0000000000000000 <topla>:
   0:	lea    rax,[rdi+rsi*1]
   4:	ret

Disassembly of section .text.toplam:
$ objdump -d -M intel --no-show-raw-insn --disassemble=toplam target/islem.o

target/islem.o:     file format elf64-x86-64


Disassembly of section .text.topla:

Disassembly of section .text.toplam:

0000000000000000 <toplam>:
   0:	test   rdi,rdi
   3:	je     1d <toplam+0x1d>
   5:	lea    rax,[rdi-0x1]
   9:	lea    rcx,[rdi-0x2]
   d:	mul    rcx
  10:	shld   rdx,rax,0x3f
  15:	lea    rax,[rdi+rdx*1]
  19:	dec    rax
  1c:	ret
  1d:	xor    eax,eax
  1f:	ret
$ rustc --edition 2024 -C opt-level=0 --crate-type=lib --emit=obj -o target/islem_debug.o islem.rs
$ objdump -d -M intel --no-show-raw-insn -r -C --disassemble=topla target/islem_debug.o

target/islem_debug.o:     file format elf64-x86-64


Disassembly of section .text.topla:

0000000000000000 <topla>:
   0:	push   rax
   1:	add    rdi,rsi
   4:	mov    QWORD PTR [rsp],rdi
   8:	seto   al
   b:	jo     13 <topla+0x13>
   d:	mov    rax,QWORD PTR [rsp]
  11:	pop    rcx
  12:	ret
  13:	lea    rdi,[rip+0x0]        # 1a <topla+0x1a>
			16: R_X86_64_PC32	.data.rel.ro..Lanon.15a2729f09bda4f55a112f95d75d0bf2.1-0x4
  1a:	call   QWORD PTR [rip+0x0]        # 20 <topla+0x20>
			1c: R_X86_64_GOTPCREL	core::panicking::panic_const::panic_const_add_overflow-0x4

Disassembly of section .text.toplam:
```

## 009-bellek-hiyerarsisi

- Video: `009-bellek-hiyerarsisi`
- Örnek yolu: `deneyler/mufredat-ornekleri/009-bellek-hiyerarsisi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:32Z
- Değişken çıktı: süreler donanıma ve o anki yüke bağlı (AMD Ryzen 5 5600: L1d 32 KiB, L2 512 KiB/çekirdek, L3 32 MiB)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
ks 'lscpu | grep -E "^L1d|^L2|^L3"'
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ lscpu | grep -E "^L1d|^L2|^L3"
L1d cache:                               192 KiB (6 instances)
L2 cache:                                3 MiB (6 instances)
L3 cache:                                32 MiB (1 instance)
$ cargo run -q --release
çalışma kümesi    ns/erişim
        16 KiB         1.19   (son=330)
        32 KiB         1.18   (son=2719)
        64 KiB         2.12   (son=7641)
       128 KiB         2.55   (son=15857)
       256 KiB         3.18   (son=6251)
       512 KiB         5.01   (son=61778)
      1024 KiB         8.50   (son=115268)
      2048 KiB        10.60   (son=43097)
      8192 KiB        12.58   (son=158047)
     16384 KiB        19.58   (son=1655160)
     32768 KiB        58.33   (son=1515190)
     65536 KiB        65.86   (son=2436746)
    262144 KiB        87.16   (son=3877667)
```

## 010-yigin-ve-obek

- Video: `010-yigin-ve-obek`
- Örnek yolu: `deneyler/mufredat-ornekleri/010-yigin-ve-obek/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:32Z
- Değişken çıktı: adresler her çalıştırmada değişir (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/yigin_obek
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/yigin_obek
kod    main()            : 0x5583d7018b90
statik SELAM metni       : 0x5583d70079ec
öbek   Box<u64>          : 0x5584096b1d60
öbek   String verisi     : 0x5584096b1d80
yığın  yerel değişken    : 0x7ffc52625168
5583d7000000-5583d7018000 r--p 00000000 00:1d 67978390                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
5583d7018000-5583d705b000 r-xp 00017000 00:1d 67978390                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
5583d705b000-5583d705f000 r--p 00059000 00:1d 67978390                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
5583d705f000-5583d7061000 rw-p 0005c000 00:1d 67978390                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
5584096b1000-5584096d2000 rw-p 00000000 00:00 0                          [heap]
7ffc52606000-7ffc52627000 rw-p 00000000 00:00 0                          [stack]
```

## 011-sanal-bellek

- Video: `011-sanal-bellek`
- Örnek yolu: `deneyler/mufredat-ornekleri/011-sanal-bellek/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:33Z
- Değişken çıktı: adresler (ASLR), RSS ve sayfa hatası sayıları (THP ayarına bağlı)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k getconf PAGESIZE
k cat /sys/kernel/mm/transparent_hugepage/enabled
k cargo build -q --release
k ./target/release/sanal_bellek
k ./target/release/sanal_bellek
```

- Gerçek çıktı (`cikti.txt`):

```text
$ getconf PAGESIZE
4096
$ cat /sys/kernel/mm/transparent_hugepage/enabled
[always] madvise never
$ cargo build -q --release
$ ./target/release/sanal_bellek
yığın adresi: 0x7ffe95fa7cb7   öbek adresi: 0x56267cfb3d60
başlangıç        : VmRSS      2000 kB  sayfa hatası     101
1 GiB ayrıldı    : VmRSS      2008 kB  sayfa hatası     102
sayfalara dokundu: VmRSS   1050580 kB  sayfa hatası  261223
AnonHugePages:      4096 kB   (2 MiB'lik dev sayfalarla karşılanan kısım)
toplam: 262144
$ ./target/release/sanal_bellek
yığın adresi: 0x7fffcce64467   öbek adresi: 0x560dd6644d60
başlangıç        : VmRSS      1888 kB  sayfa hatası      98
1 GiB ayrıldı    : VmRSS      1896 kB  sayfa hatası      99
sayfalara dokundu: VmRSS   1050468 kB  sayfa hatası  261220
AnonHugePages:      4096 kB   (2 MiB'lik dev sayfalarla karşılanan kısım)
toplam: 262144
```

## 012-isletim-sistemi-ve-sistem-cagrilari

- Video: `012-isletim-sistemi-ve-sistem-cagrilari`
- Örnek yolu: `deneyler/mufredat-ornekleri/012-isletim-sistemi-ve-sistem-cagrilari/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:33Z
- Değişken çıktı: süreler donanıma ve çekirdek ayarlarına bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/sistem_cagrisi
# Gerçekten her seferinde çekirdeğe gidiyor mu? 1000 çağrıyı sayalım
ks 'strace -c -e trace=getpid ./target/release/sistem_cagrisi 1000 > /dev/null'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/sistem_cagrisi
1000000 fonksiyon çağrısı :     1.42 ns/çağrı
1000000 getpid() çağrısı  :   273.30 ns/çağrı
$ strace -c -e trace=getpid ./target/release/sistem_cagrisi 1000 > /dev/null
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
100.00    0.001738           1      1000           getpid
------ ----------- ----------- --------- --------- ----------------
100.00    0.001738           1      1000           total
```

## 013-derleme-asamalari

- Video: `013-derleme-asamalari`
- Örnek yolu: `deneyler/mufredat-ornekleri/013-derleme-asamalari/`
- Kaynaklar: `Cargo.toml`, `kare.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:37Z
- Değişken çıktı: derleme ve çalışma süreleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# cargo aslında rustc'yi çağırır: -v ile tam komutu görelim
k cargo build -v
mkdir -p target
# Aynı fonksiyonun LLVM ara temsili (IR): optimizasyonsuz ve optimizasyonlu
k rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=llvm-ir -o target/kare_O0.ll kare.rs
ks "awk '/^define.*@kare/,/^}/' target/kare_O0.ll"
k rustc --edition 2024 --crate-type=lib -C opt-level=2 --emit=llvm-ir -o target/kare_O2.ll kare.rs
ks "awk '/^define.*@kare/,/^}/' target/kare_O2.ll"
# debug ve release ikililerinin hızı
k cargo build -q --release
k hyperfine -N --warmup 1 --runs 5 --style basic ./target/debug/elek ./target/release/elek
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -v
   Compiling elek v0.1.0 (/work/mufredat-ornekleri/013-derleme-asamalari)
     Running `/usr/local/rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu/bin/rustc --crate-name elek --edition=2024 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat,unused-externs-silent --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values())' -C metadata=bf9f79702f8348ae --out-dir /work/mufredat-ornekleri/013-derleme-asamalari/target/debug/build/elek/d3c967da860348f5/out -C incremental=/work/mufredat-ornekleri/013-derleme-asamalari/target/debug/incremental --force-warn=unused_crate_dependencies`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
$ rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=llvm-ir -o target/kare_O0.ll kare.rs
$ awk '/^define.*@kare/,/^}/' target/kare_O0.ll
define i32 @kare(i32 %x) unnamed_addr #0 !guid !6 {
start:
  %0 = call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %x, i32 %x)
  %_2.0 = extractvalue { i32, i1 } %0, 0
  %_2.1 = extractvalue { i32, i1 } %0, 1
  br i1 %_2.1, label %panic, label %bb1

bb1:                                              ; preds = %start
  ret i32 %_2.0

panic:                                            ; preds = %start
; call core::panicking::panic_const::panic_const_mul_overflow
  call void @_RNvNtNtCs8uuN1nrGffB_4core9panicking11panic_const24panic_const_mul_overflow(ptr align 8 @alloc_70dda244d6e4273e3e8465cd02482bcb) #3
  unreachable
}
$ rustc --edition 2024 --crate-type=lib -C opt-level=2 --emit=llvm-ir -o target/kare_O2.ll kare.rs
$ awk '/^define.*@kare/,/^}/' target/kare_O2.ll
define noundef i32 @kare(i32 noundef %x) unnamed_addr #0 !guid !4 {
start:
  %_0 = mul i32 %x, %x
  ret i32 %_0
}
$ cargo build -q --release
$ hyperfine -N --warmup 1 --runs 5 --style basic ./target/debug/elek ./target/release/elek
Benchmark 1: ./target/debug/elek
  Time (mean ± σ):     420.1 ms ±  27.8 ms    [User: 409.2 ms, System: 7.9 ms]
  Range (min … max):   403.0 ms … 469.1 ms    5 runs
 
Benchmark 2: ./target/release/elek
  Time (mean ± σ):      46.2 ms ±   3.2 ms    [User: 37.2 ms, System: 8.3 ms]
  Range (min … max):    41.9 ms …  49.4 ms    5 runs
 
Summary
  ./target/release/elek ran
    9.10 ± 0.87 times faster than ./target/debug/elek
```

## 014-baglama-ve-elf

- Video: `014-baglama-ve-elf`
- Örnek yolu: `deneyler/mufredat-ornekleri/014-baglama-ve-elf/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:37Z
- Değişken çıktı: ldd'nin yükleme adresleri (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ldd target/release/merhaba
ks 'readelf -h target/release/merhaba | grep -E "Class|Type|Machine|Entry"'
ks 'readelf -S -W target/release/merhaba | grep -E " \.(text|rodata|data|bss) "'
# Sembol adları: derleyici "isim karıştırma" (mangling) uygular
ks 'nm target/release/merhaba | grep "merhaba4main"'
ks 'nm -C target/release/merhaba | grep "merhaba::main"'
# Statik bağlama: C kütüphanesi de ikiliye gömülsün
ks 'RUSTFLAGS="-C target-feature=+crt-static" cargo build -q --release --target-dir target/statik'
k file target/statik/release/merhaba
k ldd target/statik/release/merhaba
ks 'stat -c "%s bayt  %n" target/release/merhaba target/statik/release/merhaba'
k ./target/statik/release/merhaba
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ldd target/release/merhaba
	linux-vdso.so.1 (0x00007f26ddea4000)
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f26dde13000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f26ddc1f000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f26ddea6000)
$ readelf -h target/release/merhaba | grep -E "Class|Type|Machine|Entry"
  Class:                             ELF64
  Type:                              DYN (Position-Independent Executable file)
  Machine:                           Advanced Micro Devices X86-64
  Entry point address:               0x14a10
$ readelf -S -W target/release/merhaba | grep -E " \.(text|rodata|data|bss) "
  [11] .rodata           PROGBITS        0000000000004dc0 004dc0 005594 00 AMS  0   0 16
  [15] .text             PROGBITS        0000000000014a10 013a10 03b063 00  AX  0   0 16
  [29] .data             PROGBITS        00000000000540d8 0510d8 000a48 00  WA  0   0  8
  [30] .bss              NOBITS          0000000000054b20 051b20 0000c2 00  WA  0   0  8
$ nm target/release/merhaba | grep "merhaba4main"
0000000000014b30 t _RNvCsjEVuznjrSGG_7merhaba4main
$ nm -C target/release/merhaba | grep "merhaba::main"
0000000000014b30 t merhaba::main
$ RUSTFLAGS="-C target-feature=+crt-static" cargo build -q --release --target-dir target/statik
$ file target/statik/release/merhaba
target/statik/release/merhaba: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, for GNU/Linux 3.2.0, BuildID[sha1]=6fbaffb53b017931e0f55d8f9a6583d8f38faad1, not stripped
$ ldd target/statik/release/merhaba
	statically linked
$ stat -c "%s bayt  %n" target/release/merhaba target/statik/release/merhaba
445024 bayt  target/release/merhaba
1380472 bayt  target/statik/release/merhaba
$ ./target/statik/release/merhaba
Merhaba, dünya!
```

## 015-derleyici-hatalarini-okumak

- Video: `015-derleyici-hatalarini-okumak`
- Örnek yolu: `deneyler/mufredat-ornekleri/015-derleyici-hatalarini-okumak/`
- Kaynaklar: `mut-eksik/Cargo.toml`, `mut-eksik/src/main.rs`, `tip-hatasi/Cargo.toml`, `tip-hatasi/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:37Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd tip-hatasi && k cargo build -q; cd ..
ks 'rustc --explain E0308 | head -n 16'
cd mut-eksik && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error[E0308]: mismatched types
 --> src/main.rs:2:21
  |
2 |     let sayi: i32 = "kırk iki";
  |               ---   ^^^^^^^^^^ expected `i32`, found `&str`
  |               |
  |               expected due to this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `tip_hatasi` (bin "tip_hatasi") due to 1 previous error
[çıkış kodu: 101]
$ rustc --explain E0308 | head -n 16
Expected type did not match the received type.

Erroneous code examples:

```
fn plus_one(x: i32) -> i32 {
    x + 1
}

plus_one("Not a number");
//       ^^^^^^^^^^^^^^ expected `i32`, found `&str`

if "Not a bool" {
// ^^^^^^^^^^^^ expected `bool`, found `&str`
}

$ cargo build -q
error[E0596]: cannot borrow `liste` as mutable, as it is not declared as mutable
 --> src/main.rs:3:5
  |
3 |     liste.push(1);
  |     ^^^^^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
2 |     let mut liste = Vec::new();
  |         +++

For more information about this error, try `rustc --explain E0596`.
error: could not compile `mut_eksik` (bin "mut_eksik") due to 1 previous error
[çıkış kodu: 101]
```

## 016-degiskenler-ve-degistirilebilirlik

- Video: `016-degiskenler-ve-degistirilebilirlik`
- Örnek yolu: `deneyler/mufredat-ornekleri/016-degiskenler-ve-degistirilebilirlik/`
- Kaynaklar: `golgeleme/Cargo.toml`, `golgeleme/src/main.rs`, `iki-kez-atama/Cargo.toml`, `iki-kez-atama/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:37Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd iki-kez-atama && k cargo build -q; cd ..
cd golgeleme && k cargo run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("x = {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `iki_kez_atama` (bin "iki_kez_atama") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q
girdi = 84
sayac = 3
```

## 017-skaler-ve-bilesik-tipler

- Video: `017-skaler-ve-bilesik-tipler`
- Örnek yolu: `deneyler/mufredat-ornekleri/017-skaler-ve-bilesik-tipler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:37Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
bool         boyut  1  hizalama  1
u8           boyut  1  hizalama  1
i16          boyut  2  hizalama  2
u32          boyut  4  hizalama  4
i64          boyut  8  hizalama  8
u128         boyut 16  hizalama 16
usize        boyut  8  hizalama  8
f32          boyut  4  hizalama  4
f64          boyut  8  hizalama  8
char         boyut  4  hizalama  4
()           boyut  0  hizalama  1
(u8, u32)    boyut  8  hizalama  4
[u16; 5]     boyut 10  hizalama  2

300_i32 as u8     = 44
-1_i32 as u32     = 4294967295
3.99_f64 as i32   = 3
-3.99_f64 as i32  = -3
1e20_f64 as i32   = 2147483647
f64::NAN as i32   = 0
u8::try_from(300) = Err(TryFromIntError(PosOverflow))
```

## 018-ifadeler-ve-tip-cikarimi

- Video: `018-ifadeler-ve-tip-cikarimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/018-ifadeler-ve-tip-cikarimi/`
- Kaynaklar: `ifadeler/Cargo.toml`, `ifadeler/src/main.rs`, `tip-gerekli/Cargo.toml`, `tip-gerekli/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:37Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ifadeler && k cargo run -q; cd ..
cd tip-gerekli && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
y = 26 (çift), sonuc = 20
i32 f64 alloc::vec::Vec<u8>
$ cargo build -q
error[E0282]: type annotations needed for `Vec<_>`
 --> src/main.rs:2:9
  |
2 |     let liste = Vec::new();
  |         ^^^^^   ---------- type must be known at this point
  |
help: consider giving `liste` an explicit type, where the type for type parameter `T` is specified
  |
2 |     let liste: Vec<T> = Vec::new();
  |              ++++++++

For more information about this error, try `rustc --explain E0282`.
error: could not compile `tip_gerekli` (bin "tip_gerekli") due to 1 previous error
[çıkış kodu: 101]
```

## 019-kosullar-ve-donguler

- Video: `019-kosullar-ve-donguler`
- Örnek yolu: `deneyler/mufredat-ornekleri/019-kosullar-ve-donguler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:38Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
n =   6:   8 adım, en yüksek değer 16
n =   7:  16 adım, en yüksek değer 52
n =  27: 111 adım, en yüksek değer 9232
n =  97: 118 adım, en yüksek değer 9232
1..1_000_000 arasında en uzun: n = 837799, 524 adım
```

## 020-fonksiyonlar-ve-cagri-yigini

- Video: `020-fonksiyonlar-ve-cagri-yigini`
- Örnek yolu: `deneyler/mufredat-ornekleri/020-fonksiyonlar-ve-cagri-yigini/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:38Z
- Değişken çıktı: adresler (ASLR); çerçeve boyutu derleme ayarına bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
seviye 0: yerel @ 0x7ffecd10f4d4
seviye 1: yerel @ 0x7ffecd10f3c4  (272 bayt aşağıda)
seviye 2: yerel @ 0x7ffecd10f2b4  (272 bayt aşağıda)
seviye 3: yerel @ 0x7ffecd10f1a4  (272 bayt aşağıda)
seviye 4: yerel @ 0x7ffecd10f094  (272 bayt aşağıda)
$ cargo run -q --release
seviye 0: yerel @ 0x7fff8d79717c
seviye 1: yerel @ 0x7fff8d79711c  (96 bayt aşağıda)
seviye 2: yerel @ 0x7fff8d7970bc  (96 bayt aşağıda)
seviye 3: yerel @ 0x7fff8d79705c  (96 bayt aşağıda)
seviye 4: yerel @ 0x7fff8d796ffc  (96 bayt aşağıda)
```

## 021-ozyineleme

- Video: `021-ozyineleme`
- Örnek yolu: `deneyler/mufredat-ornekleri/021-ozyineleme/`
- Kaynaklar: `Cargo.toml`, `kuyruk-cagrisi/Cargo.toml`, `kuyruk-cagrisi/src/main.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:29:30Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/ozyineleme
# Ana iş parçacığının yığını 8 MiB (ulimit -s); 1 KiB'lik çerçevelerle 100 bin seviye sığmaz
k ulimit -s
k ./target/debug/ozyineleme derin 1000
k ./target/debug/ozyineleme derin 100000
# Daha büyük yığınla yeni bir iş parçacığı
k ./target/debug/ozyineleme buyuk-yigin 100000
# Kuyruk çağrısı garantisi: açık `become` bu sürümde kararlı mı?
cd kuyruk-cagrisi && ks 'cargo build -q 2>&1 | head -n 8'; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/ozyineleme
fib(10) =     55  saf:       177 çağrı   hafızalı:  19 çağrı  (aynı mı: true)
fib(20) =   6765  saf:     21891 çağrı   hafızalı:  39 çağrı  (aynı mı: true)
fib(30) = 832040  saf:   2692537 çağrı   hafızalı:  59 çağrı  (aynı mı: true)
$ ulimit -s
8192
$ ./target/debug/ozyineleme derin 1000
derin(1000) = 124948
$ ./target/debug/ozyineleme derin 100000

thread 'main' (182) has overflowed its stack
fatal runtime error: stack overflow, aborting
[çıkış kodu: 134 = 128 + 6 (SIGABRT)]
$ ./target/debug/ozyineleme buyuk-yigin 100000
512 MiB yığınlı iş parçacığında derin(100000) = 12742480
$ cargo build -q 2>&1 | head -n 8
error[E0658]: `become` expression is experimental
 --> src/main.rs:6:5
  |
6 |     become say(n - 1, birikim + 1)
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #112788 <https://github.com/rust-lang/rust/issues/112788> for more information
```

## 022-bellek-hatalarinin-anatomisi

- Video: `022-bellek-hatalarinin-anatomisi`
- Örnek yolu: `deneyler/mufredat-ornekleri/022-bellek-hatalarinin-anatomisi/`
- Kaynaklar: `c/cift_birak.c`, `c/kullan_sonra_birak.c`, `c/tasma.c`, `rust/Cargo.toml`, `rust/src/main.rs`
- Toolchain: stable 1.100.0, gcc 14.2.0
- Son çalıştırma (UTC): 2026-11-20T16:03:38Z
- Değişken çıktı: AddressSanitizer adresleri ve süreç kimlikleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p target
# 1) Tampon taşması: yığın koruyucusu (canary) yakalar ve süreci durdurur
k gcc -O0 -fstack-protector-strong -o target/tasma c/tasma.c
k ./target/tasma Ferris
k ./target/tasma "bu metin sekiz bayttan cok daha uzun"
# 2) Çift iade: glibc'nin ayırıcısı fark eder
k gcc -O0 -o target/cift_birak c/cift_birak.c
k ./target/cift_birak
# 3) Kullanım-sonrası-iade: AddressSanitizer ile derleyince yakalanır
k gcc -O0 -g -fsanitize=address -o target/uaf_asan c/kullan_sonra_birak.c
ks './target/uaf_asan 2>&1 | head -n 30'
# 4) Rust: dizinin dışına erişim tanımlı bir panikle durur
cd rust && k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ gcc -O0 -fstack-protector-strong -o target/tasma c/tasma.c
$ ./target/tasma Ferris
tampon = Ferris
$ ./target/tasma "bu metin sekiz bayttan cok daha uzun"
*** stack smashing detected ***: terminated
[çıkış kodu: 134 = 128 + 6 (SIGABRT)]
$ gcc -O0 -o target/cift_birak c/cift_birak.c
$ ./target/cift_birak
free(): double free detected in tcache 2
[çıkış kodu: 134 = 128 + 6 (SIGABRT)]
$ gcc -O0 -g -fsanitize=address -o target/uaf_asan c/kullan_sonra_birak.c
$ ./target/uaf_asan 2>&1 | head -n 30
=================================================================
==1891==ERROR: AddressSanitizer: heap-use-after-free on address 0x502000000010 at pc 0x7f321abb379c bp 0x7ffc9ff9b000 sp 0x7ffc9ff9a7c0
READ of size 2 at 0x502000000010 thread T0
    #0 0x7f321abb379b in printf_common ../../../../src/libsanitizer/sanitizer_common/sanitizer_common_interceptors_format.inc:563
    #1 0x7f321abdf49a in vprintf ../../../../src/libsanitizer/sanitizer_common/sanitizer_common_interceptors.inc:1644
    #2 0x7f321abe108e in printf ../../../../src/libsanitizer/sanitizer_common/sanitizer_common_interceptors.inc:1702
    #3 0x55b287cec244 in main c/kullan_sonra_birak.c:16
    #4 0x7f321a94cca7 in __libc_start_call_main ../sysdeps/nptl/libc_start_call_main.h:58
    #5 0x7f321a94cd64 in __libc_start_main_impl ../csu/libc-start.c:360
    #6 0x55b287cec0e0 in _start (/work/mufredat-ornekleri/022-bellek-hatalarinin-anatomisi/target/uaf_asan+0x10e0) (BuildId: 95fedd94862cd2cc868a045a076292f8485e9a31)

0x502000000010 is located 0 bytes inside of 16-byte region [0x502000000010,0x502000000020)
freed by thread T0 here:
    #0 0x7f321ac0a8f8 in free ../../../../src/libsanitizer/asan/asan_malloc_linux.cpp:52
    #1 0x55b287cec1c0 in temizle c/kullan_sonra_birak.c:6
    #2 0x55b287cec200 in main c/kullan_sonra_birak.c:11
    #3 0x7f321a94cca7 in __libc_start_call_main ../sysdeps/nptl/libc_start_call_main.h:58

previously allocated by thread T0 here:
    #0 0x7f321ac0bc57 in malloc ../../../../src/libsanitizer/asan/asan_malloc_linux.cpp:69
    #1 0x55b287cec1d5 in main c/kullan_sonra_birak.c:9
    #2 0x7f321a94cca7 in __libc_start_call_main ../sysdeps/nptl/libc_start_call_main.h:58

SUMMARY: AddressSanitizer: heap-use-after-free ../../../../src/libsanitizer/sanitizer_common/sanitizer_common_interceptors_format.inc:563 in printf_common
Shadow bytes around the buggy address:
  0x501ffffffd80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501ffffffe00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501ffffffe80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501fffffff00: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x501fffffff80: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
$ cargo run -q

thread 'main' (1893) panicked at src/main.rs:4:34:
index out of bounds: the len is 8 but the index is 20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
```

## 023-bellek-yonetimi-stratejileri

- Video: `023-bellek-yonetimi-stratejileri`
- Örnek yolu: `deneyler/mufredat-ornekleri/023-bellek-yonetimi-stratejileri/`
- Kaynaklar: `python/gc_ornek.py`, `rust/Cargo.toml`, `rust/src/main.rs`
- Toolchain: stable 1.100.0, python3 (konteyner)
- Son çalıştırma (UTC): 2026-11-20T16:03:39Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Rust: kapsam bitince belirli (deterministik) bırakma (RAII)
cd rust && k cargo run -q; cd ..
# Python: referans sayma + döngüler için izleyen (tracing) çöp toplayıcı
k python3 --version
k python3 python/gc_ornek.py
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
iç kapsam bitiyor
  C bırakıldı
  B bırakıldı
  D bırakıldı
main bitiyor
  A bırakıldı
$ python3 --version
Python 3.13.5
$ python3 python/gc_ornek.py
a'nın referans sayısı: 1
  a silindi
--- döngü kuruluyor
del sonrası: hiçbir şey silinmedi
  x silindi
  y silindi
gc.collect() bulduğu erişilemez nesne: 2
```

## 024-sahiplik-kurallari

- Video: `024-sahiplik-kurallari`
- Örnek yolu: `deneyler/mufredat-ornekleri/024-sahiplik-kurallari/`
- Kaynaklar: `tasima/Cargo.toml`, `tasima/src/main.rs`, `tasima-sonrasi/Cargo.toml`, `tasima-sonrasi/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:39Z
- Değişken çıktı: adresler (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd tasima && k cargo run -q; cd ..
cd tasima-sonrasi && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
s1: yığındaki yapı 0x7ffce72081c8, öbekteki veri 0x5622bbebcd60
s2: yığındaki yapı 0x7ffce7208230, öbekteki veri 0x5622bbebcd60
$ cargo build -q
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:16
  |
2 |     let s1 = String::from("tapu");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{s1} {s2}");
  |                ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `tasima_sonrasi` (bin "tasima_sonrasi") due to 1 previous error
[çıkış kodu: 101]
```

## 025-kopyalama-ve-klonlama

- Video: `025-kopyalama-ve-klonlama`
- Örnek yolu: `deneyler/mufredat-ornekleri/025-kopyalama-ve-klonlama/`
- Kaynaklar: `copy-olamaz/Cargo.toml`, `copy-olamaz/src/main.rs`, `klon/Cargo.toml`, `klon/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:39Z
- Değişken çıktı: adresler (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd klon && k cargo run -q; cd ..
cd copy-olamaz && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
size_of::<String>() = 24 bayt (işaretçi + kapasite + uzunluk)
a = 5, b = 5
p = (1, 2), q = (1, 2)
s verisi @ 0x555642267d60, t verisi @ 0x555642267d80
$ cargo build -q
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> src/main.rs:2:8
  |
1 | #[derive(Clone, Copy)]
  |                 ---- in this derive macro expansion
2 | struct Kisi {
  |        ^^^^
3 |     ad: String,
  |     ---------- this field does not implement `Copy`

For more information about this error, try `rustc --explain E0204`.
error: could not compile `copy_olamaz` (bin "copy_olamaz") due to 1 previous error
[çıkış kodu: 101]
```

## 026-odunc-alma

- Video: `026-odunc-alma`
- Örnek yolu: `deneyler/mufredat-ornekleri/026-odunc-alma/`
- Kaynaklar: `kural/Cargo.toml`, `kural/src/main.rs`, `neden/Cargo.toml`, `neden/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:39Z
- Değişken çıktı: adresler (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd neden && k cargo run -q; cd ..
cd kural && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
kapasite 3, veri @ 0x56273aab7d60
kapasite 6, veri @ 0x56273aab7d60
kapasite 12, veri @ 0x56273aab7da0
$ cargo build -q
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let ilk = &v[0]; // paylaşılan ödünç
  |                - immutable borrow occurs here
4 |     v.push(4); // değiştirmek için özel (mut) ödünç gerekir
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("ilk = {ilk}");
  |                      --- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `kural` (bin "kural") due to 1 previous error
[çıkış kodu: 101]
```

## 027-dilimler-ve-str

- Video: `027-dilimler-ve-str`
- Örnek yolu: `deneyler/mufredat-ornekleri/027-dilimler-ve-str/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:39Z
- Değişken çıktı: panik satırındaki iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
&u8    :  8 bayt
&[u8]  : 16 bayt (işaretçi + uzunluk)
&str   : 16 bayt (işaretçi + uzunluk)
String : 24 bayt (işaretçi + kapasite + uzunluk)
"ağaç" baytları: [61, c4, 9f, 61, c3, a7]
&s[0..1] = "a"
&s[1..3] = "ğ"

thread 'main' (2309) panicked at src/main.rs:14:37:
end byte index 2 is not a char boundary; it is inside 'ğ' (bytes 1..3 of string)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
```

## 028-yasam-sureleri

- Video: `028-yasam-sureleri`
- Örnek yolu: `deneyler/mufredat-ornekleri/028-yasam-sureleri/`
- Kaynaklar: `duzeltilmis/Cargo.toml`, `duzeltilmis/src/main.rs`, `eksik-omur/Cargo.toml`, `eksik-omur/src/main.rs`, `sarkan/Cargo.toml`, `sarkan/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:40Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd sarkan && k cargo build -q; cd ..
cd eksik-omur && k cargo build -q; cd ..
cd duzeltilmis && k cargo run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error[E0597]: `x` does not live long enough
 --> src/main.rs:5:13
  |
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("r = {r}");
  |                    - borrow later used here

For more information about this error, try `rustc --explain E0597`.
error: could not compile `sarkan` (bin "sarkan") due to 1 previous error
[çıkış kodu: 101]
$ cargo build -q
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:30
  |
1 | fn uzun(a: &str, b: &str) -> &str {
  |            ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
help: consider introducing a named lifetime parameter
  |
1 | fn uzun<'a>(a: &'a str, b: &'a str) -> &'a str {
  |        ++++     ++          ++          ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `eksik_omur` (bin "eksik_omur") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q
daha uzun: ödünç
```

## 029-odunc-denetleyicinin-zihni

- Video: `029-odunc-denetleyicinin-zihni`
- Örnek yolu: `deneyler/mufredat-ornekleri/029-odunc-denetleyicinin-zihni/`
- Kaynaklar: `nll/Cargo.toml`, `nll/src/main.rs`, `temkinli/Cargo.toml`, `temkinli/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:03:40Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd nll && k cargo run -q; cd ..
cd temkinli && k cargo build -q
# Yeni nesil denetleyici Polonius (yalnızca nightly'de, deneysel bayrakla)
ks 'rustc +nightly -Z help | grep -i polonius'
ks 'RUSTFLAGS="-Zpolonius=next" cargo +nightly run -q --target-dir target/polonius'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
ilk = 1
v = [1, 2, 3, 4]
$ cargo build -q
error[E0502]: cannot borrow `*harita` as mutable because it is also borrowed as immutable
 --> src/main.rs:8:5
  |
4 | fn al_veya_ekle(harita: &mut HashMap<u32, String>, anahtar: u32) -> &String {
  |                         - let's call the lifetime of this reference `'1`
5 |     if let Some(deger) = harita.get(&anahtar) {
  |                          ------ immutable borrow occurs here
6 |         return deger;
  |                ----- returning this value requires that `*harita` is borrowed for `'1`
7 |     }
8 |     harita.insert(anahtar, String::from("yeni"));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `temkinli` (bin "temkinli") due to 1 previous error
[çıkış kodu: 101]
$ rustc +nightly -Z help | grep -i polonius
    -Z                                      polonius=val -- enable polonius-based borrow-checker (default: next)
$ RUSTFLAGS="-Zpolonius=next" cargo +nightly run -q --target-dir target/polonius
yeni
```

## 030-structlar-ve-bellek-yerlesimi

- Video: `030-structlar-ve-bellek-yerlesimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/030-structlar-ve-bellek-yerlesimi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:40Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
repr(Rust): boyut  8  a@6 b@0 c@4
repr(C)   : boyut 12  a@0 b@4 c@8
```

## 031-metotlar-ve-impl

- Video: `031-metotlar-ve-impl`
- Örnek yolu: `deneyler/mufredat-ornekleri/031-metotlar-ve-impl/`
- Kaynaklar: `hesap/Cargo.toml`, `hesap/src/main.rs`, `kapali-hesap/Cargo.toml`, `kapali-hesap/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:40Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd hesap && k cargo run -q; cd ..
cd kapali-hesap && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
bakiye: 100
Ayşe hesabı kapandı
son bakiye: 100
$ cargo build -q
error[E0382]: borrow of moved value: `h`
  --> src/main.rs:28:5
   |
23 |     let mut h = Hesap::yeni("Ayşe");
   |         ----- move occurs because `h` has type `Hesap`, which does not implement the `Copy` trait
...
26 |     let son = h.kapat();
   |                 ------- `h` moved due to this method call
27 |     println!("son bakiye: {son}");
28 |     h.yatir(5); // kapanmış hesaba para yatırmak
   |     ^ value borrowed here after move
   |
note: `Hesap::kapat` takes ownership of the receiver `self`, which moves `h`
  --> src/main.rs:16:14
   |
16 |     fn kapat(self) -> i64 { // self: hesabı tüketir
   |              ^^^^

For more information about this error, try `rustc --explain E0382`.
error: could not compile `kapali_hesap` (bin "kapali_hesap") due to 1 previous error
[çıkış kodu: 101]
```

## 032-enumlar-ve-toplam-tipler

- Video: `032-enumlar-ve-toplam-tipler`
- Örnek yolu: `deneyler/mufredat-ornekleri/032-enumlar-ve-toplam-tipler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:40Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
alan = 3.1416
alan = 6.0000
alan = 0.0000
size_of::<Sekil>() = 24 bayt
size_of::<f64>() * 2 = 16 bayt (en büyük varyantın yükü)
```

## 033-option-ve-null

- Video: `033-option-ve-null`
- Örnek yolu: `deneyler/mufredat-ornekleri/033-option-ve-null/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:40Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
&u64               : 8 bayt
Option<&u64>       : 8 bayt
Box<u64>           : 8 bayt
Option<Box<u64>>   : 8 bayt
u32                : 4 bayt
Option<u32>        : 8 bayt
Option<NonZeroU32> : 4 bayt
Option<bool>       : 1 bayt
Ferris: 7 yaşında
Ali: bulunamadı
```

## 034-oruntu-esleme

- Video: `034-oruntu-esleme`
- Örnek yolu: `deneyler/mufredat-ornekleri/034-oruntu-esleme/`
- Kaynaklar: `dilim-oruntuleri/Cargo.toml`, `dilim-oruntuleri/src/main.rs`, `eksik-kol/Cargo.toml`, `eksik-kol/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:41Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd eksik-kol && k cargo build -q; cd ..
cd dilim-oruntuleri && k cargo run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error[E0004]: non-exhaustive patterns: `Yon::Bati` not covered
  --> src/main.rs:9:11
   |
 9 |     match y {
   |           ^ pattern `Yon::Bati` not covered
   |
note: `Yon` defined here
  --> src/main.rs:1:6
   |
 1 | enum Yon {
   |      ^^^
...
 5 |     Bati,
   |     ---- not covered
   = note: the matched value is of type `Yon`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
12 ~         Yon::Dogu => (1, 0),
13 ~         Yon::Bati => todo!(),
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `eksik_kol` (bin "eksik_kol") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q
[] → boş
[7] → tek eleman: 7
[3, 1, 3] → baş ve son aynı: 3
[1, 2, 3, 4, 5] → 1 … 5 (arada 3 eleman)
```

## 035-tiplerin-matematigi

- Video: `035-tiplerin-matematigi`
- Örnek yolu: `deneyler/mufredat-ornekleri/035-tiplerin-matematigi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:41Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
()                      0 bayt  1 değer
bool                    1 bayt  2 değer
(bool, bool)            2 bayt  2 × 2 = 4 değer
Option<bool>            1 bayt  2 + 1 = 3 değer
Result<bool, ()>        1 bayt  2 + 1 = 3 değer
Hic                     0 bayt  0 değer
Option<Hic>             0 bayt  0 + 1 = 1 değer
Result<u32, Infallible> 4 bayt  (u32 ile aynı: hata kolu boş)
```

## 036-panik-ve-kurtarilamaz-hatalar

- Video: `036-panik-ve-kurtarilamaz-hatalar`
- Örnek yolu: `deneyler/mufredat-ornekleri/036-panik-ve-kurtarilamaz-hatalar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:41Z
- Değişken çıktı: iş parçacığı kimliği; geri izdeki adres/çerçeve sayısı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q -- 2
k cargo run -q -- 0
ks 'RUST_BACKTRACE=1 cargo run -q -- 0 2>&1 | grep -E -A1 "panicked at|panik::"'
# panic = "abort" profili: çözülme yok, süreç SIGABRT ile biter
k cargo run -q --release -- 0
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q -- 2
10 / 2 = 5
$ cargo run -q -- 0

thread 'main' (3227) panicked at src/main.rs:3:9:
sıfıra bölme: 10 / 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
$ RUST_BACKTRACE=1 cargo run -q -- 0 2>&1 | grep -E -A1 "panicked at|panik::"
thread 'main' (3239) panicked at src/main.rs:3:9:
sıfıra bölme: 10 / 0
--
   2: panik::bol
             at ./src/main.rs:3:9
   3: panik::main
             at ./src/main.rs:10:31
$ cargo run -q --release -- 0

thread 'main' (3251) panicked at src/main.rs:3:9:
sıfıra bölme: 10 / 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 134 = 128 + 6 (SIGABRT)]
```

## 037-result-ve-soru-isareti

- Video: `037-result-ve-soru-isareti`
- Örnek yolu: `deneyler/mufredat-ornekleri/037-result-ve-soru-isareti/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:41Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
Ok(42)
Err(ParseIntError { kind: InvalidDigit })
Err(ParseIntError { kind: PosOverflow })
toplam = 6
Error: ParseIntError { kind: InvalidDigit }
[çıkış kodu: 1]
```

## 038-hata-tipleri-tasarlamak

- Video: `038-hata-tipleri-tasarlamak`
- Örnek yolu: `deneyler/mufredat-ornekleri/038-hata-tipleri-tasarlamak/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `anyhow 1.0.104`, `proc-macro2 1.0.107`, `quote 1.0.47`, `syn 3.0.10`, `thiserror 2.0.21`, `thiserror-impl 2.0.21`, `unicode-ident 1.0.27`
- Son çalıştırma (UTC): 2026-11-20T16:03:43Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/ayar ayarlar/iyi.conf
k ./target/debug/ayar ayarlar/bozuk.conf
k ./target/debug/ayar ayarlar/yok.conf
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/ayar ayarlar/iyi.conf
port → 8080
ad → ambar
$ ./target/debug/ayar ayarlar/bozuk.conf
Error: sunucu başlatılamadı (ayarlar/bozuk.conf)

Caused by:
    satır 2: "hatali satir" anahtar=değer biçiminde değil
[çıkış kodu: 1]
$ ./target/debug/ayar ayarlar/yok.conf
Error: sunucu başlatılamadı (ayarlar/yok.conf)

Caused by:
    0: ayar dosyası okunamadı: ayarlar/yok.conf
    1: No such file or directory (os error 2)
[çıkış kodu: 1]
```

## 039-karmasiklik-analizi

- Video: `039-karmasiklik-analizi`
- Örnek yolu: `deneyler/mufredat-ornekleri/039-karmasiklik-analizi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:43Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
n =        1000: doğrusal        1000 karşılaştırma | ikili 11 (log2 n = 10.0)
n =     1000000: doğrusal     1000000 karşılaştırma | ikili 21 (log2 n = 19.9)
n =   100000000: doğrusal   100000000 karşılaştırma | ikili 28 (log2 n = 26.6)
```

## 040-vec-dinamik-dizi

- Video: `040-vec-dinamik-dizi`
- Örnek yolu: `deneyler/mufredat-ornekleri/040-vec-dinamik-dizi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:43Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
başlangıç: len 0, cap 0
len    1 → cap    4
len    5 → cap    8
len    9 → cap   16
len   17 → cap   32
len   33 → cap   64
len   65 → cap  128
len  129 → cap  256
len  257 → cap  512
len  513 → cap 1024
1000 push, yeniden ayırmada en fazla 1020 eleman taşındı (≈ push başına 1.02)
Vec::with_capacity(1000): cap 1000
```

## 041-hash-tablolari

- Video: `041-hash-tablolari`
- Örnek yolu: `deneyler/mufredat-ornekleri/041-hash-tablolari/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:03:44Z
- Değişken çıktı: HashMap sırası ve hash değeri her süreçte değişir (rastgele tohum); BTreeMap sırası sabittir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/hash
k ./target/debug/hash
# Kaynak: std'nin HashMap'i hashbrown (SwissTable) üzerine kurulu
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "use hashbrown\|Rust port of Google" $S/std/src/collections/hash/map.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/hash
HashMap sırası : ["Bursa", "Kars", "Van", "Rize", "Ankara", "İzmir", "Muş"]
BTreeMap sırası: ["Ankara", "Bursa", "Kars", "Muş", "Rize", "Van", "İzmir"]
hash("Ankara") = 26153e3c2064ee62
$ ./target/debug/hash
HashMap sırası : ["Van", "İzmir", "Ankara", "Rize", "Bursa", "Muş", "Kars"]
BTreeMap sırası: ["Ankara", "Bursa", "Kars", "Muş", "Rize", "Van", "İzmir"]
hash("Ankara") = 287ed9d435cb078d
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "use hashbrown\|Rust port of Google" $S/std/src/collections/hash/map.rs
4:use hashbrown::hash_map::{self as base, RustcOccupiedError};
59:/// The hash table implementation is a Rust port of Google's [SwissTable].
```

## 042-agaclar-ve-btreemap

- Video: `042-agaclar-ve-btreemap`
- Örnek yolu: `deneyler/mufredat-ornekleri/042-agaclar-ve-btreemap/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:03:44Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# Kaynak: B-ağacının dallanma sabiti
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^const B: usize\|^pub const CAPACITY" $S/alloc/src/collections/btree/node.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
sıralı      : {"Ada": 64, "Ali": 72, "Can": 85, "Ece": 91, "Naz": 78}
'B'..'D'    : [("Can", 85)]
ilk / son   : Some(("Ada", 64)) / Some(("Naz", 78))
1e6 arama   : HashMap   66.2 ms, BTreeMap  129.3 ms
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^const B: usize\|^pub const CAPACITY" $S/alloc/src/collections/btree/node.rs
43:const B: usize = 6;
```

## 043-siralama-ve-arama

- Video: `043-siralama-ve-arama`
- Örnek yolu: `deneyler/mufredat-ornekleri/043-siralama-ve-arama/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:03:45Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# Kaynak: std'nin sıralama algoritmaları
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "is based on \[driftsort\]" $S/alloc/src/slice.rs | head -1; grep -n "is based on \[ipnsort\]" $S/core/src/slice/mod.rs | head -1'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
sort_by_key (kararlı): [("Ayşe", 1), ("Ece", 1), ("Deniz", 2), ("Ali", 3), ("Can", 3)]
sort (kararlı) :  20832462 karşılaştırma
sort_unstable  :  20565895 karşılaştırma
n log2 n       :  19931569
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "is based on \[driftsort\]" $S/alloc/src/slice.rs | head -1; grep -n "is based on \[ipnsort\]" $S/core/src/slice/mod.rs | head -1
95:    /// The current implementation is based on [driftsort] by Orson Peters and Lukas Bergdoll, which
3118:    /// The current implementation is based on [ipnsort] by Lukas Bergdoll and Orson Peters, which
```

## 044-kuyruk-yigin-ve-graflar

- Video: `044-kuyruk-yigin-ve-graflar`
- Örnek yolu: `deneyler/mufredat-ornekleri/044-kuyruk-yigin-ve-graflar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:45Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
BFS: S → E en kısa yol 12 adım
Dijkstra: İstanbul → Ankara 450 km, yol: İstanbul → Bolu → Ankara
```

## 045-onbellek-dostu-veri-yapilari

- Video: `045-onbellek-dostu-veri-yapilari`
- Örnek yolu: `deneyler/mufredat-ornekleri/045-onbellek-dostu-veri-yapilari/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:45Z
- Değişken çıktı: süreler donanıma ve yüke bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
matris satır-öncelikli gezinme         2.75 ms
matris sütun-öncelikli gezinme        74.55 ms
Vec<u64> toplamı                       1.84 ms
LinkedList<u64> toplamı                6.93 ms
AoS: Vec<Parcacik> içinden kütle       4.03 ms
SoA: Vec<f32> kütle dizisi             2.72 ms
```

## 046-traitler-davranis-sozlesmesi

- Video: `046-traitler-davranis-sozlesmesi`
- Örnek yolu: `deneyler/mufredat-ornekleri/046-traitler-davranis-sozlesmesi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:46Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
daire (alan 12.57)
3 kenarlı kare
Daire { r: 2.0 } == Daire { r: 2.0 } → true
```

## 047-jenerikler

- Video: `047-jenerikler`
- Örnek yolu: `deneyler/mufredat-ornekleri/047-jenerikler/`
- Kaynaklar: `sinirli/Cargo.toml`, `sinirli/src/main.rs`, `sinirsiz/Cargo.toml`, `sinirsiz/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:46Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd sinirli && k cargo run -q; cd ..
cd sinirsiz && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
9
1.5
ç
$ cargo build -q
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:4:14
  |
4 |         if x > eb {
  |            - ^ -- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn en_buyuk<T: std::cmp::PartialOrd>(liste: &[T]) -> &T {
  |              ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `sinirsiz` (bin "sinirsiz") due to 1 previous error
[çıkış kodu: 101]
```

## 048-monomorfizasyon

- Video: `048-monomorfizasyon`
- Örnek yolu: `deneyler/mufredat-ornekleri/048-monomorfizasyon/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:46Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/monomorf
# Tek bir jenerik fonksiyon, ikilide üç ayrı makine kodu kopyası
ks 'nm -C target/release/monomorf | grep ikiye_katla'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/monomorf | grep -A3 "<monomorf::ikiye_katla::<"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/monomorf
42 2.5 14
$ nm -C target/release/monomorf | grep ikiye_katla
0000000000015e40 t monomorf::ikiye_katla::<f64>
0000000000015e48 t monomorf::ikiye_katla::<u8>
0000000000015e4c t monomorf::ikiye_katla::<i32>
$ objdump -d -M intel --no-show-raw-insn -C target/release/monomorf | grep -A3 "<monomorf::ikiye_katla::<"
0000000000015e40 <monomorf::ikiye_katla::<f64>>:
   15e40:	addsd  xmm0,xmm0
   15e44:	ret
   15e45:	int3
--
0000000000015e48 <monomorf::ikiye_katla::<u8>>:
   15e48:	lea    eax,[rdi+rdi*1]
   15e4b:	ret

0000000000015e4c <monomorf::ikiye_katla::<i32>>:
   15e4c:	lea    eax,[rdi+rdi*1]
   15e4f:	ret

--
   15e95:	call   15e4c <monomorf::ikiye_katla::<i32>>
   15e9a:	mov    DWORD PTR [rsp+0xc],eax
   15e9e:	movabs rax,0x3ff4000000000000
   15ea8:	mov    QWORD PTR [rsp+0x10],rax
--
   15eb8:	call   15e40 <monomorf::ikiye_katla::<f64>>
   15ebd:	movsd  QWORD PTR [rsp+0x40],xmm0
   15ec3:	mov    BYTE PTR [rsp+0x10],0x7
   15ec8:	lea    rax,[rsp+0x10]
--
   15ed2:	call   15e48 <monomorf::ikiye_katla::<u8>>
   15ed7:	mov    BYTE PTR [rsp+0xb],al
   15edb:	lea    rax,[rsp+0xc]
   15ee0:	mov    QWORD PTR [rsp+0x10],rax
```

## 049-trait-nesneleri-ve-vtable

- Video: `049-trait-nesneleri-ve-vtable`
- Örnek yolu: `deneyler/mufredat-ornekleri/049-trait-nesneleri-ve-vtable/`
- Kaynaklar: `dyn-uyumsuz/Cargo.toml`, `dyn-uyumsuz/src/main.rs`, `vtable/Cargo.toml`, `vtable/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:46Z
- Değişken çıktı: adresler (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd vtable && k cargo run -q; cd ..
cd dyn-uyumsuz && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
&Daire          :  8 bayt
&dyn Sekil      : 16 bayt (veri işaretçisi + vtable işaretçisi)
Box<dyn Sekil>  : 16 bayt
alan = 3.142
alan = 4.000
veri işaretçisi : 0x558d2d6374c0
vtable          : 0x558d2d688a58
  [0] drop      : 0x0
  [1] boyut     : 8
  [2] hizalama  : 8
  [3] alan()    : 0x558d2d648150
<Daire as Sekil>::alan adresi: 0x558d2d648150
$ cargo build -q
error[E0038]: the trait `Kaydedici` is not dyn compatible
  --> src/main.rs:13:17
   |
13 |     let k: &dyn Kaydedici = &Ekran;
   |                 ^^^^^^^^^ `Kaydedici` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> src/main.rs:2:8
   |
 1 | trait Kaydedici {
   |       --------- this trait is not dyn compatible...
 2 |     fn kaydet<T: std::fmt::Debug>(&self, deger: T);
   |        ^^^^^^ ...because method `kaydet` has generic type parameters
   = help: consider moving `kaydet` to another trait
   = help: only type `Ekran` implements `Kaydedici`; consider using it directly instead.

For more information about this error, try `rustc --explain E0038`.
error: could not compile `dyn_uyumsuz` (bin "dyn_uyumsuz") due to 1 previous error
[çıkış kodu: 101]
```

## 050-statik-ve-dinamik-dagitim

- Video: `050-statik-ve-dinamik-dagitim`
- Örnek yolu: `deneyler/mufredat-ornekleri/050-statik-ve-dinamik-dagitim/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:47Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/dagitim
# Üç fonksiyon yazdık, ikilide iki sembol var: &dyn ve Box<dyn> sürümleri aynı koda derlendi ve birleştirildi
ks 'nm -C target/release/dagitim | grep toplam'
# Dinamik sürümde dolaylı çağrı: adres bellekten (vtable'ın 4. girdisi, +0x18) okunup çağrılıyor
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dagitim | awk "/<dagitim::toplam_kutulu>:/,/ret/" | grep -E "call|ret"'
# Statik sürümde hiç çağrı yok: alan() satır içine açılmış
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dagitim | awk "/<dagitim::toplam_statik::<dagitim::Kare>>:/,/ret/" | grep -E "call|ret"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/dagitim
statik   7.04 ms | &dyn  47.61 ms | Box<dyn>  48.22 ms | eşit: true
statik   6.96 ms | &dyn  47.58 ms | Box<dyn>  47.82 ms | eşit: true
$ nm -C target/release/dagitim | grep toplam
0000000000016be0 t dagitim::toplam_statik::<dagitim::Kare>
0000000000016e30 t dagitim::toplam_kutulu
$ objdump -d -M intel --no-show-raw-insn -C target/release/dagitim | awk "/<dagitim::toplam_kutulu>:/,/ret/" | grep -E "call|ret"
   16e6f:	call   QWORD PTR [rax+0x18]
   16e9a:	ret
$ objdump -d -M intel --no-show-raw-insn -C target/release/dagitim | awk "/<dagitim::toplam_statik::<dagitim::Kare>>:/,/ret/" | grep -E "call|ret"
   16c04:	ret
```

## 051-standart-traitler-ve-donusumler

- Video: `051-standart-traitler-ve-donusumler`
- Örnek yolu: `deneyler/mufredat-ornekleri/051-standart-traitler-ve-donusumler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:47Z
- Değişken çıktı: panik satırındaki iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
eşit mi: true
kümede 2 eleman var
100 °C = 212 °F
total_cmp ile: [-1.0, 0.0, 2.5, NaN]

thread 'main' (4683) panicked at src/main.rs:41:46:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
```

## 052-closurelar

- Video: `052-closurelar`
- Örnek yolu: `deneyler/mufredat-ornekleri/052-closurelar/`
- Kaynaklar: `fnonce/Cargo.toml`, `fnonce/src/main.rs`, `yakalama/Cargo.toml`, `yakalama/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:48Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd yakalama && k cargo run -q; cd ..
cd fnonce && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
hiçbir şey yakalamayan :  0 bayt
&u64 yakalayan         :  8 bayt
&mut u32 yakalayan     :  8 bayt
String taşıyan (move)  : 24 bayt
1 11 1 2
Ferris
$ cargo build -q
error[E0382]: use of moved value: `tuket`
 --> src/main.rs:5:20
  |
4 |     println!("{}", tuket());
  |                    ------- `tuket` moved due to this call
5 |     println!("{}", tuket());
  |                    ^^^^^ value used here after move
  |
note: closure cannot be invoked more than once because it moves the variable `ad` out of its environment
 --> src/main.rs:3:25
  |
3 |     let tuket = move || ad; // çağrılınca ad'ı dışarı verir: FnOnce
  |                         ^^
note: this value implements `FnOnce`, which causes it to be moved when called
 --> src/main.rs:4:20
  |
4 |     println!("{}", tuket());
  |                    ^^^^^

For more information about this error, try `rustc --explain E0382`.
error: could not compile `fnonce` (bin "fnonce") due to 1 previous error
[çıkış kodu: 101]
```

## 053-iterator-protokolu

- Video: `053-iterator-protokolu`
- Örnek yolu: `deneyler/mufredat-ornekleri/053-iterator-protokolu/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:48Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
warning: unused `Map` that must be used
  --> src/main.rs:18:5
   |
18 | /     v.iter().map(|x| {
19 | |         println!("işleniyor {x}");
20 | |         x * 2
21 | |     }); // tembel: kimse sonuç istemedi, hiçbir şey çalışmaz
   | |______^
   |
   = note: iterators are lazy and do nothing unless consumed
   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
   |
18 |     let _ = v.iter().map(|x| {
   |     +++++++

--- collect ile:
işleniyor 1
işleniyor 2
işleniyor 3
[2, 4, 6]
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
[1, 9, 25, 49]
```

## 054-sifir-maliyetli-soyutlama

- Video: `054-sifir-maliyetli-soyutlama`
- Örnek yolu: `deneyler/mufredat-ornekleri/054-sifir-maliyetli-soyutlama/`
- Kaynaklar: `iki_yol.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:48Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p target
k rustc --edition 2024 -C opt-level=3 --crate-type=lib --emit=obj -o target/iki_yol.o iki_yol.rs
# Her fonksiyonun komut sayısı
ks 'objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -cP "^\s+[0-9a-f]+:"'
ks 'objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -cP "^\s+[0-9a-f]+:"'
# Vektör (SIMD) komutları: dört u32'yi aynı anda işleyen p... komutları
ks 'objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -oE "\bp[a-z]+\b" | sort | uniq -c'
# Komut komut karşılaştırma (adresler ve etiketler ayıklanır)
ks 'diff <(objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//") <(objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//") && echo "makine kodu birebir aynı"'
# Sıra göz ardı edilirse: aynı komut kümesi mi?
ks 'diff <(objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) <(objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) && echo "aynı 64 komut, yalnızca sıralama farklı"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ rustc --edition 2024 -C opt-level=3 --crate-type=lib --emit=obj -o target/iki_yol.o iki_yol.rs
$ objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -cP "^\s+[0-9a-f]+:"
64
$ objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -cP "^\s+[0-9a-f]+:"
64
$ objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -oE "\bp[a-z]+\b" | sort | uniq -c
      5 paddd
      4 pand
      2 pcmpeqd
      4 pmuludq
      8 pshufd
      2 punpckldq
      3 pxor
$ diff <(objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//") <(objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//") && echo "makine kodu birebir aynı"
6d5
< xor    eax,eax
7a7
> xor    eax,eax
[çıkış kodu: 1]
$ diff <(objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) <(objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) && echo "aynı 64 komut, yalnızca sıralama farklı"
aynı 64 komut, yalnızca sıralama farklı
```

## 055-fonksiyonel-desenler

- Video: `055-fonksiyonel-desenler`
- Örnek yolu: `deneyler/mufredat-ornekleri/055-fonksiyonel-desenler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:48Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
{"açalım": 1, "beraber": 1, "berber": 2, "berbere": 1, "bir": 3, "demiş": 1, "dükkanı": 1, "gel": 1}
hepsi ya da hiç : Err(ParseIntError { kind: InvalidDigit })
geçerli olanlar : [10, 20, 40]
çift [2, 4, 6, 8, 10], tek [1, 3, 5, 7, 9]
```

## 056-moduller-ve-gorunurluk

- Video: `056-moduller-ve-gorunurluk`
- Örnek yolu: `deneyler/mufredat-ornekleri/056-moduller-ve-gorunurluk/`
- Kaynaklar: `gizli-alan/Cargo.toml`, `gizli-alan/src/main.rs`, `kapsulleme/Cargo.toml`, `kapsulleme/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:48Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd kapsulleme && k cargo run -q; cd ..
cd gizli-alan && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
Ayşe çekmek istiyor: Err("yetersiz bakiye: 50 < 80")
bakiye: 50
$ cargo build -q
error[E0616]: field `bakiye` of struct `Hesap` is private
  --> src/main.rs:32:7
   |
32 |     h.bakiye = 1_000_000; // kuralı dışarıdan delmeye çalış
   |       ^^^^^^ private field

For more information about this error, try `rustc --explain E0616`.
error: could not compile `gizli_alan` (bin "gizli_alan") due to 1 previous error
[çıkış kodu: 101]
```

## 057-crateler-ve-bagimliliklar

- Video: `057-crateler-ve-bagimliliklar`
- Örnek yolu: `deneyler/mufredat-ornekleri/057-crateler-ve-bagimliliklar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `itoa 1.0.19`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.10`, `unicode-ident 1.0.27`, `zmij 1.0.23`
- Son çalıştırma (UTC): 2026-11-20T16:03:50Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cat Cargo.toml
k cargo run -q
# "1" gereksinimi hangi sürüme çözüldü? Bağımlılık ağacı:
k cargo tree
# Cargo.lock: tam sürüm + içerik özeti (checksum)
ks 'grep -A3 "name = \"serde_json\"" Cargo.lock'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cat Cargo.toml
[package]
name = "json_ornek"
version = "0.1.0"
edition = "2024"

[dependencies]
serde_json = "1"
$ cargo run -q
{"ad":"Ferris","bacak":10,"tur":"yengeç"}
$ cargo tree
json_ornek v0.1.0 (/work/mufredat-ornekleri/057-crateler-ve-bagimliliklar)
└── serde_json v1.0.154
    ├── itoa v1.0.19
    ├── memchr v2.8.3
    ├── serde_core v1.0.229
    └── zmij v1.0.23
$ grep -A3 "name = \"serde_json\"" Cargo.lock
name = "serde_json"
version = "1.0.154"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e7e9cc8b1b85264074fbcc02a88680c4096b1e47df8f739dceb03bf482f04bd6"
```

## 058-tedarik-zinciri-guvenligi

- Video: `058-tedarik-zinciri-guvenligi`
- Örnek yolu: `deneyler/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/`
- Kaynaklar: `asgari/Cargo.toml`, `asgari/src/main.rs`, `tam/Cargo.toml`, `tam/src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:03:52Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Aynı crate, farklı özellik (feature) seçimi: kaç paket koda girer?
cd tam
k grep tokio Cargo.toml
k cargo fetch -q
ks 'cargo tree --prefix none -e normal | sort -u | wc -l'
k cargo tree --depth 1 -e normal
cd ../asgari
k grep tokio Cargo.toml
k cargo fetch -q
k cargo tree -e normal
cd ..
# Denetim araçları crates.io'da yayımlanıyor (bu imajda kurulu değil)
ks 'for a in cargo-audit cargo-deny cargo-vet; do cargo search --limit 1 $a | head -n 1; done'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ grep tokio Cargo.toml
tokio = { version = "1", features = ["full"] }
$ cargo fetch -q
$ cargo tree --prefix none -e normal | sort -u | wc -l
22
$ cargo tree --depth 1 -e normal
tam v0.1.0 (/work/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/tam)
└── tokio v1.54.1
$ grep tokio Cargo.toml
tokio = { version = "1", default-features = false, features = ["rt"] }
$ cargo fetch -q
$ cargo tree -e normal
asgari v0.1.0 (/work/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/asgari)
└── tokio v1.54.1
    └── pin-project-lite v0.2.18
$ for a in cargo-audit cargo-deny cargo-vet; do cargo search --limit 1 $a | head -n 1; done
cargo-audit = "0.23.0"    # Audit Cargo.lock for crates with security vulnerabilities
note: to learn more about a package, run `cargo info <name>`
cargo-deny = "0.22.0"    # Cargo plugin to help you manage large dependency graphs
note: to learn more about a package, run `cargo info <name>`
cargo-vet = "0.10.3"    # Supply-chain security for Rust
note: to learn more about a package, run `cargo info <name>`
```

## 059-birim-ve-entegrasyon-testleri

- Video: `059-birim-ve-entegrasyon-testleri`
- Örnek yolu: `deneyler/mufredat-ornekleri/059-birim-ve-entegrasyon-testleri/`
- Kaynaklar: `duzeltilmis/Cargo.toml`, `duzeltilmis/src/lib.rs`, `duzeltilmis/tests/disaridan.rs`, `hatali/Cargo.toml`, `hatali/src/lib.rs`, `hatali/tests/disaridan.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:53Z
- Değişken çıktı: test süreleri, iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Birim + entegrasyon + belge testleri (ilk hatada durmadan)
cd hatali && k cargo test -q --no-fail-fast; cd ..
# Düzeltme: fark
ks 'diff -u hatali/src/lib.rs duzeltilmis/src/lib.rs'
cd duzeltilmis && k cargo test -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q --no-fail-fast

running 3 tests
.. 2/3
testler::cift_uzunlukta_medyan --- FAILED

failures:

---- testler::cift_uzunlukta_medyan stdout ----

thread 'testler::cift_uzunlukta_medyan' (5761) panicked at src/lib.rs:40:9:
assertion `left == right` failed
  left: Some(3.0)
 right: Some(2.5)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    testler::cift_uzunlukta_medyan

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.15s; merged doctests compilation took 0.14s
error: 1 target failed:
    `--lib`
[çıkış kodu: 101]
$ diff -u hatali/src/lib.rs duzeltilmis/src/lib.rs
--- hatali/src/lib.rs	2026-11-20 15:28:52.858579606 +0000
+++ duzeltilmis/src/lib.rs	2026-11-20 15:29:51.180755268 +0000
@@ -18,7 +18,12 @@
         return None;
     }
     v.sort_by(|a, b| a.total_cmp(b));
-    Some(v[v.len() / 2]) // HATA: çift uzunlukta ortadaki iki değerin ortalaması olmalı
+    let m = v.len() / 2;
+    if v.len() % 2 == 0 {
+        Some((v[m - 1] + v[m]) / 2.0) // çift uzunluk: ortadaki iki değerin ortalaması
+    } else {
+        Some(v[m])
+    }
 }
 
 #[cfg(test)]
[çıkış kodu: 1]
$ cargo test -q

running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.12s; merged doctests compilation took 0.11s
```

## 060-ozellik-tabanli-test-ve-fuzzing

- Video: `060-ozellik-tabanli-test-ve-fuzzing`
- Örnek yolu: `deneyler/mufredat-ornekleri/060-ozellik-tabanli-test-ve-fuzzing/`
- Kaynaklar: `Cargo.toml`, `src/lib.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `autocfg 1.5.1`, `bit-set 0.11.1`, `bit-vec 0.10.2`, `bitflags 2.13.2`, `borsh 1.8.1`, `borsh-derive 1.8.1`, `bytes 1.12.2`, `cfg-if 1.0.5`, `cfg_aliases 0.2.2`, `chacha20 0.10.2`, `core_detect 1.0.0`, `cpufeatures 0.3.1`, `equivalent 1.0.3`, `errno 0.3.15`, `fastrand 2.5.0`, `fnv 1.0.7`, `getrandom 0.4.3`, `hashbrown 0.17.1`, `indexmap 2.14.2`, `libc 0.2.191`, `linux-raw-sys 0.12.1`, `memchr 2.8.3`, `num-traits 0.2.19`, `once_cell 1.21.4`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `proptest 1.12.0`, `quick-error 1.2.3`, `quote 1.0.47`, `r-efi 6.0.0`, `rand 0.10.4`, `rand_core 0.10.1`, `rand_xorshift 0.5.0`, `regex-syntax 0.8.12`, `rustix 1.1.6`, `rusty-fork 0.3.1`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `syn 3.0.10`, `tempfile 3.27.0`, `toml_datetime 1.1.3+spec-1.1.0`, `toml_edit 0.25.19+spec-1.1.0`, `toml_parser 1.1.5+spec-1.1.0`, `unarray 0.1.4`, `unicode-ident 1.0.27`, `wait-timeout 0.2.1`, `windows-link 0.2.1`, `windows-sys 0.61.2`, `winnow 1.0.5`
- Son çalıştırma (UTC): 2026-11-20T16:03:56Z
- Değişken çıktı: proptest rastgele girdilerle başlar; küçültülmüş (shrink) örnek ve süreler her çalıştırmada değişebilir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
rm -rf proptest-regressions
ks 'cargo test -q 2>&1 | grep -E "^---- |minimal failing input|successes|^test result"'
rm -rf proptest-regressions
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q 2>&1 | grep -E "^---- |minimal failing input|successes|^test result"
---- testler::saf_surum_aralikta stdout ----
minimal failing input: a = 2903406468, b = 1391560828
	successes: 0
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## 061-clippy-rustfmt-ve-ci

- Video: `061-clippy-rustfmt-ve-ci`
- Örnek yolu: `deneyler/mufredat-ornekleri/061-clippy-rustfmt-ve-ci/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:56Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# Derleyici sustu; clippy ne diyor?
k cargo clippy -q
# Sürekli entegrasyonda kapı: uyarı = hata
ks 'cargo clippy -q -- -D warnings 2>&1 | tail -n 2'
# Biçim denetimi: farkı göster, dosyaya dokunma
k cargo fmt --check
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
81.66666666666667
$ cargo clippy -q
warning: unneeded `return` statement
 --> src/main.rs:9:5
  |
9 |     return toplam as f64 / v.len() as f64;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.100.0/index.html#needless_return
  = note: `#[warn(clippy::needless_return)]` on by default
help: remove `return`
  |
9 -     return toplam as f64 / v.len() as f64;
9 +     toplam as f64 / v.len() as f64
  |

warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
 --> src/main.rs:1:16
  |
1 | fn ortalama(v: &Vec<i32>) -> f64 {
  |                ^^^^^^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.100.0/index.html#ptr_arg
  = note: `#[warn(clippy::ptr_arg)]` on by default
help: change this to
  |
1 - fn ortalama(v: &Vec<i32>) -> f64 {
1 + fn ortalama(v: &[i32]) -> f64 {
  |

warning: the loop variable `i` is only used to index `v`
 --> src/main.rs:3:14
  |
3 |     for i in 0..v.len() {
  |              ^^^^^^^^^^
  |
note: for this index operation
 --> src/main.rs:4:27
  |
4 |         toplam = toplam + v[i];
  |                           ^^^^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.100.0/index.html#needless_range_loop
  = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator
  |
3 -     for i in 0..v.len() {
3 +     for <item> in &v {
  |

warning: manual implementation of an assign operation
 --> src/main.rs:4:9
  |
4 |         toplam = toplam + v[i];
  |         ^^^^^^^^^^^^^^^^^^^^^^ help: replace it with: `toplam += v[i]`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.100.0/index.html#assign_op_pattern
  = note: `#[warn(clippy::assign_op_pattern)]` on by default

warning: length comparison to zero
 --> src/main.rs:6:8
  |
6 |     if v.len() == 0 {
  |        ^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `v.is_empty()`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.100.0/index.html#len_zero
  = note: `#[warn(clippy::len_zero)]` on by default

$ cargo clippy -q -- -D warnings 2>&1 | tail -n 2

error: could not compile `kalite` (bin "kalite") due to 5 previous errors
$ cargo fmt --check
Diff in /work/mufredat-ornekleri/061-clippy-rustfmt-ve-ci/src/main.rs:10:
 }
 
 fn main() {
-    let   notlar = vec![70, 85,90];
+    let notlar = vec![70, 85, 90];
     println!("{}", ortalama(&notlar));
 }
 
[çıkış kodu: 1]
```

## 062-box-ve-ozyinelemeli-tipler

- Video: `062-box-ve-ozyinelemeli-tipler`
- Örnek yolu: `deneyler/mufredat-ornekleri/062-box-ve-ozyinelemeli-tipler/`
- Kaynaklar: `agac/Cargo.toml`, `agac/src/main.rs`, `sonsuz/Cargo.toml`, `sonsuz/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:56Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd sonsuz && k cargo build -q; cd ..
cd agac && k cargo run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error[E0072]: recursive type `Liste` has infinite size
 --> src/main.rs:1:1
  |
1 | enum Liste {
  | ^^^^^^^^^^
2 |     Bos,
3 |     Eleman(i32, Liste),
  |                 ----- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
3 |     Eleman(i32, Box<Liste>),
  |                 ++++     +

For more information about this error, try `rustc --explain E0072`.
error: could not compile `sonsuz` (bin "sonsuz") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q
sıralı gezinme: [20, 30, 35, 40, 50, 60, 70, 80], derinlik 4
size_of::<Box<Agac>>() = 8 bayt, size_of::<Agac>() = 24 bayt
```

## 063-rc-ve-arc-referans-sayma

- Video: `063-rc-ve-arc-referans-sayma`
- Örnek yolu: `deneyler/mufredat-ornekleri/063-rc-ve-arc-referans-sayma/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:56Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
güçlü sayaç: 1
güçlü sayaç: 3 (a, b, c aynı adrese bakıyor: true)
güçlü sayaç: 2 (c kapsam dışında)
zayıf sayaç: 1, upgrade: Some(18)
upgrade (sahipler gittikten sonra): None
Arc ile paylaşılan vektörden: [10, 20, 30], sayaç şimdi 1
```

## 064-ic-degistirilebilirlik

- Video: `064-ic-degistirilebilirlik`
- Örnek yolu: `deneyler/mufredat-ornekleri/064-ic-degistirilebilirlik/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:57Z
- Değişken çıktı: panik satırındaki iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
Cell sayaç: 2
RefCell: [1, 2, 3]
--- Rc + RefCell ile döngü:
x sayaç 2, y sayaç 2
kapsam bitti, hiçbiri bırakılmadı: bellek sızdı (güvenli ama sızıntı)
--- Weak geri bağlantı ile:
ebeveyn güçlü 1, zayıf 1
  ebeveyn bırakıldı
kapsam bitti
--- aynı anda iki borrow_mut:

thread 'main' (6846) panicked at src/main.rs:62:25:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
```

## 065-bellek-ayiricilar

- Video: `065-bellek-ayiricilar`
- Örnek yolu: `deneyler/mufredat-ornekleri/065-bellek-ayiricilar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:57Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
String::new + 1000 × push_str              ayırma     1  büyütme   8
String::with_capacity(2000) + 1000 × push  ayırma     1  büyütme   0
1000 × format!("{i}") → Vec<String>        ayırma  1001  büyütme   0
1000 × Box::new(i)                         ayırma  1001  büyütme   0
```

## 066-eszamanlilik-ve-is-parcaciklari

- Video: `066-eszamanlilik-ve-is-parcaciklari`
- Örnek yolu: `deneyler/mufredat-ornekleri/066-eszamanlilik-ve-is-parcaciklari/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:03:58Z
- Değişken çıktı: görülen sıralar ve süreler her çalıştırmada değişir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k nproc
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ nproc
12
$ cargo run -q --release
1000 denemede 7 farklı sıra görüldü; en sık: [("0123", 976), ("0132", 14), ("0213", 3), ("1023", 3)]
 1 iş parçacığı: 216816 asal,   439.6 ms, hızlanma 1.00×
 2 iş parçacığı: 216816 asal,   272.7 ms, hızlanma 1.61×
 4 iş parçacığı: 216816 asal,   149.3 ms, hızlanma 2.95×
 6 iş parçacığı: 216816 asal,   108.0 ms, hızlanma 4.07×
12 iş parçacığı: 216816 asal,    84.7 ms, hızlanma 5.19×
```

## 067-veri-yarisi-ve-send-sync

- Video: `067-veri-yarisi-ve-send-sync`
- Örnek yolu: `deneyler/mufredat-ornekleri/067-veri-yarisi-ve-send-sync/`
- Kaynaklar: `atomik/Cargo.toml`, `atomik/src/main.rs`, `c/yaris.c`, `rc-gonderilemez/Cargo.toml`, `rc-gonderilemez/src/main.rs`
- Toolchain: stable 1.100.0, gcc 14.2.0
- Son çalıştırma (UTC): 2026-11-20T16:03:59Z
- Değişken çıktı: C yarışının sonucu her çalıştırmada farklıdır
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p target
k gcc -O2 -pthread -o target/yaris c/yaris.c
k ./target/yaris
k ./target/yaris
# ThreadSanitizer yarışı adıyla bildirir
k gcc -O1 -g -fsanitize=thread -pthread -o target/yaris_tsan c/yaris.c
ks './target/yaris_tsan 2>&1 | grep -E "WARNING|Write of size|Previous write|SUMMARY" | head -n 6'
# Rust: aynı paylaşım denemesi derlenmez
cd rc-gonderilemez && k cargo build -q; cd ..
cd atomik && k cargo run -q --release; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ gcc -O2 -pthread -o target/yaris c/yaris.c
$ ./target/yaris
beklenen 20000000, bulunan 10006703
$ ./target/yaris
beklenen 20000000, bulunan 10726010
$ gcc -O1 -g -fsanitize=thread -pthread -o target/yaris_tsan c/yaris.c
$ ./target/yaris_tsan 2>&1 | grep -E "WARNING|Write of size|Previous write|SUMMARY" | head -n 6
WARNING: ThreadSanitizer: data race (pid=11088)
  Previous write of size 8 at 0x562caf17f058 by thread T1:
SUMMARY: ThreadSanitizer: data race c/yaris.c:8 in artir
WARNING: ThreadSanitizer: data race (pid=11088)
  Write of size 8 at 0x562caf17f058 by thread T2:
  Previous write of size 8 at 0x562caf17f058 by thread T1:
$ cargo build -q
error[E0277]: `Rc<RefCell<i32>>` cannot be sent between threads safely
  --> src/main.rs:8:27
   |
 8 |       let t = thread::spawn(move || {
   |               ------------- ^------
   |               |             |
   |  _____________|_____________within this `{closure@src/main.rs:8:27: 8:34}`
   | |             |
   | |             required by a bound introduced by this call
 9 | |         *kopya.borrow_mut() += 1;
10 | |     });
   | |_____^ `Rc<RefCell<i32>>` cannot be sent between threads safely
   |
   = help: within `{closure@src/main.rs:8:27: 8:34}`, the trait `Send` is not implemented for `Rc<RefCell<i32>>`
note: required because `kopya` is used within this closure
  --> src/main.rs:9:10
   |
 9 |         *kopya.borrow_mut() += 1;
   |          ^^^^^
note: required by a bound in `spawn`
  --> /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/thread/functions.rs:125:0

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rc_gonderilemez` (bin "rc_gonderilemez") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q --release
beklenen 20000000, bulunan 20000000
```

## 068-mutex-ve-kilitler

- Video: `068-mutex-ve-kilitler`
- Örnek yolu: `deneyler/mufredat-ornekleri/068-mutex-ve-kilitler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:00Z
- Değişken çıktı: süreler; panik satırındaki iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
Mutex           : 8000000 ( 216.9 ms)
AtomicU64       : 8000000 (  90.0 ms)
yerel + toplama : 8000000 (   0.6 ms)

thread '<unnamed>' (11251) panicked at src/main.rs:50:9:
kilit elimdeyken çöktüm
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
kilit zehirlenmiş! içerik yine de okunabilir: [1, 2, 3, 4]
```

## 069-kilitlenme-ve-esgudum

- Video: `069-kilitlenme-ve-esgudum`
- Örnek yolu: `deneyler/mufredat-ornekleri/069-kilitlenme-ve-esgudum/`
- Kaynaklar: `kilitlenme/Cargo.toml`, `kilitlenme/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:03Z
- Değişken çıktı: iki iş parçacığının yazdırma sırası
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd kilitlenme
k cargo build -q
# Ters sıralı kilitler: sonsuza dek bekler; 3 saniye sonra timeout öldürür (124)
k timeout 3 ./target/debug/kilitlenme
# Tek bir kilit sırası: kilitlenme yok
k timeout 3 ./target/debug/kilitlenme sirali
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ timeout 3 ./target/debug/kilitlenme
1. iş parçacığı A'yı aldı, B'yi bekliyor
2. iş parçacığı ilk kilidi aldı, ikincisini bekliyor
[çıkış kodu: 124]
$ timeout 3 ./target/debug/kilitlenme sirali
1. iş parçacığı A'yı aldı, B'yi bekliyor
1. iş parçacığı transferi yaptı (100 / 100)
2. iş parçacığı ilk kilidi aldı, ikincisini bekliyor
2. iş parçacığı transferi yaptı (100 / 100)
bitti
```

## 070-kanallar-ve-mesajlasma

- Video: `070-kanallar-ve-mesajlasma`
- Örnek yolu: `deneyler/mufredat-ornekleri/070-kanallar-ve-mesajlasma/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:04Z
- Değişken çıktı: milisaniye zaman damgaları ve aynı anlara düşen satırların sırası
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
1² = 1
2² = 4
3² = 9
4² = 16
5² = 25
[   0 ms] üretici 1 gönderdi
[   0 ms] üretici 2 gönderdi
[ 300 ms] yavaş tüketici 1 aldı
[ 300 ms] üretici 3 gönderdi
[ 400 ms] yavaş tüketici 2 aldı
[ 400 ms] üretici 4 gönderdi
[ 500 ms] yavaş tüketici 3 aldı
[ 500 ms] üretici 5 gönderdi
[ 600 ms] yavaş tüketici 4 aldı
[ 700 ms] yavaş tüketici 5 aldı
```

## 071-atomikler-ve-bellek-siralamasi

- Video: `071-atomikler-ve-bellek-siralamasi`
- Örnek yolu: `deneyler/mufredat-ornekleri/071-atomikler-ve-bellek-siralamasi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:05Z
- Değişken çıktı: Relaxed sayımı her çalıştırmada değişir (SeqCst her zaman 0 olmalı)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/siralama_testi
# Makine kodundaki fark: SeqCst yazma xchg (tam bariyer) olarak derleniyor
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_oku_gevsek>:"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A5 "<siralama_testi::yaz_oku_sirali>:"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/siralama_testi
Relaxed : 1000000 turda r1 = r2 = 0 sonucu 495535 kez görüldü
SeqCst  : 1000000 turda r1 = r2 = 0 sonucu 0 kez görüldü
$ objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_oku_gevsek>:"
00000000000182e0 <siralama_testi::yaz_oku_gevsek>:
   182e0:	mov    DWORD PTR [rdi],0x1
   182e6:	mov    eax,DWORD PTR [rsi]
   182e8:	ret
   182e9:	int3
$ objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A5 "<siralama_testi::yaz_oku_sirali>:"
00000000000182f0 <siralama_testi::yaz_oku_sirali>:
   182f0:	mov    eax,0x1
   182f5:	xchg   DWORD PTR [rdi],eax
   182f7:	mov    eax,DWORD PTR [rsi]
   182f9:	ret
   182fa:	int3
```

## 072-veri-paralelligi

- Video: `072-veri-paralelligi`
- Örnek yolu: `deneyler/mufredat-ornekleri/072-veri-paralelligi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `crossbeam-deque 0.8.8`, `crossbeam-epoch 0.9.21`, `crossbeam-utils 0.8.23`, `either 1.19.0`, `rayon 1.12.0`, `rayon-core 1.13.0`
- Son çalıştırma (UTC): 2026-11-20T16:04:08Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
rayon iş parçacığı: 12
sıralı  :   112.0 ms
paralel :    15.4 ms  (hızlanma 7.28×, sonuç aynı: true)
yanlış paylaşım (aynı satır):   982.0 ms
ayrı önbellek satırları     :    87.8 ms
```

## 073-surecler

- Video: `073-surecler`
- Örnek yolu: `deneyler/mufredat-ornekleri/073-surecler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:08Z
- Değişken çıktı: süreç kimlikleri (pid)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/surecler
# Çekirdek düzeyinde: yeni süreç (clone/clone3), yeni program (execve), bekleme (wait4)
ks 'strace -f -e trace=clone,clone3,vfork,execve,wait4,exit_group ./target/debug/surecler 2>&1 | grep -vE "^(ebeveyn|çocuk|uname|başlatıl)"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/surecler
ebeveyn: pid 11889
çocuk: pid 11890, ebeveyni 11889
çocuğun çıkış durumu: exit status: 3 (code = Some(3))
uname -sr çıktısı yakalandı: "Linux 7.2.15-1-omarchy"
başlatılamadı: No such file or directory (os error 2)
$ strace -f -e trace=clone,clone3,vfork,execve,wait4,exit_group ./target/debug/surecler 2>&1 | grep -vE "^(ebeveyn|çocuk|uname|başlatıl)"
execve("./target/debug/surecler", ["./target/debug/surecler"], 0x7ffc43765eb8 /* 15 vars */) = 0
clone3({flags=CLONE_VM|CLONE_VFORK|CLONE_CLEAR_SIGHAND, exit_signal=SIGCHLD, stack=0x7f8f6f122000, stack_size=0x9000}, 88strace: Process 11899 attached
 <unfinished ...>
[pid 11899] execve("/usr/local/cargo/bin/sh", ["sh", "-c", "echo \"\303\247ocuk: pid $$, ebeveyni $"...], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11899] execve("/usr/local/sbin/sh", ["sh", "-c", "echo \"\303\247ocuk: pid $$, ebeveyni $"...], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11899] execve("/usr/local/bin/sh", ["sh", "-c", "echo \"\303\247ocuk: pid $$, ebeveyni $"...], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11899] execve("/usr/sbin/sh", ["sh", "-c", "echo \"\303\247ocuk: pid $$, ebeveyni $"...], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11899] execve("/usr/bin/sh", ["sh", "-c", "echo \"\303\247ocuk: pid $$, ebeveyni $"...], 0x7fff27ae6418 /* 15 vars */ <unfinished ...>
[pid 11898] <... clone3 resumed>)       = 11899
[pid 11898] wait4(11899,  <unfinished ...>
[pid 11899] <... execve resumed>)       = 0
[pid 11899] exit_group(3)               = ?
[pid 11899] +++ exited with 3 +++
<... wait4 resumed>[{WIFEXITED(s) && WEXITSTATUS(s) == 3}], 0, NULL) = 11899
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=11899, si_uid=0, si_status=3, si_utime=0, si_stime=0} ---
çocuğun çıkış durumu: exit status: 3 (code = Some(3))
clone3({flags=CLONE_VM|CLONE_VFORK|CLONE_CLEAR_SIGHAND, exit_signal=SIGCHLD, stack=0x7f8f6f122000, stack_size=0x9000}, 88strace: Process 11900 attached
 <unfinished ...>
[pid 11900] execve("/usr/local/cargo/bin/uname", ["uname", "-sr"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11900] execve("/usr/local/sbin/uname", ["uname", "-sr"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11900] execve("/usr/local/bin/uname", ["uname", "-sr"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11900] execve("/usr/sbin/uname", ["uname", "-sr"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11900] execve("/usr/bin/uname", ["uname", "-sr"], 0x7fff27ae6418 /* 15 vars */ <unfinished ...>
[pid 11898] <... clone3 resumed>)       = 11900
[pid 11900] <... execve resumed>)       = 0
[pid 11900] exit_group(0)               = ?
[pid 11898] wait4(11900,  <unfinished ...>
[pid 11900] +++ exited with 0 +++
<... wait4 resumed>[{WIFEXITED(s) && WEXITSTATUS(s) == 0}], 0, NULL) = 11900
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=11900, si_uid=0, si_status=0, si_utime=0, si_stime=0} ---
clone3({flags=CLONE_VM|CLONE_VFORK|CLONE_CLEAR_SIGHAND, exit_signal=SIGCHLD, stack=0x7f8f6f122000, stack_size=0x9000}, 88strace: Process 11901 attached
 <unfinished ...>
[pid 11901] execve("/usr/local/cargo/bin/olmayan-program", ["olmayan-program"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11901] execve("/usr/local/sbin/olmayan-program", ["olmayan-program"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11901] execve("/usr/local/bin/olmayan-program", ["olmayan-program"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11901] execve("/usr/sbin/olmayan-program", ["olmayan-program"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11901] execve("/usr/bin/olmayan-program", ["olmayan-program"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11901] execve("/sbin/olmayan-program", ["olmayan-program"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11901] execve("/bin/olmayan-program", ["olmayan-program"], 0x7fff27ae6418 /* 15 vars */) = -1 ENOENT (No such file or directory)
[pid 11901] exit_group(127)             = ?
[pid 11898] <... clone3 resumed>)       = 11901
[pid 11901] +++ exited with 127 +++
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=11901, si_uid=0, si_status=127, si_utime=0, si_stime=0} ---
exit_group(0)                           = ?
+++ exited with 0 +++
```

## 074-dosyalar-ve-tamponlama

- Video: `074-dosyalar-ve-tamponlama`
- Örnek yolu: `deneyler/mufredat-ornekleri/074-dosyalar-ve-tamponlama/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:04:24Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
ks 'strace -c -e trace=write ./target/release/tamponlama tamponsuz 2>&1 | tail -n 4'
ks 'strace -c -e trace=write ./target/release/tamponlama tamponlu 2>&1 | tail -n 4'
k hyperfine -N --warmup 2 --runs 10 --style basic "./target/release/tamponlama tamponsuz" "./target/release/tamponlama tamponlu"
ks 'wc -l /tmp/tamponsuz.txt /tmp/tamponlu.txt && cmp /tmp/tamponsuz.txt /tmp/tamponlu.txt && echo "iki dosya aynı"'
# Kaynak (nightly rust-src): BufWriter/BufReader varsayılan tampon boyutu
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A3 "pub const DEFAULT_BUF_SIZE" $S/alloc/src/io/read.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ strace -c -e trace=write ./target/release/tamponlama tamponsuz 2>&1 | tail -n 4
------ ----------- ----------- --------- --------- ----------------
100.00    1.510840           5    300000           write
------ ----------- ----------- --------- --------- ----------------
100.00    1.510840           5    300000           total
$ strace -c -e trace=write ./target/release/tamponlama tamponlu 2>&1 | tail -n 4
------ ----------- ----------- --------- --------- ----------------
100.00    0.001929          12       158           write
------ ----------- ----------- --------- --------- ----------------
100.00    0.001929          12       158           total
$ hyperfine -N --warmup 2 --runs 10 --style basic "./target/release/tamponlama tamponsuz" "./target/release/tamponlama tamponlu"
Benchmark 1: ./target/release/tamponlama tamponsuz
  Time (mean ± σ):     814.7 ms ±  13.9 ms    [User: 81.4 ms, System: 726.5 ms]
  Range (min … max):   796.8 ms … 837.0 ms    10 runs
 
Benchmark 2: ./target/release/tamponlama tamponlu
  Time (mean ± σ):       5.8 ms ±   0.1 ms    [User: 1.5 ms, System: 2.8 ms]
  Range (min … max):     5.7 ms …   6.0 ms    10 runs
 
Summary
  ./target/release/tamponlama tamponlu ran
  139.88 ± 4.09 times faster than ./target/release/tamponlama tamponsuz
$ wc -l /tmp/tamponsuz.txt /tmp/tamponlu.txt && cmp /tmp/tamponsuz.txt /tmp/tamponlu.txt && echo "iki dosya aynı"
 100000 /tmp/tamponsuz.txt
 100000 /tmp/tamponlu.txt
 200000 total
iki dosya aynı
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A3 "pub const DEFAULT_BUF_SIZE" $S/alloc/src/io/read.rs
787:pub const DEFAULT_BUF_SIZE: usize = cfg_select! {
788-    target_os = "espidf" => 512,
789-    _ => 8 * 1024,
790-};
```

## 075-dayaniklilik-ve-fsync

- Video: `075-dayaniklilik-ve-fsync`
- Örnek yolu: `deneyler/mufredat-ornekleri/075-dayaniklilik-ve-fsync/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:24Z
- Değişken çıktı: süreler diske ve dosya sistemine bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p calisma
k cargo build -q --release
# Ana makinenin diskine bağlı dizin (btrfs)
k ./target/release/dayaniklilik olc calisma
# Konteynerin kendi kök dosya sistemi (/tmp burada)
k ./target/release/dayaniklilik olc /tmp
# Neden bu kadar hızlı? Kök dizinin bağlama seçeneklerine bakalım
ks 'grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"'
ks 'grep " /work " /proc/mounts | cut -d" " -f1-3'
# Atomik değiştirme kalıbının sistem çağrıları
ks 'strace -e trace=openat,write,fsync,fdatasync,rename,renameat2 ./target/release/dayaniklilik atomik calisma 2>&1 | grep -v "/lib\|/etc\|/proc\|/sys"'
k cat calisma/ayar.json
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/dayaniklilik olc calisma
her kayıtta fsync : 200 kayıt   163.71 ms (0.819 ms/kayıt)
sonda tek fsync   : 200 kayıt     2.62 ms (0.013 ms/kayıt)
$ ./target/release/dayaniklilik olc /tmp
her kayıtta fsync : 200 kayıt     1.69 ms (0.008 ms/kayıt)
sonda tek fsync   : 200 kayıt     1.60 ms (0.008 ms/kayıt)
$ grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"
overlay
fsync=volatile
$ grep " /work " /proc/mounts | cut -d" " -f1-3
/dev/mapper/root /work btrfs
$ strace -e trace=openat,write,fsync,fdatasync,rename,renameat2 ./target/release/dayaniklilik atomik calisma 2>&1 | grep -v "/lib\|/etc\|/proc\|/sys"
openat(AT_FDCWD, "calisma/ayar.json.gecici", O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0666) = 3
write(3, "{\"surum\": 2}\n", 13)        = 13
fsync(3)                                = 0
rename("calisma/ayar.json.gecici", "calisma/ayar.json") = 0
openat(AT_FDCWD, "calisma", O_RDONLY|O_CLOEXEC) = 4
fsync(4)                                = 0
+++ exited with 0 +++
$ cat calisma/ayar.json
{"surum": 2}
```

## 076-borular-ve-sinyaller

- Video: `076-borular-ve-sinyaller`
- Örnek yolu: `deneyler/mufredat-ornekleri/076-borular-ve-sinyaller/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:25Z
- Değişken çıktı: iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/borular
# Okuyan taraf boruyu erken kapatırsa: Rust SIGPIPE'ı yok sayar, yazma EPIPE hatası döner, println! paniğe dönüşür
ks './target/debug/borular cok-yaz | head -n 2'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/borular
sort çıktısı:
elma
kiraz
zeytin
SIGTERM sonrası: signal: 15 (SIGTERM)
kill() sonrası  : signal: 9 (SIGKILL)
$ ./target/debug/borular cok-yaz | head -n 2
satır 0
satır 1

thread 'main' (12192) panicked at /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/io/stdio.rs:1209:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## 077-ag-katmanlari-ve-soketler

- Video: `077-ag-katmanlari-ve-soketler`
- Örnek yolu: `deneyler/mufredat-ornekleri/077-ag-katmanlari-ve-soketler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:25Z
- Değişken çıktı: port numaraları ve dosya tanımlayıcı numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/soketler
# Aynı programın ağ sistem çağrıları
ks 'strace -f -e trace=socket,bind,listen,accept4,connect,write,sendto,recvfrom,read,shutdown,close ./target/debug/soketler 2>&1 | grep -E "socket\(|bind\(|listen\(|accept4\(|connect\(|sendto\(|recvfrom\(" '
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/soketler
sunucu dinliyor: 127.0.0.1:40917
istemci: 127.0.0.1:38338 → 127.0.0.1:40917
sunucu: 127.0.0.1:38338 bağlandı
istemci aldı: "MERHABA AĞ"
$ strace -f -e trace=socket,bind,listen,accept4,connect,write,sendto,recvfrom,read,shutdown,close ./target/debug/soketler 2>&1 | grep -E "socket\(|bind\(|listen\(|accept4\(|connect\(|sendto\(|recvfrom\(" 
socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_IP) = 3
bind(3, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 0
listen(3, 128)                          = 0
[pid 12311] socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_IP) = 4
[pid 12311] connect(4, {sa_family=AF_INET, sin_port=htons(38593), sin_addr=inet_addr("127.0.0.1")}, 16) = 0
[pid 12311] sendto(4, "merhaba a\304\237", 11, MSG_NOSIGNAL, NULL, 0) = 11
[pid 12311] recvfrom(4,  <unfinished ...>
[pid 12312] accept4(3, {sa_family=AF_INET, sin_port=htons(33160), sin_addr=inet_addr("127.0.0.1")}, [128 => 16], SOCK_CLOEXEC) = 5
[pid 12312] recvfrom(5, "merhaba a\304\237", 64, 0, NULL, NULL) = 11
[pid 12312] sendto(5, "MERHABA A\304\236", 11, MSG_NOSIGNAL, NULL, 0 <unfinished ...>
[pid 12311] recvfrom(4,  <unfinished ...>
```

## 078-tcp-akis-dogasi

- Video: `078-tcp-akis-dogasi`
- Örnek yolu: `deneyler/mufredat-ornekleri/078-tcp-akis-dogasi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:26Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
sunucu tek read ile 13 bayt aldı: "bir|iki|üç|"
TCP_NODELAY = false: 20 istek   781.6 ms (istek başına 39.08 ms)
TCP_NODELAY = true : 20 istek     0.4 ms (istek başına 0.02 ms)
```

## 079-udp-ve-dns

- Video: `079-udp-ve-dns`
- Örnek yolu: `deneyler/mufredat-ornekleri/079-udp-ve-dns/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:26Z
- Değişken çıktı: kaynak port numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
ks 'grep -v "^#" /etc/hosts | grep localhost'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
3 bayt datagram (127.0.0.1:46781 kaynaklı): "bir"
3 bayt datagram (127.0.0.1:46781 kaynaklı): "iki"
4 bayt datagram (127.0.0.1:46781 kaynaklı): "üç"
localhost:80 → [[::1]:80, 127.0.0.1:80]
$ grep -v "^#" /etc/hosts | grep localhost
127.0.0.1	localhost
::1	localhost
```

## 080-http-elle

- Video: `080-http-elle`
- Örnek yolu: `deneyler/mufredat-ornekleri/080-http-elle/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:27Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
./target/debug/http_elle 2 2> calisma_sunucu.log &
sleep 0.5
# 1) Elle yazılmış ham istek (bash'in /dev/tcp yönlendirmesiyle)
ks 'exec 3<>/dev/tcp/127.0.0.1/8099; printf "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n" >&3; cat <&3 | od -c | head -n 12'
# 2) curl ile: > giden, < gelen satırlar
ks 'curl -sv http://127.0.0.1:8099/ 2>&1 | grep -E "^[<>] |^Merhaba"'
wait
k cat calisma_sunucu.log
rm -f calisma_sunucu.log
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ exec 3<>/dev/tcp/127.0.0.1/8099; printf "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n" >&3; cat <&3 | od -c | head -n 12
0000000   H   T   T   P   /   1   .   1       2   0   0       O   K  \r
0000020  \n   C   o   n   t   e   n   t   -   T   y   p   e   :       t
0000040   e   x   t   /   p   l   a   i   n   ;       c   h   a   r   s
0000060   e   t   =   u   t   f   -   8  \r  \n   C   o   n   t   e   n
0000100   t   -   L   e   n   g   t   h   :       1   5  \r  \n   C   o
0000120   n   n   e   c   t   i   o   n   :       c   l   o   s   e  \r
0000140  \n  \r  \n   M   e   r   h   a   b   a   ,       H   T   T   P
0000160   !  \n
0000162
$ curl -sv http://127.0.0.1:8099/ 2>&1 | grep -E "^[<>] |^Merhaba"
> GET / HTTP/1.1
> Host: 127.0.0.1:8099
> User-Agent: curl/8.14.1
> Accept: */*
> 
< HTTP/1.1 200 OK
< Content-Type: text/plain; charset=utf-8
< Content-Length: 15
< Connection: close
< 
Merhaba, HTTP!
$ cat calisma_sunucu.log
[sunucu] istek: GET / HTTP/1.1
[sunucu] istek: GET / HTTP/1.1
```

## 081-protokol-tasarimi

- Video: `081-protokol-tasarimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/081-protokol-tasarimi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:28Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
tel üzerindeki baytlar: [00, 00, 00, 07, 6d, 65, 72, 68, 61, 62, 61]
saf sunucu tek read'de 48 bayt aldı (3 mesaj yapışık geldi)
çerçeveli sunucu mesaj aldı: "GET anahtar"
çerçeveli sunucu mesaj aldı: "SET anahtar 42"
çerçeveli sunucu mesaj aldı: "DEL anahtar"
```

## 082-serilestirme

- Video: `082-serilestirme`
- Örnek yolu: `deneyler/mufredat-ornekleri/082-serilestirme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `itoa 1.0.19`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.10`, `unicode-ident 1.0.27`, `zmij 1.0.23`
- Son çalıştırma (UTC): 2026-11-20T16:04:30Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
JSON  (59 bayt): {"sensor":7,"sicaklik":21.5,"zaman":1763640000,"birim":"C"}
ikili (16 bayt): [07, 00, 00, 00, ac, 41, c0, 02, 1f, 69, 00, 00, 00, 00, 01, 43]
JSON'dan geri okundu, aynı mı: true
eski kayıt okundu: Olcum { sensor: 3, sicaklik: 19.0, zaman: 1700000000, birim: "" }
"invalid type: string \"yedi\", expected u16 at line 1 column 16"
```

## 083-neden-async

- Video: `083-neden-async`
- Örnek yolu: `deneyler/mufredat-ornekleri/083-neden-async/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:04:39Z
- Değişken çıktı: süreler ve bellek ölçümleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cat /sys/fs/cgroup/pids.max
k cargo build -q --release
k ./target/release/neden_async is-parcacigi 1000
k ./target/release/neden_async gorev 1000
k ./target/release/neden_async gorev 100000
# Bu konteynerde süreç/iş parçacığı sınırı var: 5000 iş parçacığı açılamaz
ks './target/release/neden_async is-parcacigi 5000 2>&1 | grep -E "panicked|failed|Resource" | head -n 3'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cat /sys/fs/cgroup/pids.max
2048
$ cargo build -q --release
$ ./target/release/neden_async is-parcacigi 1000
1000 iş parçacığı : başlatma   19.5 ms, RSS 12672 kB, sanal 8294860 kB
    toplam süre 1.02 s
$ ./target/release/neden_async gorev 1000
1000 async görev   : başlatma    0.9 ms, RSS 3656 kB, sanal 816016 kB
    toplam süre 1.00 s
$ ./target/release/neden_async gorev 100000
100000 async görev   : başlatma   32.9 ms, RSS 41504 kB, sanal 853892 kB
    toplam süre 1.03 s
$ ./target/release/neden_async is-parcacigi 5000 2>&1 | grep -E "panicked|failed|Resource" | head -n 3
thread 'main' (14541) panicked at /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/thread/functions.rs:131:29:
failed to spawn thread: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }
```

## 084-epoll-ve-olay-dongusu

- Video: `084-epoll-ve-olay-dongusu`
- Örnek yolu: `deneyler/mufredat-ornekleri/084-epoll-ve-olay-dongusu/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:04:44Z
- Değişken çıktı: dosya tanımlayıcı numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/olay_dongusu
# Olay döngüsünün çekirdekle konuşması: epoll
ks 'strace -f -e trace=epoll_create1,epoll_ctl,epoll_wait,epoll_pwait,epoll_pwait2,accept4,connect ./target/debug/olay_dongusu 2>&1 | grep -v "^yankı"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/olay_dongusu
yankı: yanki
$ strace -f -e trace=epoll_create1,epoll_ctl,epoll_wait,epoll_pwait,epoll_pwait2,accept4,connect ./target/debug/olay_dongusu 2>&1 | grep -v "^yankı"
epoll_create1(EPOLL_CLOEXEC)            = 3
epoll_ctl(3, EPOLL_CTL_ADD, 4, {events=EPOLLIN|EPOLLRDHUP|EPOLLET, data=0}) = 0
epoll_ctl(5, EPOLL_CTL_ADD, 8, {events=EPOLLIN|EPOLLRDHUP|EPOLLET, data=0x1}) = 0
epoll_ctl(5, EPOLL_CTL_ADD, 9, {events=EPOLLIN|EPOLLOUT|EPOLLRDHUP|EPOLLET, data=0x560fd2859380}) = 0
connect(10, {sa_family=AF_INET, sin_port=htons(37981), sin_addr=inet_addr("127.0.0.1")}, 16) = -1 EINPROGRESS (Operation now in progress)
epoll_ctl(5, EPOLL_CTL_ADD, 10, {events=EPOLLIN|EPOLLOUT|EPOLLRDHUP|EPOLLET, data=0x560fd2859780}) = 0
epoll_wait(3, [{events=EPOLLIN, data=0x560fd2859380}, {events=EPOLLOUT, data=0x560fd2859780}], 1024, -1) = 2
accept4(9, {sa_family=AF_INET, sin_port=htons(60602), sin_addr=inet_addr("127.0.0.1")}, [128 => 16], SOCK_CLOEXEC|SOCK_NONBLOCK) = 11
epoll_ctl(5, EPOLL_CTL_ADD, 11, {events=EPOLLIN|EPOLLOUT|EPOLLRDHUP|EPOLLET, data=0x560fd2859900}) = 0
epoll_ctl(5, EPOLL_CTL_DEL, 11, NULL)   = 0
epoll_ctl(5, EPOLL_CTL_DEL, 9, NULL)    = 0
epoll_wait(3, [{events=EPOLLIN|EPOLLOUT|EPOLLRDHUP, data=0x560fd2859780}], 1024, -1) = 1
epoll_ctl(5, EPOLL_CTL_DEL, 10, NULL)   = 0
+++ exited with 0 +++
```

## 085-future-durum-makinesi

- Video: `085-future-durum-makinesi`
- Örnek yolu: `deneyler/mufredat-ornekleri/085-future-durum-makinesi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:44Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
size_of_val(bos())                  = 1 bayt
size_of_val(tampon_await_boyunca()) = 4098 bayt
size_of_val(tampon_await_oncesi())  = 16 bayt
async blok oluşturuldu; henüz hiçbir şey olmadı
async blok çalıştı
ilk poll → Ready(())
poll 1 → Pending
poll 2 → Pending
poll 3 → Pending
poll 4 → Ready("bitti")
```

## 086-kendi-yurutucumuz

- Video: `086-kendi-yurutucumuz`
- Örnek yolu: `deneyler/mufredat-ornekleri/086-kendi-yurutucumuz/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:45Z
- Değişken çıktı: milisaniye değerleri ±1-2 ms oynayabilir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
[görev] başladı          (  0 ms)
[görev] 1. bekleme bitti (200 ms)
[görev] 2. bekleme bitti (500 ms)
[yürütücü] 3 yoklamada tamamlandı
sonuç = 42
```

## 087-pin-ve-kendine-referans

- Video: `087-pin-ve-kendine-referans`
- Örnek yolu: `deneyler/mufredat-ornekleri/087-pin-ve-kendine-referans/`
- Kaynaklar: `sabitlenmis/Cargo.toml`, `sabitlenmis/src/main.rs`, `tasima-sorunu/Cargo.toml`, `tasima-sorunu/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:04:45Z
- Değişken çıktı: adresler (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd tasima-sorunu && k cargo run -q; cd ..
cd sabitlenmis && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
a.veri @ 0x7ffeed0fbb08   a.isaretci = 0x7ffeed0fbb08
b.veri @ 0x7ffeed0fbb68   b.isaretci = 0x7ffeed0fbb08  ← hâlâ eski yeri gösteriyor
$ cargo build -q
error[E0277]: `PhantomPinned` cannot be unpinned
  --> src/main.rs:10:23
   |
10 |     let _p = Pin::new(&mut s); // Pin::new yalnızca Unpin tipler için güvenli
   |              -------- ^^^^^^ within `Sabit`, the trait `Unpin` is not implemented for `PhantomPinned`
   |              |
   |              required by a bound introduced by this call
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
note: required because it appears within the type `Sabit`
  --> src/main.rs:5:5
   |
 4 | struct Sabit {
 5 |     _isaret: PhantomPinned, // "beni taşıma" işareti: tip Unpin olmaz
   |     ^^^^^^^^^^^^^^^^^^^^^^ required by this field
note: required by a bound in `Pin::<Ptr>::new`
  --> /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/pin.rs:1181:4

For more information about this error, try `rustc --explain E0277`.
error: could not compile `sabitlenmis` (bin "sabitlenmis") due to 1 previous error
[çıkış kodu: 101]
```

## 088-tokio-ile-async

- Video: `088-tokio-ile-async`
- Örnek yolu: `deneyler/mufredat-ornekleri/088-tokio-ile-async/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:04:53Z
- Değişken çıktı: süreler ve kovalara düşen istemci sayıları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/tokio_sunucu 100
# Dinleme kuyruğu taşma sayaçları (bu konteynerin ağ ad alanı) — önce
ks "awk '/^TcpExt:/ { if (!h) { split(\$0, ad); h=1 } else { for (i=2;i<=NF;i++) if (ad[i] ~ /^Listen(Overflows|Drops)\$/) print ad[i], \$i } }' /proc/net/netstat"
k ./target/release/tokio_sunucu 1000
# ... ve sonra
ks "awk '/^TcpExt:/ { if (!h) { split(\$0, ad); h=1 } else { for (i=2;i<=NF;i++) if (ad[i] ~ /^Listen(Overflows|Drops)\$/) print ad[i], \$i } }' /proc/net/netstat"
# tokio'nun bind() ile kurduğu dinleme kuyruğu uzunluğu
ks 'strace -f -e trace=listen ./target/release/tokio_sunucu 10 2>&1 | grep "listen("'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/tokio_sunucu 100
çalışma zamanı işçi iş parçacığı: 12
100 eşzamanlı istemci (her biri sunucuda 100 ms bekletildi): toplam 123 ms
  istemci süreleri: <0,5 s: 100  ~1 s: 0  ~2 s: 0  ≥2,5 s: 0
zaman aşımı: yavaş iş iptal edildi
$ awk '/^TcpExt:/ { if (!h) { split($0, ad); h=1 } else { for (i=2;i<=NF;i++) if (ad[i] ~ /^Listen(Overflows|Drops)$/) print ad[i], $i } }' /proc/net/netstat
ListenOverflows 0
ListenDrops 0
$ ./target/release/tokio_sunucu 1000
çalışma zamanı işçi iş parçacığı: 12
1000 eşzamanlı istemci (her biri sunucuda 100 ms bekletildi): toplam 1815 ms
  istemci süreleri: <0,5 s: 444  ~1 s: 380  ~2 s: 176  ≥2,5 s: 0
zaman aşımı: yavaş iş iptal edildi
$ awk '/^TcpExt:/ { if (!h) { split($0, ad); h=1 } else { for (i=2;i<=NF;i++) if (ad[i] ~ /^Listen(Overflows|Drops)$/) print ad[i], $i } }' /proc/net/netstat
ListenOverflows 1682
ListenDrops 1682
$ strace -f -e trace=listen ./target/release/tokio_sunucu 10 2>&1 | grep "listen("
[pid 17733] listen(9, 128)              = 0
```

## 089-async-tuzaklari

- Video: `089-async-tuzaklari`
- Örnek yolu: `deneyler/mufredat-ornekleri/089-async-tuzaklari/`
- Kaynaklar: `bloklama/Cargo.toml`, `bloklama/src/main.rs`, `kilit-await/Cargo.toml`, `kilit-await/src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:05:03Z
- Değişken çıktı: milisaniye değerleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd bloklama && k cargo run -q; cd ..
cd kilit-await && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
std::thread::sleep ile iki görev : 1000 ms
tokio::time::sleep ile iki görev :  501 ms
  [uzun iş] 1. adım yapıldı
  [uzun iş] future düşürüldü (iptal)
zaman aşımı kazandı
$ cargo build -q
error: future cannot be sent between threads safely
   --> src/main.rs:8:5
    |
  8 | /     tokio::spawn(async move {
  9 | |         let mut kilit = s.lock().unwrap();
 10 | |         tokio::time::sleep(Duration::from_millis(10)).await; // kilit elde tutulurken bekleme
 11 | |         *kilit += 1;
 12 | |     })
    | |______^ future created by async block is not `Send`
    |
    = help: within `{async block@src/main.rs:8:18: 8:28}`, the trait `Send` is not implemented for `std::sync::MutexGuard<'_, i32>`
note: future is not `Send` as this value is used across an await
   --> src/main.rs:10:55
    |
  9 |         let mut kilit = s.lock().unwrap();
    |             --------- has type `std::sync::MutexGuard<'_, i32>` which is not `Send`
 10 |         tokio::time::sleep(Duration::from_millis(10)).await; // kilit elde tutulurken bekleme
    |                                                       ^^^^^ await occurs here, with `mut kilit` maybe used later
note: required by a bound in `tokio::spawn`
   --> /usr/local/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.54.1/src/task/spawn.rs:176:21
    |
174 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
175 |     where
176 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`

error: could not compile `kilit_await` (bin "kilit_await") due to 1 previous error
[çıkış kodu: 101]
```

## 090-unsafe-ne-demek

- Video: `090-unsafe-ne-demek`
- Örnek yolu: `deneyler/mufredat-ornekleri/090-unsafe-ne-demek/`
- Kaynaklar: `ham-isaretci/Cargo.toml`, `ham-isaretci/src/main.rs`, `sozlesme/Cargo.toml`, `sozlesme/src/main.rs`, `surum-2024/Cargo.toml`, `surum-2024/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:04Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ham-isaretci && k cargo build -q; cd ..
# Hangi işlemler unsafe gerektirir? Derleyicinin hata açıklaması listeler
ks 'rustc --explain E0133 | head -n 12'
cd sozlesme && k cargo run -q; cd ..
# 2024 sürümünün kuralları: no_mangle ve extern blokları da "unsafe" diye işaretlenmeli
cd surum-2024 && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> src/main.rs:4:20
  |
4 |     println!("{}", *p); // izlemek (dereference) değil
  |                    ^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `ham_isaretci` (bin "ham_isaretci") due to 1 previous error
[çıkış kodu: 101]
$ rustc --explain E0133 | head -n 12
Unsafe code was used outside of an unsafe block.

Erroneous code example:

```
unsafe fn f() { return; } // This is the unsafe code

fn main() {
    f(); // error: call to unsafe function requires unsafe function or block
}
```

$ cargo run -q
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> src/main.rs:4:5
  |
4 |     *p // 2024 sürümünde: unsafe fn gövdesi de unsafe blok ister (uyarı)
  |     ^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
note: an unsafe function restricts its caller, but its body is safe by default
 --> src/main.rs:3:1
  |
3 | unsafe fn oku(p: *const i32) -> i32 {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>
  = note: `#[warn(unsafe_op_in_unsafe_fn)]` (part of `#[warn(rust_2024_compatibility)]`) on by default

For more information about this error, try `rustc --explain E0133`.
7
$ cargo build -q
error: extern blocks must be unsafe
 --> src/main.rs:6:1
  |
6 |   extern "C" {
  |   ^
  |   |
  |  _help: needs `unsafe` before the extern keyword: `unsafe`
  | |
7 | |     fn abs(x: i32) -> i32;
8 | | }
  | |_^

error: unsafe attribute used without unsafe
 --> src/main.rs:1:3
  |
1 | #[no_mangle]
  |   ^^^^^^^^^ usage of unsafe attribute
  |
help: wrap the attribute in `unsafe(...)`
  |
1 | #[unsafe(no_mangle)]
  |   +++++++         +

error: could not compile `surum_2024` (bin "surum_2024") due to 2 previous errors
[çıkış kodu: 101]
```

## 091-tanimsiz-davranis-ve-miri

- Video: `091-tanimsiz-davranis-ve-miri`
- Örnek yolu: `deneyler/mufredat-ornekleri/091-tanimsiz-davranis-ve-miri/`
- Kaynaklar: `Cargo.toml`, `src/bin/birak_sonra_kullan.rs`, `src/bin/gecersiz_bool.rs`, `src/bin/sinir_disi.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:05:04Z
- Değişken çıktı: sınır dışı ve bırakılmış bellekten okunan değerler tanımsızdır, her derlemede/çalıştırmada değişebilir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Normal çalıştırma: "çalışıyor gibi" görünür
k cargo run -q --bin sinir_disi
k cargo run -q --release --bin birak_sonra_kullan
k cargo run -q --release --bin gecersiz_bool
# miri: programı yorumlayarak her bellek erişimini denetler
ks 'cargo +nightly miri run -q --bin sinir_disi 2>&1 | head -n 12'
ks 'cargo +nightly miri run -q --bin birak_sonra_kullan 2>&1 | head -n 12'
ks 'cargo +nightly miri run -q --bin gecersiz_bool 2>&1 | head -n 10'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --bin sinir_disi
v[3] = 0
$ cargo run -q --release --bin birak_sonra_kullan
değer = 32767
$ cargo run -q --release --bin gecersiz_bool
doğru
$ cargo +nightly miri run -q --bin sinir_disi 2>&1 | head -n 12
error: Undefined Behavior: memory access failed: attempting to access 1 byte, but got alloc309+0x3 which is at or beyond the end of the allocation of size 3 bytes
 --> src/bin/sinir_disi.rs:4:27
  |
4 |     let disari = unsafe { *p.add(3) }; // 4. eleman yok: tanımsız davranış
  |                           ^^^^^^^^^ Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc309 was allocated here:
 --> src/bin/sinir_disi.rs:2:13
  |
2 |     let v = vec![10u8, 20, 30];
$ cargo +nightly miri run -q --bin birak_sonra_kullan 2>&1 | head -n 12
error: Undefined Behavior: memory access failed: alloc307 has been freed, so this pointer is dangling
 --> src/bin/birak_sonra_kullan.rs:6:37
  |
6 |     println!("değer = {}", unsafe { *p }); // bırakılmış bellek: tanımsız davranış
  |                                     ^^ Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc307 was allocated here:
 --> src/bin/birak_sonra_kullan.rs:3:20
  |
3 |         let kutu = Box::new(5u32);
$ cargo +nightly miri run -q --bin gecersiz_bool 2>&1 | head -n 10
error: Undefined Behavior: constructing invalid value of type bool: encountered 0x02, but expected a boolean
 --> src/bin/gecersiz_bool.rs:2:28
  |
2 |     let b: bool = unsafe { std::mem::transmute(2u8) }; // bool yalnızca 0 ya da 1 olabilir
  |                            ^^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
```

## 092-guvenli-soyutlama-yazmak

- Video: `092-guvenli-soyutlama-yazmak`
- Örnek yolu: `deneyler/mufredat-ornekleri/092-guvenli-soyutlama-yazmak/`
- Kaynaklar: `Cargo.toml`, `src/lib.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:05:06Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Normal testler: ikisi de "geçer" — hatalı sürümün tanımsız davranışı görünmez
k cargo test -q
# miri altında: doğru sürüm temiz, hatalı sürüm yakalanır
ks 'cargo +nightly miri test -q dogru_surum 2>&1 | grep -E "test result|running"'
ks 'cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "Undefined Behavior|error:|-->" | head -n 6'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo +nightly miri test -q dogru_surum 2>&1 | grep -E "test result|running"
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.11s
$ cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "Undefined Behavior|error:|-->" | head -n 6
error: Undefined Behavior: trying to retag from <130716> for Unique permission at alloc40299[0x4], but that tag does not exist in the borrow stack for this location
  --> src/lib.rs:23:9
  --> src/lib.rs:24:13
  --> src/lib.rs:25:13
error: aborting due to 1 previous error
error: test failed, to rerun pass `--lib`
```

## 093-ffi-c-ile-konusmak

- Video: `093-ffi-c-ile-konusmak`
- Örnek yolu: `deneyler/mufredat-ornekleri/093-ffi-c-ile-konusmak/`
- Kaynaklar: `Cargo.toml`, `build.rs`, `c/hesap.c`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `cc 1.11.3`, `find-msvc-tools 0.1.15`, `shlex 2.0.1`
- Son çalıştırma (UTC): 2026-11-20T16:05:07Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# İkilide iki dünyanın sembolleri yan yana
ks 'nm target/debug/ffi | grep -E " T (topla|selamla|karesi_arti_bir|rust_kare)$"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
topla(2, 3) = 5
C diyor ki: merhaba Ferris
strlen("Ferris") = 6
karesi_arti_bir(7) = 50 (C, Rust'taki rust_kare'yi çağırdı)
C dizgisinin baytları: [70, 101, 114, 114, 105, 115, 0]
$ nm target/debug/ffi | grep -E " T (topla|selamla|karesi_arti_bir|rust_kare)$"
0000000000016676 T karesi_arti_bir
0000000000016230 T rust_kare
000000000001663a T selamla
0000000000016626 T topla
```

## 094-bellek-esleme

- Video: `094-bellek-esleme`
- Örnek yolu: `deneyler/mufredat-ornekleri/094-bellek-esleme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `libc 0.2.191`
- Son çalıştırma (UTC): 2026-11-20T16:05:08Z
- Değişken çıktı: adres (ASLR) ve sayfa hatası sayısı (çekirdeğin bir hatada kaç sayfa eşlediğine bağlı)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
ks 'strace -e trace=openat,mmap,munmap ./target/release/mmap 2>&1 | grep -E "esleme.bin|67108864"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
eşlendi: 0x7f8bb182e000, ilk 7 bayt: "MERHABA"
16384 sayfaya dokunuldu → 492 küçük sayfa hatası (toplam = 77)
$ strace -e trace=openat,mmap,munmap ./target/release/mmap 2>&1 | grep -E "esleme.bin|67108864"
openat(AT_FDCWD, "/tmp/esleme.bin", O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0666) = 3
openat(AT_FDCWD, "/tmp/esleme.bin", O_RDONLY|O_CLOEXEC) = 3
mmap(NULL, 67108864, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7f749b51d000
munmap(0x7f749b51d000, 67108864)        = 0
```

## 095-bildirimsel-makrolar

- Video: `095-bildirimsel-makrolar`
- Örnek yolu: `deneyler/mufredat-ornekleri/095-bildirimsel-makrolar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:08Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
{"armut": 5, "ayva": 1, "elma": 3}
en_buyuk!(3, 9, 4, 7) = 9
iki_kati!(x) = 20 (C önişlemcisi gibi metin yapıştırsaydı 2 * 2 = 4 olurdu)
```

## 096-makro-genisletme

- Video: `096-makro-genisletme`
- Örnek yolu: `deneyler/mufredat-ornekleri/096-makro-genisletme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:05:08Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Makrolar genişletildikten sonra derleyicinin gördüğü kod (yalnızca nightly'de: -Z bayrağı)
k cargo +nightly rustc -q -- -Zunpretty=expanded
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo +nightly rustc -q -- -Zunpretty=expanded
#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
fn main() {
    let ad = "Ferris";
    let v =
        ::alloc::boxed::box_assume_init_into_vec_unsafe(::alloc::intrinsics::write_box_via_move(::alloc::boxed::Box::new_uninit(),
                [1, 2, 3]));
    {
        ::std::io::_print(format_args!("merhaba {1}, {0} eleman\n", v.len(),
                ad));
    };
}
```

## 097-prosedurel-makrolar

- Video: `097-prosedurel-makrolar`
- Örnek yolu: `deneyler/mufredat-ornekleri/097-prosedurel-makrolar/`
- Kaynaklar: `Cargo.toml`, `hatali-kullanim/Cargo.toml`, `hatali-kullanim/src/main.rs`, `kullanici/Cargo.toml`, `kullanici/src/main.rs`, `tanit-turet/Cargo.toml`, `tanit-turet/src/lib.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `proc-macro2 1.0.107`, `quote 1.0.47`, `syn 3.0.10`, `unicode-ident 1.0.27`
- Son çalıştırma (UTC): 2026-11-20T16:05:09Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q -p kullanici
k cargo build -q -p hatali-kullanim
ks 'cargo tree -p tanit-turet -e normal --depth 1'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q -p kullanici
Siparis alanları: ["numara", "musteri", "tutar_kurus"]
$ cargo build -q -p hatali-kullanim
error: Tanit yalnızca struct'lar için türetilebilir
 --> hatali-kullanim/src/main.rs:4:6
  |
4 | enum Durum {
  |      ^^^^^

error: could not compile `hatali-kullanim` (bin "hatali-kullanim") due to 1 previous error
[çıkış kodu: 101]
$ cargo tree -p tanit-turet -e normal --depth 1
tanit-turet v0.1.0 (proc-macro) (/work/mufredat-ornekleri/097-prosedurel-makrolar/tanit-turet)
├── proc-macro2 v1.0.107
├── quote v1.0.47
└── syn v3.0.10
```

## 098-lexer-ve-parser

- Video: `098-lexer-ve-parser`
- Örnek yolu: `deneyler/mufredat-ornekleri/098-lexer-ve-parser/`
- Kaynaklar: `ifade.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:05:09Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Ayrıştırıcının ürettiği soyut sözdizimi ağacı (AST): öncelik ağacın biçiminde
ks 'rustc +nightly --edition 2024 -Zunpretty=ast-tree ifade.rs | grep -E "kind: (Binary|Lit)|Add|Mul|symbol" '
# Sözdizimi hatası: ayrıştırıcı nerede takıldığını söyler
ks 'echo "fn main() { let x = (1 + 2; }" > /tmp/bozuk.rs && rustc --edition 2024 /tmp/bozuk.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ rustc +nightly --edition 2024 -Zunpretty=ast-tree ifade.rs | grep -E "kind: (Binary|Lit)|Add|Mul|symbol" 
                                                    kind: Binary(
                                                            node: Add,
                                                            kind: Lit(
                                                                    symbol: "1",
                                                            kind: Binary(
                                                                    node: Mul,
                                                                    kind: Lit(
                                                                            symbol: "2",
                                                                    kind: Lit(
                                                                            symbol: "3",
$ echo "fn main() { let x = (1 + 2; }" > /tmp/bozuk.rs && rustc --edition 2024 /tmp/bozuk.rs
error: mismatched closing delimiter: `}`
 --> /tmp/bozuk.rs:1:21
  |
1 | fn main() { let x = (1 + 2; }
  |           -         ^       ^ mismatched closing delimiter
  |           |         |
  |           |         unclosed delimiter
  |           closing delimiter possibly meant for this

error: aborting due to 1 previous error

[çıkış kodu: 1]
```

## 099-hir-ve-tip-denetimi

- Video: `099-hir-ve-tip-denetimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/099-hir-ve-tip-denetimi/`
- Kaynaklar: `seker.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:05:09Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# HIR: sözdizimsel şekerler (for, ?) açılmış hâli
ks 'rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn topla/,/^}/p"'
ks 'rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn oku/,/^}/p"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn topla/,/^}/p"
fn topla(v: &'_ [i32])
    ->
        i32 {
    let mut s = 0;
    {
        let _t =
            match into_iter(v) {
                mut iter =>
                    loop {
                        match next(&mut iter) {
                            None {} => break,
                            Some {  0: x } => { s += x; }
                        }
                    },
            };
        _t
    };
    s
}
$ rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn oku/,/^}/p"
fn oku(metin: &'_ str)
    ->
        Result<i32,
        std::num::ParseIntError> {
    let n: i32 =
        match branch(metin.parse()) {
            Break {  0: residual } => return from_residual(residual),
            Continue {  0: val } => val,
        };
    Ok(n + 1)
}
```

## 100-mir-ve-odunc-denetimi

- Video: `100-mir-ve-odunc-denetimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/100-mir-ve-odunc-denetimi/`
- Kaynaklar: `mir.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:09Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p target
# MIR: temel bloklardan (bb0, bb1...) oluşan kontrol akış grafiği; ödünç denetimi burada yapılır
k rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=mir -o target/mir.mir mir.rs
ks 'sed -n "/fn uzunluk/,/^}/p" target/mir.mir'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=mir -o target/mir.mir mir.rs
$ sed -n "/fn uzunluk/,/^}/p" target/mir.mir
fn uzunluk(_1: String) -> usize {
    debug s => _1;
    let mut _0: usize;
    let _2: std::string::String;
    let mut _3: &std::string::String;
    scope 1 {
        debug t => _2;
    }

    bb0: {
        _2 = move _1;
        _3 = &_2;
        _0 = String::len(move _3) -> [return: bb1, unwind: bb3];
    }

    bb1: {
        drop(_2) -> [return: bb2, unwind continue];
    }

    bb2: {
        return;
    }

    bb3 (cleanup): {
        drop(_2) -> [return: bb4, unwind terminate(cleanup)];
    }

    bb4 (cleanup): {
        resume;
    }
}
```

## 101-llvm-ve-optimizasyon

- Video: `101-llvm-ve-optimizasyon`
- Örnek yolu: `deneyler/mufredat-ornekleri/101-llvm-ve-optimizasyon/`
- Kaynaklar: `dongu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:10Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p target
k rustc --edition 2024 --crate-type=lib -C opt-level=0 -C debug-assertions=off -C overflow-checks=off --emit=llvm-ir -o target/O0.ll dongu.rs
k rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/O3.ll dongu.rs
ks 'echo "O0 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O0.ll | wc -l)  O3 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll | wc -l)"'
# O3: döngü (br/phi) kalmadı mı? Hangi komutlar var?
ks 'awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ rustc --edition 2024 --crate-type=lib -C opt-level=0 -C debug-assertions=off -C overflow-checks=off --emit=llvm-ir -o target/O0.ll dongu.rs
$ rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/O3.ll dongu.rs
$ echo "O0 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O0.ll | wc -l)  O3 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll | wc -l)"
O0 satır: 45  O3 satır: 29
$ awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll
define noundef i64 @kareler_toplami(i64 noundef %n) unnamed_addr #0 !guid !4 {
start:
  %_63.not = icmp eq i64 %n, 0
  br i1 %_63.not, label %bb3, label %bb2.preheader

bb2.preheader:                                    ; preds = %start
  %0 = add i64 %n, -1
  %1 = zext i64 %0 to i65
  %2 = add i64 %n, -2
  %3 = zext i64 %2 to i65
  %4 = mul i65 %1, %3
  %5 = add i64 %n, -3
  %6 = zext i64 %5 to i65
  %7 = mul i65 %4, %6
  %8 = lshr i65 %7, 1
  %9 = trunc nuw i65 %8 to i64
  %10 = mul i64 %9, 6148914691236517206
  %11 = add i64 %n, %10
  %12 = lshr i65 %4, 1
  %13 = trunc nuw i65 %12 to i64
  %14 = mul i64 %13, 3
  %15 = add i64 %11, %14
  %16 = add i64 %15, -1
  br label %bb3

bb3:                                              ; preds = %bb2.preheader, %start
  %s.sroa.0.0.lcssa = phi i64 [ 0, %start ], [ %16, %bb2.preheader ]
  ret i64 %s.sroa.0.0.lcssa
}
```

## 102-kod-uretimi-ve-artimli-derleme

- Video: `102-kod-uretimi-ve-artimli-derleme`
- Örnek yolu: `deneyler/mufredat-ornekleri/102-kod-uretimi-ve-artimli-derleme/`
- Kaynaklar: `uret.sh`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): 2026-11-20T16:05:39Z
- Değişken çıktı: derleme süreleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
rm -rf calisma && mkdir -p calisma && cd calisma && bash ../uret.sh && cd buyuk
k wc -l src/main.rs
ks '/usr/bin/time -f "temiz derleme (debug): %e s" cargo build -q'
ks '/usr/bin/time -f "değişiklik yok: %e s" cargo build -q'
ks 'sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "tek satır değişti (artımlı): %e s" cargo build -q'
ks 'CARGO_INCREMENTAL=0 /usr/bin/time -f "aynı değişiklik, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz'
ks 'sed -i "s/(x ^ 15001)/(x ^ 15002)/" src/main.rs && CARGO_INCREMENTAL=0 /usr/bin/time -f "tek satır değişti, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz'
# Kod üretim birimleri (codegen-units): paralel kod üretimi ↔ optimizasyon fırsatı
ks 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 /usr/bin/time -f "release, codegen-units=16: %e s" cargo build -q --release --target-dir target/cgu16'
ks 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 /usr/bin/time -f "release, codegen-units=1 : %e s" cargo build -q --release --target-dir target/cgu1'
ks './target/cgu16/release/buyuk; ./target/cgu1/release/buyuk'
# Zaman nereye gidiyor? Artımlı yeniden derlemede derleyici aşamaları (nightly, -Ztime-passes), en uzun 8 aşama
k cargo +nightly build -q --target-dir target/nightly
ks 'sed -i "s/(x ^ 15002)/(x ^ 15003)/" src/main.rs && cargo +nightly rustc -q --target-dir target/nightly -- -Ztime-passes 2>&1 | grep -E "time: +[0-9.]+;" | sort -k2 -n -r | head -n 8'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ wc -l src/main.rs
40004 src/main.rs
$ /usr/bin/time -f "temiz derleme (debug): %e s" cargo build -q
temiz derleme (debug): 4.01 s
$ /usr/bin/time -f "değişiklik yok: %e s" cargo build -q
değişiklik yok: 0.01 s
$ sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "tek satır değişti (artımlı): %e s" cargo build -q
tek satır değişti (artımlı): 2.61 s
$ CARGO_INCREMENTAL=0 /usr/bin/time -f "aynı değişiklik, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz
aynı değişiklik, artımlı kapalı: 3.28 s
$ sed -i "s/(x ^ 15001)/(x ^ 15002)/" src/main.rs && CARGO_INCREMENTAL=0 /usr/bin/time -f "tek satır değişti, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz
tek satır değişti, artımlı kapalı: 3.51 s
$ CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 /usr/bin/time -f "release, codegen-units=16: %e s" cargo build -q --release --target-dir target/cgu16
release, codegen-units=16: 4.04 s
$ CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 /usr/bin/time -f "release, codegen-units=1 : %e s" cargo build -q --release --target-dir target/cgu1
release, codegen-units=1 : 6.14 s
$ ./target/cgu16/release/buyuk; ./target/cgu1/release/buyuk
9356063799146033477
9356063799146033477
$ cargo +nightly build -q --target-dir target/nightly
$ sed -i "s/(x ^ 15002)/(x ^ 15003)/" src/main.rs && cargo +nightly rustc -q --target-dir target/nightly -- -Ztime-passes 2>&1 | grep -E "time: +[0-9.]+;" | sort -k2 -n -r | head -n 8
time:   2.419; rss:   32MB ->  277MB ( +245MB)	total
time:   1.021; rss:  566MB ->  527MB (  -39MB)	LLVM_passes
time:   0.924; rss:  501MB ->  494MB (   -7MB)	finish_ongoing_codegen
time:   0.325; rss:  455MB ->  577MB ( +122MB)	codegen_crate
time:   0.247; rss:  487MB ->  577MB (  +90MB)	codegen_to_LLVM_IR
time:   0.244; rss:  487MB ->  566MB (  +79MB)	compile_first_CGU_batch
time:   0.141; rss:  387MB ->  455MB (  +67MB)	misc_checking_3
time:   0.118; rss:   55MB ->  167MB ( +111MB)	parse_crate
```

## 103-olcum-bilimi

- Video: `103-olcum-bilimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/103-olcum-bilimi/`
- Kaynaklar: `black-box/Cargo.toml`, `black-box/src/main.rs`, `tuzak/Cargo.toml`, `tuzak/src/main.rs`, `tuzak-n-gizli/Cargo.toml`, `tuzak-n-gizli/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:38:19Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Tuzak 1: ilk sürümümüzde satır/sütun gezinme süreleri neredeyse aynı çıktı (045'in ilk hâli)
cd tuzak && k cargo run -q --release; cd ..
# Aynı kod, tek fark: matris boyutu n derleme anında bilinmiyor (black_box)
cd tuzak-n-gizli && ks 'cargo run -q --release | grep matris'; cd ..
# Tuzak 2: sonucu kullanılmayan hesap ölçülemez
cd black-box && k cargo run -q --release
# İstatistik: tek ölçüm değil, dağılım (hyperfine: ısınma, ortalama ± σ, aralık)
k cargo build -q --release
k hyperfine -N --warmup 3 --runs 30 --style basic ./target/release/black_box
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
matris satır-öncelikli gezinme         3.03 ms
matris sütun-öncelikli gezinme         2.39 ms
Vec<u64> toplamı                       1.27 ms
LinkedList<u64> toplamı                6.97 ms
AoS: Vec<Parcacik> içinden kütle       3.83 ms
SoA: Vec<f32> kütle dizisi             2.79 ms
$ cargo run -q --release | grep matris
matris satır-öncelikli gezinme         2.69 ms
matris sütun-öncelikli gezinme       117.23 ms
$ cargo run -q --release
sonuç atılınca      :      0.000 ms
black_box ile       :     31.996 ms
$ cargo build -q --release
$ hyperfine -N --warmup 3 --runs 30 --style basic ./target/release/black_box
Benchmark 1: ./target/release/black_box
  Time (mean ± σ):      32.8 ms ±   0.4 ms    [User: 32.0 ms, System: 0.5 ms]
  Range (min … max):    32.1 ms …  33.6 ms    30 runs
 
```

## 104-profil-cikarma

- Video: `104-profil-cikarma`
- Örnek yolu: `deneyler/mufredat-ornekleri/104-profil-cikarma/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:41Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/profil
# callgrind: her fonksiyonda yürütülen komut sayısı (zamanlamadan bağımsız, tekrarlanabilir)
ks 'valgrind --tool=callgrind --callgrind-out-file=/tmp/cg.out ./target/release/profil 2>&1 | grep -E "Collected|bulunan"'
# main'in çağırdıkları ve kapsayıcı (inclusive) maliyetleri; dosya yolları kısaltıldı
ks 'callgrind_annotate --inclusive=yes /tmp/cg.out 2>/dev/null | grep -E "PROGRAM TOTALS|=> .*profil::(yavas_arama|hizli_hazirlik|hizli_arama) " | sed -E "s#=> .*:(profil::[a-z_]+) #=> \1 #"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/profil
bulunan: 2000 / 2000
$ valgrind --tool=callgrind --callgrind-out-file=/tmp/cg.out ./target/release/profil 2>&1 | grep -E "Collected|bulunan"
bulunan: 2000 / 2000
==20456== Collected : 24591969
$ callgrind_annotate --inclusive=yes /tmp/cg.out 2>/dev/null | grep -E "PROGRAM TOTALS|=> .*profil::(yavas_arama|hizli_hazirlik|hizli_arama) " | sed -E "s#=> .*:(profil::[a-z_]+) #=> \1 #"
24,591,969 (100.0%)  PROGRAM TOTALS
21,276,515 (86.52%)  => profil::yavas_arama (1x)
2,671,669 (10.86%)  => profil::hizli_hazirlik (1x)
292,013 ( 1.19%)  => profil::hizli_arama (1x)
```

## 105-dallanma-tahmini

- Video: `105-dallanma-tahmini`
- Örnek yolu: `deneyler/mufredat-ornekleri/105-dallanma-tahmini/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:42Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/dallanma
# Dallanan sürüm: döngünün içinde koşullu atlama (jns: işaret biti 0 ise atla → x < 128)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dallanan>:/,/^$/" | grep -E "\bj(ns|s)\b"'
# Dalsız sürüm: vektör karşılaştırma (pcmpgtb) + maske (pand); döngüde veri bağımlı atlama yok
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dalsiz>:/,/^$/" | grep -oE "\b(pcmpgtb|pand|paddq)\b" | sort | uniq -c'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/dallanma
dallanan, karışık veri          272.2 ms   (toplam 9577481740)
dalsız,   karışık veri           29.1 ms   (toplam 9577481740)
dallanan, sıralı veri            28.9 ms   (toplam 9577481740)
dalsız,   sıralı veri            29.2 ms   (toplam 9577481740)
$ objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dallanan>:/,/^$/" | grep -E "\bj(ns|s)\b"
   1829e:	jns    18290 <dallanma::dallanan+0x20>
$ objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dalsiz>:/,/^$/" | grep -oE "\b(pcmpgtb|pand|paddq)\b" | sort | uniq -c
      4 paddq
      2 pand
      2 pcmpgtb
```

## 106-simd-ve-vektorlestirme

- Video: `106-simd-ve-vektorlestirme`
- Örnek yolu: `deneyler/mufredat-ornekleri/106-simd-ve-vektorlestirme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:43Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/simd
# Hangi toplama komutları? addss: tek (skaler) f32, addps: 4'lü paket (SSE)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_basit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c'
# Bu işlemciye özel derleme: AVX2 ile 8'li paketler (vaddps ymm)
ks 'RUSTFLAGS="-C target-cpu=native" cargo build -q --release --target-dir target/native && ./target/native/release/simd'
ks 'objdump -d -M intel --no-show-raw-insn -C target/native/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -E "vaddps +ymm" | head -n 2'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/simd
f32, basit toplam        11.02 ms  sonuç 21960720
f32, 8 şeritli toplam     2.12 ms  sonuç 23999998
i32, toplam               2.14 ms  sonuç 47999995
$ objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_basit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c
      9 addss
$ objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c
     10 addps
      9 addss
$ RUSTFLAGS="-C target-cpu=native" cargo build -q --release --target-dir target/native && ./target/native/release/simd
f32, basit toplam        10.85 ms  sonuç 21960720
f32, 8 şeritli toplam     1.81 ms  sonuç 23999998
i32, toplam               1.94 ms  sonuç 47999995
$ objdump -d -M intel --no-show-raw-insn -C target/native/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -E "vaddps +ymm" | head -n 2
   178d0:	vaddps ymm0,ymm0,YMMWORD PTR [rcx]
   17920:	vaddps ymm0,ymm0,YMMWORD PTR [rcx+rdx*4]
```

## 107-derleyici-ayarlari-lto-pgo

- Video: `107-derleyici-ayarlari-lto-pgo`
- Örnek yolu: `deneyler/mufredat-ornekleri/107-derleyici-ayarlari-lto-pgo/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `itoa 1.0.19`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.10`, `unicode-ident 1.0.27`, `zmij 1.0.23`
- Son çalıştırma (UTC): 2026-11-20T16:05:54Z
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k cargo build -q --profile kucuk
k cargo build -q --profile hizli
ks 'stat -c "%8s bayt  %n" target/release/ayarlar target/kucuk/ayarlar target/hizli/ayarlar'
k hyperfine -N --warmup 2 --runs 10 --style basic ./target/release/ayarlar ./target/kucuk/ayarlar ./target/hizli/ayarlar
# PGO (profil güdümlü optimizasyon) için llvm-profdata gerekir: bu imajda kurulu mu?
ks 'rustup component list --installed | grep -c llvm-tools; command -v llvm-profdata || echo "llvm-profdata yok"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ cargo build -q --profile kucuk
$ cargo build -q --profile hizli
$ stat -c "%8s bayt  %n" target/release/ayarlar target/kucuk/ayarlar target/hizli/ayarlar
  544360 bayt  target/release/ayarlar
  318808 bayt  target/kucuk/ayarlar
  440712 bayt  target/hizli/ayarlar
$ hyperfine -N --warmup 2 --runs 10 --style basic ./target/release/ayarlar ./target/kucuk/ayarlar ./target/hizli/ayarlar
Benchmark 1: ./target/release/ayarlar
  Time (mean ± σ):      15.8 ms ±   1.6 ms    [User: 15.2 ms, System: 0.3 ms]
  Range (min … max):    14.2 ms …  19.7 ms    10 runs
 
Benchmark 2: ./target/kucuk/ayarlar
  Time (mean ± σ):      20.8 ms ±   0.3 ms    [User: 20.2 ms, System: 0.5 ms]
  Range (min … max):    20.4 ms …  21.4 ms    10 runs
 
Benchmark 3: ./target/hizli/ayarlar
  Time (mean ± σ):      12.8 ms ±   0.2 ms    [User: 12.4 ms, System: 0.3 ms]
  Range (min … max):    12.5 ms …  13.0 ms    10 runs
 
Summary
  ./target/hizli/ayarlar ran
    1.24 ± 0.13 times faster than ./target/release/ayarlar
    1.63 ± 0.03 times faster than ./target/kucuk/ayarlar
$ rustup component list --installed | grep -c llvm-tools; command -v llvm-profdata || echo "llvm-profdata yok"
0
llvm-profdata yok
```

## 108-tip-gudumlu-tasarim

- Video: `108-tip-gudumlu-tasarim`
- Örnek yolu: `deneyler/mufredat-ornekleri/108-tip-gudumlu-tasarim/`
- Kaynaklar: `durum-tipleri/Cargo.toml`, `durum-tipleri/src/main.rs`, `yanlis-gecis/Cargo.toml`, `yanlis-gecis/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:54Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd durum-tipleri && k cargo run -q; cd ..
cd yanlis-gecis && ks 'cargo build -q 2>&1 | head -n 16'; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
2 kalem gönderildi
Some("boş sipariş onaylanamaz")
ayse@ornek.com adresine: 2 kalem gönderildi
geçersiz e-posta: "ayse.ornek.com"
$ cargo build -q 2>&1 | head -n 16
error[E0599]: no method named `gonder` found for struct `Siparis<Taslak>` in the current scope
  --> src/main.rs:57:47
   |
 8 | struct Siparis<Durum> {
   | --------------------- method `gonder` not found for this struct
...
57 |     let s = Siparis::yeni().ekle("defter", 2).gonder(); // onaylamadan göndermeye çalış
   |                                               ^^^^^^ method not found in `Siparis<Taslak>`
   |
   = note: the method was found for `Siparis<Onayli>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `yanlis_gecis` (bin "yanlis_gecis") due to 1 previous error
```

## 109-hata-sinirlari-ve-katmanlar

- Video: `109-hata-sinirlari-ve-katmanlar`
- Örnek yolu: `deneyler/mufredat-ornekleri/109-hata-sinirlari-ve-katmanlar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:54Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
defter × 3: 200 sipariş alındı, kalan stok 9
defter × 50: 400 defter: 50 istendi, stokta 12 var
defter × 0: 400 adet en az 1 olmalı
silgi × 1: 500 geçici bir sorun oluştu, lütfen tekrar deneyin
    [günlük] altyapı hatası: No such file or directory (os error 2)
```

## 110-api-tasarimi

- Video: `110-api-tasarimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/110-api-tasarimi/`
- Kaynaklar: `Cargo.toml`, `istemci/Cargo.toml`, `istemci/src/bin/duzgun.rs`, `istemci/src/main.rs`, `kutuphane/Cargo.toml`, `kutuphane/src/lib.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:54Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# non_exhaustive: dışarıdaki crate, joker (_) kol olmadan eşleştiremez
k cargo build -q -p istemci --bin istemci
# Joker kol eklenmiş sürüm: derlenir; must_use uyarısı atılan dönüş değerini yakalar
k cargo run -q -p istemci --bin duzgun
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q -p istemci --bin istemci
error[E0004]: non-exhaustive patterns: `_` not covered
  --> istemci/src/main.rs:4:11
   |
 4 |     match h {
   |           ^ pattern `_` not covered
   |
note: `Hata` defined here
  --> kutuphane/src/lib.rs:23:1
   |
23 | pub enum Hata {
   | ^^^^^^^^^^^^^
   = note: the matched value is of type `Hata`
   = note: `Hata` is marked as non-exhaustive, so a wildcard `_` is necessary to match exhaustively
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
 6 ~         Hata::YetkiYok => "yetki yok",
 7 ~         _ => todo!(),
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `istemci` (bin "istemci") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q -p istemci --bin duzgun
warning: unused return value of `Kullanici::yeniden_adlandir` that must be used
  --> istemci/src/bin/duzgun.rs:17:5
   |
17 |     k.yeniden_adlandir("Cem"); // dönüş değeri atıldı
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: yeni bir Kullanici döndürür, mevcut olanı değiştirmez
   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
   |
17 |     let _ = k.yeniden_adlandir("Cem"); // dönüş değeri atıldı
   |     +++++++

Merhaba, Ali!
Merhaba, Ayşe!
Merhaba, Ayşe!
Can bulunamadı
```

## 111-test-edilebilir-mimari

- Video: `111-test-edilebilir-mimari`
- Örnek yolu: `deneyler/mufredat-ornekleri/111-test-edilebilir-mimari/`
- Kaynaklar: `Cargo.toml`, `src/lib.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:55Z
- Değişken çıktı: test süresi
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo test -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q

running 4 tests
....
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 112-gozlemlenebilirlik

- Video: `112-gozlemlenebilirlik`
- Örnek yolu: `deneyler/mufredat-ornekleri/112-gozlemlenebilirlik/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `cfg-if 1.0.5`, `lazy_static 1.5.1`, `log 0.4.35`, `nu-ansi-term 0.50.3`, `once_cell 1.21.4`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `sharded-slab 0.1.7`, `smallvec 1.16.6`, `syn 2.0.119`, `thread_local 1.1.10`, `tracing 0.1.44`, `tracing-attributes 0.1.31`, `tracing-core 0.1.36`, `tracing-log 0.2.0`, `tracing-subscriber 0.3.23`, `unicode-ident 1.0.27`, `valuable 0.1.1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:05:57Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
 INFO siparis_isle{id=41 kalem_sayisi=2}: işleniyor
 INFO siparis_isle{id=41 kalem_sayisi=2}:odeme_al{tutar=500}: onaylandı
DEBUG siparis_isle{id=41 kalem_sayisi=2}: ödeme tamam tutar=500
 INFO siparis_isle{id=42 kalem_sayisi=0}: işleniyor
 WARN siparis_isle{id=42 kalem_sayisi=0}: boş sipariş
 INFO siparis_isle{id=43 kalem_sayisi=7}: işleniyor
ERROR siparis_isle{id=43 kalem_sayisi=7}:odeme_al{tutar=1750}: limit aşıldı tutar=1750 sinir=1000
```

## 113-ajan-kodunu-okumak

- Video: `113-ajan-kodunu-okumak`
- Örnek yolu: `deneyler/mufredat-ornekleri/113-ajan-kodunu-okumak/`
- Kaynaklar: `ajanin-yazdigi/Cargo.toml`, `ajanin-yazdigi/src/lib.rs`, `incelenmis/Cargo.toml`, `incelenmis/src/lib.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:58Z
- Değişken çıktı: iş parçacığı kimlikleri, test süreleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Ajanın kendi testleri geçiyor:
cd ajanin-yazdigi
ks 'cargo test -q ajanin_testleri 2>&1 | grep "test result"'
# İncelemede eklediğimiz sınır testleri:
ks 'cargo test -q inceleme_testleri 2>&1 | grep -E "^---- |panicked|attempt|ParseIntError|test result"'
cd ../incelenmis
ks 'cargo test -q 2>&1 | grep "test result" | head -n 1'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q ajanin_testleri 2>&1 | grep "test result"
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
$ cargo test -q inceleme_testleri 2>&1 | grep -E "^---- |panicked|attempt|ParseIntError|test result"
---- inceleme_testleri::bozuk_girdi stdout ----
thread 'inceleme_testleri::bozuk_girdi' (22120) panicked at src/lib.rs:4:54:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
---- inceleme_testleri::yuzden_fazla stdout ----
thread 'inceleme_testleri::yuzden_fazla' (22122) panicked at src/lib.rs:5:5:
attempt to subtract with overflow
test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
$ cargo test -q 2>&1 | grep "test result" | head -n 1
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 114-inceleme-listesi

- Video: `114-inceleme-listesi`
- Örnek yolu: `deneyler/mufredat-ornekleri/114-inceleme-listesi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:05:58Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# Varsayılan clippy ne diyor? (lint adları uyarıların yardım bağlantısından)
ks 'cargo clippy -q 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c'
# İnceleme için sıkı mod: pedantic + unwrap_used
ks 'cargo clippy -q -- -W clippy::pedantic -W clippy::unwrap_used 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c | sort -rn'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
2 kayıt, toplam 48 bayt, ilk: ELMA
$ cargo clippy -q 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c
      1 iter_next_slice
$ cargo clippy -q -- -W clippy::pedantic -W clippy::unwrap_used 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c | sort -rn
      2 unwrap_used
      2 needless_pass_by_value
      1 missing_panics_doc
      1 iter_next_slice
      1 implicit_hasher
      1 cast_possible_truncation
      1 cast_lossless
```

## 115-araclarla-dogrulama

- Video: `115-araclarla-dogrulama`
- Örnek yolu: `deneyler/mufredat-ornekleri/115-araclarla-dogrulama/`
- Kaynaklar: `Cargo.toml`, `bariz/Cargo.toml`, `bariz/src/lib.rs`, `src/lib.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Bağımlılıklar (Cargo.lock'ta çözülen): `autocfg 1.5.1`, `bit-set 0.11.1`, `bit-vec 0.10.2`, `bitflags 2.13.2`, `borsh 1.8.1`, `borsh-derive 1.8.1`, `bytes 1.12.2`, `cfg-if 1.0.5`, `cfg_aliases 0.2.2`, `chacha20 0.10.2`, `core_detect 1.0.0`, `cpufeatures 0.3.1`, `equivalent 1.0.3`, `errno 0.3.15`, `fastrand 2.5.0`, `fnv 1.0.7`, `getrandom 0.4.3`, `hashbrown 0.17.1`, `indexmap 2.14.2`, `libc 0.2.191`, `linux-raw-sys 0.12.1`, `memchr 2.8.3`, `num-traits 0.2.19`, `once_cell 1.21.4`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `proptest 1.12.0`, `quick-error 1.2.3`, `quote 1.0.47`, `r-efi 6.0.0`, `rand 0.10.4`, `rand_core 0.10.1`, `rand_xorshift 0.5.0`, `regex-syntax 0.8.12`, `rustix 1.1.6`, `rusty-fork 0.3.1`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `syn 3.0.10`, `tempfile 3.27.0`, `toml_datetime 1.1.3+spec-1.1.0`, `toml_edit 0.25.19+spec-1.1.0`, `toml_parser 1.1.5+spec-1.1.0`, `unarray 0.1.4`, `unicode-ident 1.0.27`, `wait-timeout 0.2.1`, `windows-link 0.2.1`, `windows-sys 0.61.2`, `winnow 1.0.5`
- Son çalıştırma (UTC): 2026-11-20T16:06:05Z
- Değişken çıktı: proptest'in bulduğu en küçük karşı örnek ve süreler değişebilir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
rm -rf proptest-regressions
# 0) Bariz hâli: derleyicinin varsayılan "deny" uyarısı yakalar
cd bariz && k cargo build -q; cd ..
# 1) Dolaylı hâli derlenir; örnek testler geçer, farklılık (diferansiyel) testi patlar
ks 'cargo test -q 2>&1 | grep -E -A4 "^---- |minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"; cargo test -q 2>&1 | grep "test result"'
# 2) miri: "geçen" sayaç testindeki tanımsız davranışı yakalar
ks 'cargo +nightly miri test -q sayac_artiyor 2>&1 | grep -E "Undefined Behavior|-->|test result" | head -n 4'
rm -rf proptest-regressions
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error: assigning to `&T` is undefined behavior, consider using an `UnsafeCell`
 --> src/lib.rs:5:9
  |
3 |     let p = sayac as *const u32 as *mut u32;
  |             ------------------------------- casting happened here
4 |     unsafe {
5 |         *p += 1;
  |         ^^^^^^^
  |
  = note: for more information, visit <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html>
  = note: `#[deny(invalid_reference_casting)]` on by default

error: could not compile `bariz` (lib) due to 1 previous error
[çıkış kodu: 101]
$ cargo test -q 2>&1 | grep -E -A4 "^---- |minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"; cargo test -q 2>&1 | grep "test result"
proptest: Saving this and future failures in /work/mufredat-ornekleri/115-araclarla-dogrulama/proptest-regressions/lib.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc c4d598a2571a78b814cf17328f8543392968da0c0ae210f9dca2e8b87aef731c
minimal failing input: v = [
    0,
    0,
    0,
    1,
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo +nightly miri test -q sayac_artiyor 2>&1 | grep -E "Undefined Behavior|-->|test result" | head -n 4
error: Undefined Behavior: attempting a write access using <130405> at alloc41025[0x0], but that tag only grants SharedReadOnly permission for this location
 --> src/lib.rs:3:14
 --> src/lib.rs:8:19
```

## 116-dogru-soruyu-sormak

- Video: `116-dogru-soruyu-sormak`
- Örnek yolu: `deneyler/mufredat-ornekleri/116-dogru-soruyu-sormak/`
- Kaynaklar: `1-sozlesme/Cargo.toml`, `1-sozlesme/src/lib.rs`, `2-uygulama/Cargo.toml`, `2-uygulama/src/lib.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:05Z
- Değişken çıktı: iş parçacığı kimlikleri, test süreleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Önce sözleşme: imza + belge + testler; gövde todo!()
cd 1-sozlesme && ks 'cargo test -q 2>&1 | grep -E "not yet implemented|test result" | sort | uniq -c'; cd ..
# Sonra uygulama (ajan ya da insan): aynı testler
cd 2-uygulama && ks 'cargo test -q 2>&1 | grep "test result" | head -n 1'; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q 2>&1 | grep -E "not yet implemented|test result" | sort | uniq -c
      5 not yet implemented: henüz yazılmadı: önce sözleşme ve testler
      1 test result: FAILED. 0 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo test -q 2>&1 | grep "test result" | head -n 1
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 117-ambar-tasarim

- Video: `117-ambar-tasarim`
- Örnek yolu: `deneyler/mufredat-ornekleri/117-ambar-tasarim/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/ambar/` — kaynaklar: `Cargo.toml`, `src/bin/ambar.rs`, `src/crc.rs`, `src/gunluk.rs`, `src/lib.rs`, `src/motor.rs`, `src/protokol.rs`, `src/sunucu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:06Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/ambar
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
k ./target/release/ambar demo calisma/veri
# Süreç bitti, bellek gitti. Yeni bir süreç günlüğü baştan okuyarak durumu yeniden kurar:
k ./target/release/ambar kurtar calisma/veri
# Günlük dosyasının baytları: 5 kayıt (SET, SET, üzerine yazma, silme, SET)
k od -A d -c calisma/veri/gunluk.ambar
rm -rf calisma
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/ambar demo calisma/veri
bellekteki durum: [("dil", "Rust"), ("maskot", "Ferris")]
$ ./target/release/ambar kurtar calisma/veri
geçerli kayıt: 5, geçerli bayt: 96, atılan bayt: 0
anahtar sayısı: 2
  dil = Rust
  maskot = Ferris
$ od -A d -c calisma/veri/gunluk.ambar
0000000 210 353 301 272 003  \0  \0  \0 004  \0  \0  \0   d   i   l   r
0000016   u   s   t 021   ] 031 222 003  \0  \0  \0 004  \0  \0  \0   y
0000032   i   l   2   0   1   5 266   D 363 032 003  \0  \0  \0 004  \0
0000048  \0  \0   d   i   l   R   u   s   t   .   Y 355   ; 003  \0  \0
0000064  \0 377 377 377 377   y   i   l   b 031 307   N 006  \0  \0  \0
0000080 006  \0  \0  \0   m   a   s   k   o   t   F   e   r   r   i   s
0000096
```

## 118-ambar-depolama-motoru

- Video: `118-ambar-depolama-motoru`
- Örnek yolu: `deneyler/mufredat-ornekleri/118-ambar-depolama-motoru/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/ambar/` — kaynaklar: `Cargo.toml`, `src/bin/ambar.rs`, `src/crc.rs`, `src/gunluk.rs`, `src/lib.rs`, `src/motor.rs`, `src/protokol.rs`, `src/sunucu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:06Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/ambar
ks 'cargo test -q --lib 2>&1 | grep "test result"'
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
k ./target/release/ambar doldur calisma/veri 1000
ks 'stat -c "%s bayt" calisma/veri/gunluk.ambar'
# 1) Yarım yazma: son kaydın son 5 baytı diske hiç ulaşmamış gibi
k truncate -s -5 calisma/veri/gunluk.ambar
k ./target/release/ambar kurtar calisma/veri
# 2) Sessiz bozulma, kaydın verisinde: tek bir bayt değişsin
rm -rf calisma/veri && ./target/release/ambar doldur calisma/veri 1000 > /dev/null
ks 'printf "X" | dd of=calisma/veri/gunluk.ambar bs=1 seek=16910 conv=notrunc status=none'
k ./target/release/ambar kurtar calisma/veri
# 3) Sessiz bozulma, kaydın başlığında (uzunluk alanı): kayıt sınırı kaybolur
rm -rf calisma/veri && ./target/release/ambar doldur calisma/veri 1000 > /dev/null
ks 'printf "X" | dd of=calisma/veri/gunluk.ambar bs=1 seek=16900 conv=notrunc status=none'
k ./target/release/ambar kurtar calisma/veri
rm -rf calisma
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q --lib 2>&1 | grep "test result"
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo build -q --release
$ ./target/release/ambar doldur calisma/veri 1000
1000 kayıt yazıldı
$ stat -c "%s bayt" calisma/veri/gunluk.ambar
33890 bayt
$ truncate -s -5 calisma/veri/gunluk.ambar
$ ./target/release/ambar kurtar calisma/veri
geçerli kayıt: 999, geçerli bayt: 33856, atılan bayt: 29
kurtarma notu: 33856. baytta yarım kayıt (29/34 bayt)
anahtar sayısı: 999
  anahtar00000 = değer-0
  anahtar00001 = değer-1
  anahtar00002 = değer-2
$ printf "X" | dd of=calisma/veri/gunluk.ambar bs=1 seek=16910 conv=notrunc status=none
$ ./target/release/ambar kurtar calisma/veri
geçerli kayıt: 500, geçerli bayt: 16890, atılan bayt: 17000
kurtarma notu: 16890. baytta CRC uyuşmazlığı (bozuk kayıt)
anahtar sayısı: 500
  anahtar00000 = değer-0
  anahtar00001 = değer-1
  anahtar00002 = değer-2
$ printf "X" | dd of=calisma/veri/gunluk.ambar bs=1 seek=16900 conv=notrunc status=none
$ ./target/release/ambar kurtar calisma/veri
geçerli kayıt: 500, geçerli bayt: 16890, atılan bayt: 17000
kurtarma notu: 16890. baytta yarım kayıt (17000/5767202 bayt)
anahtar sayısı: 500
  anahtar00000 = değer-0
  anahtar00001 = değer-1
  anahtar00002 = değer-2
```

## 119-ambar-ag-protokolu

- Video: `119-ambar-ag-protokolu`
- Örnek yolu: `deneyler/mufredat-ornekleri/119-ambar-ag-protokolu/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/ambar/` — kaynaklar: `Cargo.toml`, `src/bin/ambar.rs`, `src/crc.rs`, `src/gunluk.rs`, `src/lib.rs`, `src/motor.rs`, `src/protokol.rs`, `src/sunucu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:06Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/ambar
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
./target/release/ambar sunucu 127.0.0.1:7001 calisma/veri her 2> calisma/sunucu.log &
SP=$!
sleep 0.5
k ./target/release/ambar istemci 127.0.0.1:7001 "SET dil rust" "GET dil" "SET selam merhaba dünya" "GET selam" "DEL dil" "GET dil" "SAY" "UCUR hepsini"
# Protokol düz metin: istemci olmadan, ham TCP ile de konuşulabilir
ks 'exec 3<>/dev/tcp/127.0.0.1/7001; printf "GET selam\nSAY\n" >&3; head -n 2 <&3'
kill $SP
k cat calisma/sunucu.log
rm -rf calisma
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/ambar istemci 127.0.0.1:7001 "SET dil rust" "GET dil" "SET selam merhaba dünya" "GET selam" "DEL dil" "GET dil" SAY "UCUR hepsini"
> SET dil rust
< OK
> GET dil
< DEGER rust
> SET selam merhaba dünya
< OK
> GET selam
< DEGER merhaba dünya
> DEL dil
< OK
> GET dil
< YOK
> SAY
< SAYI 1
> UCUR hepsini
< HATA bilinmeyen komut: "UCUR hepsini"
$ exec 3<>/dev/tcp/127.0.0.1/7001; printf "GET selam\nSAY\n" >&3; head -n 2 <&3
DEGER merhaba dünya
SAYI 1
$ cat calisma/sunucu.log
kurtarıldı: 0 kayıt
ambar dinliyor: 127.0.0.1:7001
```

## 120-ambar-eszamanlilik-ve-dayaniklilik

- Video: `120-ambar-eszamanlilik-ve-dayaniklilik`
- Örnek yolu: `deneyler/mufredat-ornekleri/120-ambar-eszamanlilik-ve-dayaniklilik/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/ambar/` — kaynaklar: `Cargo.toml`, `src/bin/ambar.rs`, `src/crc.rs`, `src/gunluk.rs`, `src/lib.rs`, `src/motor.rs`, `src/protokol.rs`, `src/sunucu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:29:49Z
- Değişken çıktı: süre, onaylanan yazma sayısı (sunucunun öldürüldüğü ana bağlı)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/ambar
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
# 1) 8 istemci aynı anda yazıyor: tek bir yazma bile kayboluyor mu?
./target/release/ambar sunucu 127.0.0.1:7002 calisma/eszamanli yok 2>/dev/null &
SP=$!
sleep 0.5
k ./target/release/ambar yuk 127.0.0.1:7002 8 2000
kill $SP
# 2) Çökme deneyi: sunucu "her yazmada fsync" ile çalışırken kill -9
./target/release/ambar sunucu 127.0.0.1:7003 calisma/cokme her 2>/dev/null &
SP=$!
sleep 0.5
./target/release/ambar yazici 127.0.0.1:7003 100000000 > calisma/yazici.txt &
YP=$!
sleep 2
{ kill -9 $SP; wait $SP; } 2>/dev/null
wait $YP
k cat calisma/yazici.txt
k ./target/release/ambar kurtar calisma/cokme
# 3) Aynı deney fsync olmadan: süreç ölse de çekirdeğin sayfa önbelleği yaşıyor
./target/release/ambar sunucu 127.0.0.1:7004 calisma/cokme_yok yok 2>/dev/null &
SP=$!
sleep 0.5
./target/release/ambar yazici 127.0.0.1:7004 100000000 > calisma/yazici_yok.txt &
YP=$!
sleep 2
{ kill -9 $SP; wait $SP; } 2>/dev/null
wait $YP
k cat calisma/yazici_yok.txt
ks './target/release/ambar kurtar calisma/cokme_yok | grep "geçerli kayıt"'
rm -rf calisma
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/ambar yuk 127.0.0.1:7002 8 2000
8 istemci × 2000 SET = 16000 yazma, 0.19 s, 85384 yazma/s, sunucu: SAYI 16000
$ cat calisma/yazici.txt
onaylanan yazma: 2047
$ ./target/release/ambar kurtar calisma/cokme
geçerli kayıt: 2048, geçerli bayt: 50090, atılan bayt: 0
anahtar sayısı: 2048
  k0000000 = v0
  k0000001 = v1
  k0000002 = v2
$ cat calisma/yazici_yok.txt
onaylanan yazma: 108976
$ ./target/release/ambar kurtar calisma/cokme_yok | grep "geçerli kayıt"
geçerli kayıt: 108976, geçerli bayt: 2831242, atılan bayt: 0
```

## 121-ambar-performans

- Video: `121-ambar-performans`
- Örnek yolu: `deneyler/mufredat-ornekleri/121-ambar-performans/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/ambar/` — kaynaklar: `Cargo.toml`, `src/bin/ambar.rs`, `src/crc.rs`, `src/gunluk.rs`, `src/lib.rs`, `src/motor.rs`, `src/protokol.rs`, `src/sunucu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:17Z
- Değişken çıktı: yazma/s değerleri diske ve yüke bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/ambar
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
port=7010
for kip in her toplu yok; do
  port=$((port + 1))
  ./target/release/ambar sunucu 127.0.0.1:$port calisma/p_$kip $kip 2>/dev/null &
  SP=$!
  sleep 0.5
  echo "# fsync kipi: $kip (veri: ana makinenin diski, btrfs)"
  k ./target/release/ambar yuk 127.0.0.1:$port 16 300
  kill $SP
done
# Tuzak: aynı "her" kipi konteynerin kendi /tmp'sinde (overlay, fsync=volatile)
./target/release/ambar sunucu 127.0.0.1:7019 /tmp/ambar_her her 2>/dev/null &
SP=$!
sleep 0.5
echo "# fsync kipi: her (veri: konteynerin /tmp'si)"
k ./target/release/ambar yuk 127.0.0.1:7019 16 300
kill $SP
rm -rf calisma /tmp/ambar_her
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
# fsync kipi: her (veri: ana makinenin diski, btrfs)
$ ./target/release/ambar yuk 127.0.0.1:7011 16 300
16 istemci × 300 SET = 4800 yazma, 4.89 s, 981 yazma/s, sunucu: SAYI 4800
# fsync kipi: toplu (veri: ana makinenin diski, btrfs)
$ ./target/release/ambar yuk 127.0.0.1:7012 16 300
16 istemci × 300 SET = 4800 yazma, 0.95 s, 5065 yazma/s, sunucu: SAYI 4800
# fsync kipi: yok (veri: ana makinenin diski, btrfs)
$ ./target/release/ambar yuk 127.0.0.1:7013 16 300
16 istemci × 300 SET = 4800 yazma, 0.04 s, 114916 yazma/s, sunucu: SAYI 4800
# fsync kipi: her (veri: konteynerin /tmp'si)
$ ./target/release/ambar yuk 127.0.0.1:7019 16 300
16 istemci × 300 SET = 4800 yazma, 0.04 s, 135009 yazma/s, sunucu: SAYI 4800
```

## 122-http-ayristirici

- Video: `122-http-ayristirici`
- Örnek yolu: `deneyler/mufredat-ornekleri/122-http-ayristirici/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/kapi/` — kaynaklar: `Cargo.toml`, `src/ayristirici.rs`, `src/bin/kapi.rs`, `src/dosya.rs`, `src/havuz.rs`, `src/lib.rs`, `src/zaman_uyumsuz.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:06:28Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/kapi
ks 'cargo test -q --lib -- --list 2>/dev/null | grep ": test$"'
ks 'cargo test -q --lib 2>&1 | grep "test result"'
k cargo build -q --release
# Durum makinesi parça parça beslenirken
k ./target/release/kapi parca-parca
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q --lib -- --list 2>/dev/null | grep ": test$"
ayristirici::testler::bayt_bayt_beslemek_ayni_sonucu_verir: test
ayristirici::testler::bozuk_istek_satiri: test
ayristirici::testler::sonsuz_baslik_reddedilir: test
ayristirici::testler::tek_parcada_iki_istek: test
$ cargo test -q --lib 2>&1 | grep "test result"
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo build -q --release
$ ./target/release/kapi parca-parca
besle("GET /belgeler/not") → Ok(None)
besle(".txt HTTP/1.1\r\nHo") → Ok(None)
besle("st: kapi\r\n") → Ok(None)
besle("\r\n") → Ok(Some(("GET", "/belgeler/not.txt", [("Host", "kapi")])))
```

## 123-http-sunucu

- Video: `123-http-sunucu`
- Örnek yolu: `deneyler/mufredat-ornekleri/123-http-sunucu/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/kapi/` — kaynaklar: `Cargo.toml`, `src/ayristirici.rs`, `src/bin/kapi.rs`, `src/dosya.rs`, `src/havuz.rs`, `src/lib.rs`, `src/zaman_uyumsuz.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:06:29Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/kapi
k cargo build -q --release
./target/release/kapi havuz 127.0.0.1:8101 kok 8 2> /tmp/kapi.log &
SP=$!
sleep 0.5
ks 'curl -s -i http://127.0.0.1:8101/'
ks 'curl -s http://127.0.0.1:8101/belgeler/not.txt'
ks 'curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/yok.html'
# Dizin geçişi saldırısı: curl yolu "düzeltmesin" diye --path-as-is
ks 'curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/../../etc/passwd"'
ks 'curl -s -o /dev/null -w "%{http_code}\n" -X POST http://127.0.0.1:8101/'
# Bozuk istek: ham TCP
ks 'exec 3<>/dev/tcp/127.0.0.1/8101; printf "MERHABA\r\n\r\n" >&3; cat <&3'
kill $SP
k cat /tmp/kapi.log
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ curl -s -i http://127.0.0.1:8101/
HTTP/1.1 200 OK
Content-Length: 23
Content-Type: text/plain; charset=utf-8

<h1>Merhaba Kapı</h1>
$ curl -s http://127.0.0.1:8101/belgeler/not.txt
gizli olmayan not
$ curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/yok.html
404
$ curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/../../etc/passwd"
403
$ curl -s -o /dev/null -w "%{http_code}\n" -X POST http://127.0.0.1:8101/
405
$ exec 3<>/dev/tcp/127.0.0.1/8101; printf "MERHABA\r\n\r\n" >&3; cat <&3
HTTP/1.1 400 Bad Request
Content-Length: 28
Content-Type: text/plain; charset=utf-8

BozukIstekSatiri("MERHABA")
$ cat /tmp/kapi.log
havuz sunucusu 8 işçiyle dinliyor: 127.0.0.1:8101
```

## 124-http-eszamanlilik-ve-yuk

- Video: `124-http-eszamanlilik-ve-yuk`
- Örnek yolu: `deneyler/mufredat-ornekleri/124-http-eszamanlilik-ve-yuk/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/kapi/` — kaynaklar: `Cargo.toml`, `src/ayristirici.rs`, `src/bin/kapi.rs`, `src/dosya.rs`, `src/havuz.rs`, `src/lib.rs`, `src/zaman_uyumsuz.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:06:33Z
- Değişken çıktı: istek/s ve süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/kapi
k cargo build -q --release
./target/release/kapi havuz 127.0.0.1:8101 kok 8 2>/dev/null &
P1=$!
./target/release/kapi async 127.0.0.1:8102 kok 2>/dev/null &
P2=$!
sleep 0.5
echo "# 8 işçili havuz:"
k ./target/release/kapi yuk 127.0.0.1:8101 100 200
echo "# tokio (görev başına bağlantı):"
k ./target/release/kapi yuk 127.0.0.1:8102 100 200
echo "# 8 yavaş istemci (başlığı bitirmiyor) varken:"
k ./target/release/kapi yavas 127.0.0.1:8101 8
k ./target/release/kapi yavas 127.0.0.1:8102 8
kill $P1 $P2
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
# 8 işçili havuz:
$ ./target/release/kapi yuk 127.0.0.1:8101 100 200
100 bağlantı × 200 istek: 249533 istek/s; bağlantı süresi en kısa 6 ms, ortanca 46 ms, en uzun 77 ms
# tokio (görev başına bağlantı):
$ ./target/release/kapi yuk 127.0.0.1:8102 100 200
100 bağlantı × 200 istek: 300508 istek/s; bağlantı süresi en kısa 45 ms, ortanca 61 ms, en uzun 64 ms
# 8 yavaş istemci (başlığı bitirmiyor) varken:
$ ./target/release/kapi yavas 127.0.0.1:8101 8
8 yavaş bağlantı açıkken normal bir istek: 3061 ms → yanıt yok (Resource temporarily unavailable (os error 11))
$ ./target/release/kapi yavas 127.0.0.1:8102 8
8 yavaş bağlantı açıkken normal bir istek: 0 ms → HTTP/1.1 200 OK
```

## 125-yorumlayici-lexer

- Video: `125-yorumlayici-lexer`
- Örnek yolu: `deneyler/mufredat-ornekleri/125-yorumlayici-lexer/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/minik/` — kaynaklar: `Cargo.toml`, `src/ast.rs`, `src/bin/minik.rs`, `src/derleyici.rs`, `src/lexer.rs`, `src/lib.rs`, `src/parser.rs`, `src/vm.rs`, `src/yorumlayici.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:34Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/minik
k cargo build -q --release
k cat ornekler/ifade.mk
k ./target/release/minik token ornekler/ifade.mk
k ./target/release/minik token ornekler/garip_karakter.mk
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ cat ornekler/ifade.mk
let x = 2 + 3 * (4 - 1);
print(x);
$ ./target/release/minik token ornekler/ifade.mk
 1:1  Let
 1:5  Ad("x")
 1:7  Esit
 1:9  Sayi(2)
 1:11 Arti
 1:13 Sayi(3)
 1:15 Yildiz
 1:17 SolParantez
 1:18 Sayi(4)
 1:20 Eksi
 1:22 Sayi(1)
 1:23 SagParantez
 1:24 NoktaliVirgul
 2:1  Print
 2:6  SolParantez
 2:7  Ad("x")
 2:8  SagParantez
 2:9  NoktaliVirgul
 3:1  Son
$ ./target/release/minik token ornekler/garip_karakter.mk
hata: satır 1, sütun 11: beklenmeyen karakter '@'
[çıkış kodu: 1]
```

## 126-yorumlayici-parser

- Video: `126-yorumlayici-parser`
- Örnek yolu: `deneyler/mufredat-ornekleri/126-yorumlayici-parser/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/minik/` — kaynaklar: `Cargo.toml`, `src/ast.rs`, `src/bin/minik.rs`, `src/derleyici.rs`, `src/lexer.rs`, `src/lib.rs`, `src/parser.rs`, `src/vm.rs`, `src/yorumlayici.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:34Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/minik
k cargo build -q --release
# Öncelik ağacın biçiminde: 2 + (3 * (4 - 1))
k ./target/release/minik ast ornekler/ifade.mk
k cat ornekler/fib.mk
k ./target/release/minik ast ornekler/fib.mk
k ./target/release/minik ast ornekler/hatali.mk
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/minik ast ornekler/ifade.mk
(let x (+ 2 (* 3 (- 4 1))))
(print x)
$ cat ornekler/fib.mk
// Özyinelemeli Fibonacci
fn fib(n) {
    if n < 2 { return n; }
    return fib(n - 1) + fib(n - 2);
}

let i = 0;
while i <= 10 {
    print(fib(i));
    i = i + 1;
}
print(fib(20));
$ ./target/release/minik ast ornekler/fib.mk
(fn fib (n)
  (if (< n 2)
    (return n)
  )
  (return (+ (fib (- n 1)) (fib (- n 2))))
)
(let i 0)
(while (<= i 10)
  (print (fib i))
  (= i (+ i 1))
)
(print (fib 20))
$ ./target/release/minik ast ornekler/hatali.mk
hata: satır 1, sütun 13: bir ifade bekleniyordu, Yildiz bulundu
[çıkış kodu: 1]
```

## 127-yorumlayici-degerlendirici

- Video: `127-yorumlayici-degerlendirici`
- Örnek yolu: `deneyler/mufredat-ornekleri/127-yorumlayici-degerlendirici/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/minik/` — kaynaklar: `Cargo.toml`, `src/ast.rs`, `src/bin/minik.rs`, `src/derleyici.rs`, `src/lexer.rs`, `src/lib.rs`, `src/parser.rs`, `src/vm.rs`, `src/yorumlayici.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:34Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/minik
k cargo build -q --release
k ./target/release/minik calistir ornekler/fib.mk
k cat ornekler/sifira_bolme.mk
k ./target/release/minik calistir ornekler/sifira_bolme.mk
k ./target/release/minik calistir ornekler/sonsuz.mk
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/minik calistir ornekler/fib.mk
0
1
1
2
3
5
8
13
21
34
55
6765
$ cat ornekler/sifira_bolme.mk
fn ortalama(toplam, adet) {
    return toplam / adet;
}
print(ortalama(10, 2));
print(ortalama(10, 0));
$ ./target/release/minik calistir ornekler/sifira_bolme.mk
5
hata: çalışma hatası: sıfıra bölme
[çıkış kodu: 1]
$ ./target/release/minik calistir ornekler/sonsuz.mk
hata: çalışma hatası: çağrı derinliği sınırı aşıldı (2000)
[çıkış kodu: 1]
```

## 128-yorumlayici-bytecode-vm

- Video: `128-yorumlayici-bytecode-vm`
- Örnek yolu: `deneyler/mufredat-ornekleri/128-yorumlayici-bytecode-vm/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/minik/` — kaynaklar: `Cargo.toml`, `src/ast.rs`, `src/bin/minik.rs`, `src/derleyici.rs`, `src/lexer.rs`, `src/lib.rs`, `src/parser.rs`, `src/vm.rs`, `src/yorumlayici.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-20T16:06:34Z
- Değişken çıktı: süreler ve hızlanma oranı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler/minik
k cargo build -q --release
k cat ornekler/kare.mk
k ./target/release/minik listele ornekler/kare.mk
k ./target/release/minik vm ornekler/kare.mk
# İki motor aynı programda aynı çıktıyı veriyor mu?
ks 'diff <(./target/release/minik calistir ornekler/fib.mk) <(./target/release/minik vm ornekler/fib.mk) && echo "fib.mk: iki motorun çıktısı aynı"'
k cat ornekler/kiyas.mk
k ./target/release/minik kiyasla ornekler/kiyas.mk
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ cat ornekler/kare.mk
fn kare(x) {
    return x * x;
}
let t = 0;
let i = 1;
while i <= 3 && t < 100 {
    t = t + kare(i);
    i = i + 1;
}
print(t);
$ ./target/release/minik listele ornekler/kare.mk
kare (1 parametre, 1 yerel yuva):
  0000  YereliOku(0)
  0001  YereliOku(0)
  0002  Ikili(Carp)
  0003  Don
  0004  DonBos
<ana> (0 parametre, 2 yerel yuva):
  0000  Sabit(0)
  0001  YereleYaz(0)
  0002  Sabit(1)
  0003  YereleYaz(1)
  0004  YereliOku(1)
  0005  Sabit(3)
  0006  Ikili(KucukEsit)
  0007  VeAtla(11)
  0008  YereliOku(0)
  0009  Sabit(100)
  0010  Ikili(Kucuk)
  0011  YanlissaAtla(22)
  0012  YereliOku(0)
  0013  YereliOku(1)
  0014  Cagir(0, 1)
  0015  Ikili(Topla)
  0016  YereleYaz(0)
  0017  YereliOku(1)
  0018  Sabit(1)
  0019  Ikili(Topla)
  0020  YereleYaz(1)
  0021  Atla(4)
  0022  YereliOku(0)
  0023  Yazdir
  0024  DonBos
$ ./target/release/minik vm ornekler/kare.mk
14
$ diff <(./target/release/minik calistir ornekler/fib.mk) <(./target/release/minik vm ornekler/fib.mk) && echo "fib.mk: iki motorun çıktısı aynı"
fib.mk: iki motorun çıktısı aynı
$ cat ornekler/kiyas.mk
fn fib(n) {
    if n < 2 { return n; }
    return fib(n - 1) + fib(n - 2);
}
print(fib(27));
$ ./target/release/minik kiyasla ornekler/kiyas.mk
196418
196418
ağaç yürüten yorumlayıcı:    163.9 ms (5084966 ifade düğümü değerlendirildi)
bayt kod + yığın makinesi:     36.1 ms (6356210 komut yürütüldü)
hızlanma: 4.54×
```

## 129-yolun-sonu

- Video: `129-yolun-sonu`
- Örnek yolu: `deneyler/mufredat-ornekleri/129-yolun-sonu/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/ambar/` — kaynaklar: `Cargo.toml`, `src/bin/ambar.rs`, `src/crc.rs`, `src/gunluk.rs`, `src/lib.rs`, `src/motor.rs`, `src/protokol.rs`, `src/sunucu.rs`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/kapi/` — kaynaklar: `Cargo.toml`, `src/ayristirici.rs`, `src/bin/kapi.rs`, `src/dosya.rs`, `src/havuz.rs`, `src/lib.rs`, `src/zaman_uyumsuz.rs`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/minik/` — kaynaklar: `Cargo.toml`, `src/ast.rs`, `src/bin/minik.rs`, `src/derleyici.rs`, `src/lexer.rs`, `src/lib.rs`, `src/parser.rs`, `src/vm.rs`, `src/yorumlayici.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): 2026-11-20T16:06:36Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd ../ortak-projeler
for p in ambar kapi minik; do
  ks "cd $p && cargo test -q 2>&1 | grep 'test result' | head -n 1 | sed 's/^/$p: /'"
done
# Üç bitirme projesinin Rust satır sayısı
ks 'wc -l ambar/src/*.rs ambar/src/bin/*.rs kapi/src/*.rs kapi/src/bin/*.rs minik/src/*.rs minik/src/bin/*.rs | tail -n 1'
# Bu müfredat için konteynerde çalıştırılan deney sayısı
ks 'ls -d ../[0-9][0-9][0-9]-*/ | wc -l'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cd ambar && cargo test -q 2>&1 | grep 'test result' | head -n 1 | sed 's/^/ambar: /'
ambar: test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cd kapi && cargo test -q 2>&1 | grep 'test result' | head -n 1 | sed 's/^/kapi: /'
kapi: test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cd minik && cargo test -q 2>&1 | grep 'test result' | head -n 1 | sed 's/^/minik: /'
minik: test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ wc -l ambar/src/*.rs ambar/src/bin/*.rs kapi/src/*.rs kapi/src/bin/*.rs minik/src/*.rs minik/src/bin/*.rs | tail -n 1
 1711 total
$ ls -d ../[0-9][0-9][0-9]-*/ | wc -l
129
```

