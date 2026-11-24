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
imaj: localhost/nkl-rust:1.100.0  kimlik: 2fcffdebdebad090ec5450a47230d86cda084c707d95a1bea46897ff3d99f2fc
tarih: 2026-11-24T21:09:21Z
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
- [048-tek-bicimlestirme](#048-tek-bicimlestirme)
- [049-trait-nesneleri-ve-vtable](#049-trait-nesneleri-ve-vtable)
- [050-statik-ve-dinamik-yonlendirme](#050-statik-ve-dinamik-yonlendirme)
- [051-standart-traitler-ve-donusumler](#051-standart-traitler-ve-donusumler)
- [052-closurelar](#052-closurelar)
- [053-yineleyici-protokolu](#053-yineleyici-protokolu)
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
- Kaynaklar: `dogru-sira/Cargo.toml`, `dogru-sira/src/main.rs`, `klon/Cargo.toml`, `klon/src/main.rs`, `odunc/Cargo.toml`, `odunc/src/main.rs`, `sarkan/Cargo.toml`, `sarkan/src/main.rs`, `yeniden-kullanim/Cargo.toml`, `yeniden-kullanim/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: adresler her çalıştırmada değişir (ASLR); miri çıktısındaki alloc numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# 1) Güvenli Rust: iade edilen blok bir sonraki isteğe yeniden verilir (adresler yalnızca sayı olarak karşılaştırılıyor)
cd yeniden-kullanim
k cargo build -q
k ./target/debug/yeniden_kullanim
ks 'for i in $(seq 100); do ./target/debug/yeniden_kullanim | tail -n 1; done | sort | uniq -c'
cd ..
# 2) unsafe Rust: sarkan işaretçiyle okumak tanımsız davranıştır; sonuç derleme ayarına göre değişebilir
cd sarkan
k cargo build -q
k ./target/debug/sarkan
k cargo build -q --release
k ./target/release/sarkan
# 3) miri programı yorumlayarak çalıştırır ve tanımsız davranışı yakalar
k cargo +nightly miri run -q
cd ..
# 4) Güvenli Rust: aynı desen derlenmez (E0505)
cd odunc && k cargo build -q; cd ..
# 5) Kendin dene: ödüncün son kullanımı iadeden önceye alınınca program derlenir
cd dogru-sira && k cargo run -q; cd ..
# 6) Derleyicinin önerisi (help: clone) uygulanınca program derlenir; referans artık bir kopyaya bakar
cd klon && k cargo run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/yeniden_kullanim
isim  bloğu: 0x5643f6dc3d60
sifre bloğu: 0x5643f6dc3d60
aynı blok mu? evet
$ for i in $(seq 100); do ./target/debug/yeniden_kullanim | tail -n 1; done | sort | uniq -c
    100 aynı blok mu? evet
$ cargo build -q
$ ./target/debug/sarkan
eski adresteki 6 bayt: 67 69 7a 6c 69 2d  -> "gizli-"
$ cargo build -q --release
$ ./target/release/sarkan
eski adresteki 6 bayt: 00 00 00 00 00 00  -> "\0\0\0\0\0\0"
$ cargo +nightly miri run -q
error: Undefined Behavior: memory access failed: alloc330 has been freed, so this pointer is dangling
  --> src/main.rs:10:30
   |
10 |         okunan.push(unsafe { *p.add(i) }); // tanımsız davranış: iade edilmiş belleği okumak
   |                              ^^^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc330 was allocated here:
  --> src/main.rs:3:16
   |
 3 |     let isim = String::from("Ferris");
   |                ^^^^^^^^^^^^^^^^^^^^^^
help: alloc330 was deallocated here:
  --> src/main.rs:6:5
   |
 6 |     drop(isim); // blok iade edildi; p artık sarkan bir işaretçi
   |     ^^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error

[çıkış kodu: 1]
$ cargo build -q
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
error: could not compile `odunc` (bin "odunc") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q
isim = Ferris
$ cargo run -q
isim = Ferris
```

## 002-laboratuvar-kurulumu

- Video: `002-laboratuvar-kurulumu`
- Örnek yolu: `deneyler/mufredat-ornekleri/002-laboratuvar-kurulumu/`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: cargo'nun "Finished ... in X s" süresi; süreç kimlikleri (PID), konak adı (hostname) ve ad alanı kimlik numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Konteynerin içindeyiz: araç sürümleri ve çekirdek
k rustc --version
k cargo --version
k uname -r
ks 'grep PRETTY_NAME /etc/os-release'
# Yalıtım: bu sürecin ad alanları (namespace) ve konteynerin içinden görünen süreçler
k ls /proc/self/ns
k ps -e -o pid,comm
k hostname
# Yalıtımın kanıtı ad alanı kimliklerinde: ana makinede aynı komut başka numaralar verir
k readlink /proc/self/ns/pid /proc/self/ns/uts /proc/self/ns/net
# cgroup ad alanı: süreç kendi kontrol grubunu kök (/) olarak görür
k cat /proc/self/cgroup
# Kontrol grubunun bir kaynak sınırı: en fazla süreç sayısı
k cat /sys/fs/cgroup/pids.max
# Araç rafı: sürüm satırları
k cargo clippy --version
k rustfmt --version
k valgrind --version
ks 'strace -V | head -n 1'
ks 'gdb --version | head -n 1'
k hyperfine --version
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
$ ls /proc/self/ns
cgroup
ipc
mnt
net
pid
pid_for_children
time
time_for_children
user
uts
$ ps -e -o pid,comm
    PID COMMAND
      1 bash
    962 bash
    986 ps
$ hostname
c8a1a101fed7
$ readlink /proc/self/ns/pid /proc/self/ns/uts /proc/self/ns/net
pid:[4026534006]
uts:[4026534004]
net:[4026533945]
$ cat /proc/self/cgroup
0::/
$ cat /sys/fs/cgroup/pids.max
2048
$ cargo clippy --version
clippy 0.1.100 (0ef1bc3860 2026-11-09)
$ rustfmt --version
rustfmt 1.11.0-stable (0ef1bc3860 2026-11-09)
$ valgrind --version
valgrind-3.24.0
$ strace -V | head -n 1
strace -- version 6.13
$ gdb --version | head -n 1
GNU gdb (Debian 16.3-1) 16.3
$ hyperfine --version
hyperfine 1.19.0
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
- Kaynaklar: `Cargo.toml`, `iki-satir/Cargo.toml`, `iki-satir/src/main.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: strace -c süreleri ve satır sırası (tablo sürelere göre sıralanır; sayılar aynı kalır); ikili dosya boyutu derleyici sürümüne bağlı
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
# Kendin dene: iki println! kaç write çağrısı yapar?
cd iki-satir && k cargo build --release -q && ks 'strace -e trace=write ./target/release/iki_satir > /dev/null'; cd ..
# Çözülmüş sembol adı
ks 'nm -C target/release/merhaba | grep "merhaba::main"'
# Çağrıların kaçı yükleyiciye ve C kütüphanesine ait? Rust'ın kendi hazırlığı poll ile başlar
ks 'strace ./target/release/merhaba 2>&1 >/dev/null | grep -c ""'
ks 'strace ./target/release/merhaba 2>&1 >/dev/null | grep -n -m1 "^poll"'
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
  0.00    0.000000           0         5           read
  0.00    0.000000           0         1           write
  0.00    0.000000           0         4           close
  0.00    0.000000           0         4           fstat
  0.00    0.000000           0         1           poll
  0.00    0.000000           0        13           mmap
  0.00    0.000000           0         5           mprotect
  0.00    0.000000           0         1           munmap
  0.00    0.000000           0         3           brk
  0.00    0.000000           0         5           rt_sigaction
  0.00    0.000000           0         2           pread64
  0.00    0.000000           0         1         1 access
  0.00    0.000000           0         1           execve
  0.00    0.000000           0         2           sigaltstack
  0.00    0.000000           0         1           arch_prctl
  0.00    0.000000           0         1           gettid
  0.00    0.000000           0         1           sched_getaffinity
  0.00    0.000000           0         1           set_tid_address
  0.00    0.000000           0         4           openat
  0.00    0.000000           0         1           set_robust_list
  0.00    0.000000           0         2           prlimit64
  0.00    0.000000           0         1           getrandom
  0.00    0.000000           0         1           rseq
------ ----------- ----------- --------- --------- ------------------
100.00    0.000000           0        61         1 total
$ nm target/release/merhaba | grep -i "merhaba.*main"
0000000000014b30 t _RNvCsjEVuznjrSGG_7merhaba4main
$ cargo build --release -q
$ strace -e trace=write ./target/release/iki_satir > /dev/null
write(1, "Merhaba, d\303\274nya!\n", 17) = 17
write(1, "\304\260kinci sat\304\261r\n", 15) = 15
+++ exited with 0 +++
$ nm -C target/release/merhaba | grep "merhaba::main"
0000000000014b30 t merhaba::main
$ strace ./target/release/merhaba 2>&1 >/dev/null | grep -c ""
63
$ strace ./target/release/merhaba 2>&1 >/dev/null | grep -n -m1 "^poll"
39:poll([{fd=0, events=0}, {fd=1, events=0}, {fd=2, events=0}], 3, 0) = 0 (Timeout)
```

## 004-bit-ve-bayt

- Video: `004-bit-ve-bayt`
- Örnek yolu: `deneyler/mufredat-ornekleri/004-bit-ve-bayt/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): 2026-11-22T08:35:27Z
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
ks 'lscpu | grep "Byte Order"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
onluk         : 1000000
ikilik        : 00000000000011110100001001000000
onaltılık     : 0x000f4240
bellekte      : [40, 42, 0f, 00]
little-endian : [40, 42, 0f, 00]
big-endian    : [00, 0f, 42, 40]
'A' baytı     : 65 = 0x41 = 01000001
$ lscpu | grep "Byte Order"
Byte Order:                              Little Endian
```

## 005-tam-sayilar

- Video: `005-tam-sayilar`
- Örnek yolu: `deneyler/mufredat-ornekleri/005-tam-sayilar/`
- Kaynaklar: `Cargo.toml`, `sabit-tasma/Cargo.toml`, `sabit-tasma/src/main.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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

thread 'main' (1364) panicked at src/main.rs:12:41:
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
- Son çalıştırma (UTC): çalıştırılmadı
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
- Kaynaklar: `Cargo.toml`, `ajan-masasi/Cargo.toml`, `ajan-masasi/src/main.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# Ajan Masası (K1): para tutarını f64 ile tutan ajan kodu
cd ajan-masasi && k cargo run -q; cd ..
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
$ cargo run -q
toplam (f64)       = 0.30000000000000004
toplam == 0.30     → false
1.15 TL kaç kuruş? = 114
1.15 * 100.0       = 114.99999999999999
toplam (kuruş)     = 145 → 1.45 TL
```

## 008-islemci-ve-assembly

- Video: `008-islemci-ve-assembly`
- Örnek yolu: `deneyler/mufredat-ornekleri/008-islemci-ve-assembly/`
- Kaynaklar: `islem.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler donanıma ve o anki yüke bağlı (AMD Ryzen 5 5600: L1d 32 KiB, L2 512 KiB/çekirdek, L3 32 MiB)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
ks 'lscpu | grep -E "^L1d|^L2|^L3"'
k cargo run -q --release
# Kendin dene: aynı boyutlar, sıralı halka (alanda yerellik ve önceden getirme)
k cargo run -q --release -- sirali
```

- Gerçek çıktı (`cikti.txt`):

```text
$ lscpu | grep -E "^L1d|^L2|^L3"
L1d cache:                               192 KiB (6 instances)
L2 cache:                                3 MiB (6 instances)
L3 cache:                                32 MiB (1 instance)
$ cargo run -q --release
örnek halka (n = 8): 0 → 4 → 2 → 1 → 3 → 5 → 7 → 6 → 0
erişim düzeni: rastgele (Sattolo)
çalışma kümesi    ns/erişim
        16 KiB         1.14   (son=330)
        32 KiB         1.17   (son=2719)
        64 KiB         2.17   (son=7641)
       128 KiB         2.60   (son=15857)
       256 KiB         2.84   (son=6251)
       512 KiB         5.24   (son=61778)
      1024 KiB         8.90   (son=115268)
      2048 KiB        10.92   (son=43097)
      8192 KiB        12.62   (son=158047)
     16384 KiB        40.19   (son=1655160)
     32768 KiB        59.98   (son=1515190)
     65536 KiB        69.71   (son=2436746)
    262144 KiB        85.93   (son=3877667)
$ cargo run -q --release -- sirali
erişim düzeni: sıralı
çalışma kümesi    ns/erişim
        16 KiB         1.15   (son=1664)
      1024 KiB         1.14   (son=38528)
     32768 KiB         1.14   (son=1611392)
    262144 KiB         1.15   (son=10000000)
```

## 010-yigin-ve-obek

- Video: `010-yigin-ve-obek`
- Örnek yolu: `deneyler/mufredat-ornekleri/010-yigin-ve-obek/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: adresler her çalıştırmada değişir (ASLR); 100 çalıştırmadaki farklı yığın son hanesi sayısı da rastgeledir, öbekte hep 1
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/yigin_obek
# Kendin dene: ikinci çalıştırmada hangi haneler değişiyor?
k ./target/debug/yigin_obek
# 100 çalıştırmada son üç onaltılık hane kaç farklı değer alıyor? (yığın ve öbek)
ks 'for i in $(seq 100); do ./target/debug/yigin_obek | grep "^yığın"; done | sed -E "s/.*(...)$/\1/" | sort -u | wc -l'
ks 'for i in $(seq 100); do ./target/debug/yigin_obek | grep "Box<u64>"; done | sed -E "s/.*(...)$/\1/" | sort -u | wc -l'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/yigin_obek
kod    main()            : 0x55692815eb90
statik SELAM metni       : 0x55692814d9ec
öbek   Box<u64>          : 0x5569571a2d60
öbek   String verisi     : 0x5569571a2d80
yığın  yerel değişken    : 0x7ffe78beba58
556928146000-55692815e000 r--p 00000000 00:1d 80417821                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
55692815e000-5569281a1000 r-xp 00017000 00:1d 80417821                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
5569281a1000-5569281a5000 r--p 00059000 00:1d 80417821                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
5569281a5000-5569281a7000 rw-p 0005c000 00:1d 80417821                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
5569571a2000-5569571c3000 rw-p 00000000 00:00 0                          [heap]
7ffe78bcd000-7ffe78bee000 rw-p 00000000 00:00 0                          [stack]
$ ./target/debug/yigin_obek
kod    main()            : 0x55ce95459b90
statik SELAM metni       : 0x55ce954489ec
öbek   Box<u64>          : 0x55ceb5407d60
öbek   String verisi     : 0x55ceb5407d80
yığın  yerel değişken    : 0x7fffac128778
55ce95441000-55ce95459000 r--p 00000000 00:1d 80417821                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
55ce95459000-55ce9549c000 r-xp 00017000 00:1d 80417821                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
55ce9549c000-55ce954a0000 r--p 00059000 00:1d 80417821                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
55ce954a0000-55ce954a2000 rw-p 0005c000 00:1d 80417821                   /work/mufredat-ornekleri/010-yigin-ve-obek/target/debug/yigin_obek
55ceb5407000-55ceb5428000 rw-p 00000000 00:00 0                          [heap]
7fffac109000-7fffac12a000 rw-p 00000000 00:00 0                          [stack]
$ for i in $(seq 100); do ./target/debug/yigin_obek | grep "^yığın"; done | sed -E "s/.*(...)$/\1/" | sort -u | wc -l
83
$ for i in $(seq 100); do ./target/debug/yigin_obek | grep "Box<u64>"; done | sed -E "s/.*(...)$/\1/" | sort -u | wc -l
1
```

## 011-sanal-bellek

- Video: `011-sanal-bellek`
- Örnek yolu: `deneyler/mufredat-ornekleri/011-sanal-bellek/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
yığın adresi: 0x7fffbfb54037   öbek adresi: 0x561e32e95d60
başlangıç        : VmRSS      2092 kB  sayfa hatası     101
1 GiB ayrıldı    : VmRSS      2100 kB  sayfa hatası     102
sayfalara dokundu: VmRSS   1050672 kB  sayfa hatası  262245
AnonHugePages:         0 kB   (2 MiB'lik büyük sayfalarla karşılanan kısım)
toplam: 262144
$ ./target/release/sanal_bellek
yığın adresi: 0x7ffdb923eed7   öbek adresi: 0x55b5e2ca5d60
başlangıç        : VmRSS      2100 kB  sayfa hatası      98
1 GiB ayrıldı    : VmRSS      2108 kB  sayfa hatası      99
sayfalara dokundu: VmRSS   1050680 kB  sayfa hatası  262242
AnonHugePages:         0 kB   (2 MiB'lik büyük sayfalarla karşılanan kısım)
toplam: 262144
```

## 012-isletim-sistemi-ve-sistem-cagrilari

- Video: `012-isletim-sistemi-ve-sistem-cagrilari`
- Örnek yolu: `deneyler/mufredat-ornekleri/012-isletim-sistemi-ve-sistem-cagrilari/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
1000000 fonksiyon çağrısı :     1.22 ns/çağrı
1000000 getpid() çağrısı  :   275.41 ns/çağrı
$ strace -c -e trace=getpid ./target/release/sistem_cagrisi 1000 > /dev/null
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
100.00    0.001383           1      1000           getpid
------ ----------- ----------- --------- --------- ----------------
100.00    0.001383           1      1000           total
```

## 013-derleme-asamalari

- Video: `013-derleme-asamalari`
- Örnek yolu: `deneyler/mufredat-ornekleri/013-derleme-asamalari/`
- Kaynaklar: `Cargo.toml`, `kare.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
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
  Time (mean ± σ):     448.7 ms ±  23.4 ms    [User: 437.6 ms, System: 6.7 ms]
  Range (min … max):   424.3 ms … 481.3 ms    5 runs
 
Benchmark 2: ./target/release/elek
  Time (mean ± σ):      43.8 ms ±   3.0 ms    [User: 35.4 ms, System: 7.7 ms]
  Range (min … max):    40.7 ms …  48.6 ms    5 runs
 
Summary
  ./target/release/elek ran
   10.25 ± 0.88 times faster than ./target/debug/elek
```

## 014-baglama-ve-elf

- Video: `014-baglama-ve-elf`
- Örnek yolu: `deneyler/mufredat-ornekleri/014-baglama-ve-elf/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
# Yığın geri sarma (unwinding) için gereken işlevler libgcc_s kütüphanesinden gelir
ks 'nm -D target/release/merhaba | grep _Unwind | head -n 3'
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
	linux-vdso.so.1 (0x00007f6935753000)
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f69356c2000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f69354ce000)
	/lib64/ld-linux-x86-64.so.2 (0x00007f6935755000)
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
$ nm -D target/release/merhaba | grep _Unwind | head -n 3
                 U _Unwind_Backtrace@GCC_3.3
                 U _Unwind_DeleteException@GCC_3.0
                 U _Unwind_GetDataRelBase@GCC_3.0
$ RUSTFLAGS="-C target-feature=+crt-static" cargo build -q --release --target-dir target/statik
$ file target/statik/release/merhaba
target/statik/release/merhaba: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked, for GNU/Linux 3.2.0, BuildID[sha1]=acdf44f769388fce664765f20f0a1eac21c28cc4, not stripped
$ ldd target/statik/release/merhaba
	statically linked
$ stat -c "%s bayt  %n" target/release/merhaba target/statik/release/merhaba
445024 bayt  target/release/merhaba
1380592 bayt  target/statik/release/merhaba
$ ./target/statik/release/merhaba
Merhaba, dünya!
```

## 015-derleyici-hatalarini-okumak

- Video: `015-derleyici-hatalarini-okumak`
- Örnek yolu: `deneyler/mufredat-ornekleri/015-derleyici-hatalarini-okumak/`
- Kaynaklar: `mut-eksik/Cargo.toml`, `mut-eksik/src/main.rs`, `tip-hatasi/Cargo.toml`, `tip-hatasi/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
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
(u8, u32) alanları: .0@0 .1@4

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
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# Derleyicinin gördüğü kontrol akışı: adimlar fonksiyonunun MIR'ındaki temel bloklar ve atlamalar
mkdir -p target
k rustc --edition 2024 -C opt-level=2 --emit=mir -o target/collatz.mir src/main.rs
ks 'awk "/^fn adimlar/,/^}/" target/collatz.mir | grep -E "^    bb[0-9]+|switchInt|goto|return"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
n =   6:   8 adım, en yüksek değer 16
n =   7:  16 adım, en yüksek değer 52
n =  27: 111 adım, en yüksek değer 9232
n =  97: 118 adım, en yüksek değer 9232
1..1_000_000 arasında en uzun: n = 837799, 524 adım
$ rustc --edition 2024 -C opt-level=2 --emit=mir -o target/collatz.mir src/main.rs
$ awk "/^fn adimlar/,/^}/" target/collatz.mir | grep -E "^    bb[0-9]+|switchInt|goto|return"
    bb0: {
        goto -> bb1;
    bb1: {
        switchInt(copy _4) -> [1: bb6, otherwise: bb2];
    bb2: {
        switchInt(copy _6) -> [0: bb3, otherwise: bb4];
    bb3: {
        goto -> bb5;
    bb4: {
        goto -> bb5;
    bb5: {
        _9 = integer_max::<u64>(move _3, move _10) -> [return: bb7, unwind unreachable];
    bb6: {
        return;
    bb7: {
        goto -> bb1;
```

## 020-fonksiyonlar-ve-cagri-yigini

- Video: `020-fonksiyonlar-ve-cagri-yigini`
- Örnek yolu: `deneyler/mufredat-ornekleri/020-fonksiyonlar-ve-cagri-yigini/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: adresler (ASLR); çerçeve boyutu derleme ayarına bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
k cargo run -q --release
# Dönüş adresleri: gdb seviye 3'te durur; çağrı zinciri ve kaydedilmiş dönüş adresi (gdb ASLR'yi kapatır, adresler sabit görünür)
ks 'gdb -q -batch -ex "break cagri_yigini::asagi if seviye == 3" -ex run -ex bt -ex "info frame" ./target/debug/cagri_yigini 2>&1 | grep -E "^#|saved rip|called by"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
seviye 0: yerel @ 0x7ffc3639c9c4
seviye 1: yerel @ 0x7ffc3639c8b4  (272 bayt aşağıda)
seviye 2: yerel @ 0x7ffc3639c7a4  (272 bayt aşağıda)
seviye 3: yerel @ 0x7ffc3639c694  (272 bayt aşağıda)
seviye 4: yerel @ 0x7ffc3639c584  (272 bayt aşağıda)
$ cargo run -q --release
seviye 0: yerel @ 0x7ffcbd34438c
seviye 1: yerel @ 0x7ffcbd34432c  (96 bayt aşağıda)
seviye 2: yerel @ 0x7ffcbd3442cc  (96 bayt aşağıda)
seviye 3: yerel @ 0x7ffcbd34426c  (96 bayt aşağıda)
seviye 4: yerel @ 0x7ffcbd34420c  (96 bayt aşağıda)
$ gdb -q -batch -ex "break cagri_yigini::asagi if seviye == 3" -ex run -ex bt -ex "info frame" ./target/debug/cagri_yigini 2>&1 | grep -E "^#|saved rip|called by"
#0  cagri_yigini::asagi (seviye=3, onceki=140737488348484) at src/main.rs:3
#1  0x000055555556917d in cagri_yigini::asagi (seviye=2, onceki=140737488348756) at src/main.rs:11
#2  0x000055555556917d in cagri_yigini::asagi (seviye=1, onceki=140737488349028) at src/main.rs:11
#3  0x000055555556917d in cagri_yigini::asagi (seviye=0, onceki=0) at src/main.rs:11
#4  0x0000555555568f7c in cagri_yigini::main () at src/main.rs:16
 rip = 0x555555568f98 in cagri_yigini::asagi (src/main.rs:3); saved rip = 0x55555556917d
 called by frame at 0x7fffffffe630
```

## 021-ozyineleme

- Video: `021-ozyineleme`
- Örnek yolu: `deneyler/mufredat-ornekleri/021-ozyineleme/`
- Kaynaklar: `Cargo.toml`, `kuyruk-cagrisi/Cargo.toml`, `kuyruk-cagrisi/src/main.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: taşma mesajındaki iş parçacığı kimliği (thread 'main' (N)); Linux'ta ana iş parçacığının kimliği süreç kimliğidir
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
fib(5) =      5  saf:        15 çağrı   memoization ile:   9 çağrı  (aynı mı: true)
fib(10) =     55  saf:       177 çağrı   memoization ile:  19 çağrı  (aynı mı: true)
fib(20) =   6765  saf:     21891 çağrı   memoization ile:  39 çağrı  (aynı mı: true)
fib(30) = 832040  saf:   2692537 çağrı   memoization ile:  59 çağrı  (aynı mı: true)
$ ulimit -s
8192
$ ./target/debug/ozyineleme derin 1000
derin(1000) = 124948
$ ./target/debug/ozyineleme derin 100000

thread 'main' (3334) has overflowed its stack
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
- Kaynaklar: `asan/Cargo.toml`, `asan/src/main.rs`, `cifte-iade/Cargo.toml`, `cifte-iade/src/main.rs`, `cifte-iade-guvenli/Cargo.toml`, `cifte-iade-guvenli/src/main.rs`, `sinir/Cargo.toml`, `sinir/src/main.rs`, `sinir-disi/Cargo.toml`, `sinir-disi/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: AddressSanitizer adresleri ve süreç kimlikleri; panik satırındaki iş parçacığı kimliği; miri alloc numaraları; ayırma dışı okumada kullanılmayan baytların içeriği (bu kayıtta sıfır)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# 1) Sınır dışı erişim. Güvenli Rust: her indeks denetlenir, dışarısı tanımlı bir paniktir
cd sinir && k cargo run -q; cd ..
# unsafe Rust: 8 baytlık bir öbek bloğunun ötesini okumak (tanımsız davranış); makinede komşu blok görünür
cd sinir-disi
k cargo run -q
# miri iki deneysel ödünç modelinde de aynı hatayı verir: erişim ayırmanın sonunun ötesinde
k cargo +nightly miri run -q
ks 'MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri run -q'
cd ..
# 2) Çifte iade. unsafe Rust: glibc'nin ayırıcısı bu durumda ikinci iadeyi fark eder ve süreci durdurur
cd cifte-iade
k cargo run -q
k cargo +nightly miri run -q
cd ..
# Güvenli Rust: değer taşındıktan sonra bir daha kullanılamaz (E0382)
cd cifte-iade-guvenli && k cargo build -q; cd ..
# 3) İade sonrası kullanım ve gölge bellek: AddressSanitizer (nightly)
cd asan
k cargo run -q
ks 'ASAN_SYMBOLIZER_PATH=/usr/bin/addr2line RUSTFLAGS="-Zsanitizer=address" cargo +nightly run -q --target x86_64-unknown-linux-gnu 2>&1'
cd ..
```

- Gerçek çıktı (kısaltılmış; 179 satırın tamamı `cikti.txt` içinde):

```text
$ cargo run -q

thread 'main' (3375) panicked at src/main.rs:4:34:
index out of bounds: the len is 8 but the index is 20
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
$ cargo run -q
+0 : 46 65 72 72 69 73 00 00
+8 : 00 00 00 00 00 00 00 00
+16: 00 00 00 00 00 00 00 00
+24: 21 00 00 00 00 00 00 00
+32: 67 69 7a 6c 69 31 32 33
$ cargo +nightly miri run -q
+0 : 46 65 72 72 69 73 00 00
error: Undefined Behavior: memory access failed: attempting to access 1 byte, but got alloc313+0x8 which is at or beyond the end of the allocation of size 8 bytes
 --> src/main.rs:9:33
  |
9 |             let bayt = unsafe { *p.add(satir * 8 + i) }; // 8 ve ötesi: ayırmanın dışı
  |                                 ^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc313 was allocated here:
 --> src/main.rs:3:28
  |
3 |     let ad: Box<[u8; 8]> = Box::new(*b"Ferris\0\0");
  |                            ^^^^^^^^^^^^^^^^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error

[çıkış kodu: 1]
$ MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri run -q
+0 : 46 65 72 72 69 73 00 00
error: Undefined Behavior: memory access failed: attempting to access 1 byte, but got alloc313+0x8 which is at or beyond the end of the allocation of size 8 bytes
 --> src/main.rs:9:33
  |
9 |             let bayt = unsafe { *p.add(satir * 8 + i) }; // 8 ve ötesi: ayırmanın dışı
  |                                 ^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc313 was allocated here:
 --> src/main.rs:3:28
… (11 satır atlandı)
free(): double free detected in tcache 2
[çıkış kodu: 134 = 128 + 6 (SIGABRT)]
… (2 satır atlandı)
error: Undefined Behavior: constructing invalid value of type std::boxed::Box<u64>: encountered a dangling box (use-after-free)
… (21 satır atlandı)
error[E0382]: use of moved value: `kutu`
… (3 satır atlandı)
  |         ---- move occurs because `kutu` has type `Box<u64>`, which does not implement the `Copy` trait
… (1 satır atlandı)
  |          ---- value moved here
… (1 satır atlandı)
  |          ^^^^ value used here after move
… (10 satır atlandı)
ilk bayt: 0x67
… (2 satır atlandı)
==3776==ERROR: AddressSanitizer: heap-use-after-free on address 0x7b5b6a7e0010 at pc 0x55c523c08d61 bp 0x7fff53250e30 sp 0x7fff53250e28
READ of size 1 at 0x7b5b6a7e0010 thread T0
    #0 0x55c523c08d60 in asan::main /work/mufredat-ornekleri/022-bellek-hatalarinin-anatomisi/asan/src/main.rs:8
… (18 satır atlandı)
0x7b5b6a7e0010 is located 0 bytes inside of 6-byte region [0x7b5b6a7e0010,0x7b5b6a7e0016)
freed by thread T0 here:
… (6 satır atlandı)
previously allocated by thread T0 here:
… (6 satır atlandı)
    #6 0x55c523c08c6a in asan::main /work/mufredat-ornekleri/022-bellek-hatalarinin-anatomisi/asan/src/main.rs:4
… (4 satır atlandı)
SUMMARY: AddressSanitizer: heap-use-after-free /work/mufredat-ornekleri/022-bellek-hatalarinin-anatomisi/asan/src/main.rs:8 in asan::main
… (6 satır atlandı)
=>0x7b5b6a7e0000: fa fa[fd]fa fa fa 00 01 fa fa fa fa fa fa fa fa
… (5 satır atlandı)
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07 
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==3776==ABORTING
[çıkış kodu: 1]
```

## 023-bellek-yonetimi-stratejileri

- Video: `023-bellek-yonetimi-stratejileri`
- Örnek yolu: `deneyler/mufredat-ornekleri/023-bellek-yonetimi-stratejileri/`
- Kaynaklar: `elle/Cargo.toml`, `elle/src/main.rs`, `isaretle-supur/Cargo.toml`, `isaretle-supur/src/main.rs`, `raii/Cargo.toml`, `raii/src/main.rs`, `sayac/Cargo.toml`, `sayac/src/main.rs`, `zayif/Cargo.toml`, `zayif/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: miri çıktısındaki alloc numaraları; elle programının yazdığı adresler (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# 1) Elle yönetim: unsafe std::alloc; iade etmeyi unutmak sessizdir, miri sızıntıyı bildirir
cd elle
k cargo run -q
k cargo +nightly miri run -q
cd ..
# 2) Referans sayımı (Rc): sayaç sıfıra inince hemen bırakma; çevrim ise hiç bırakılmaz
cd sayac
k cargo run -q
k cargo +nightly miri run -q
cd ..
# Kendin dene: çevrimin bir yönü Weak olunca ikisi de bırakılır
cd zayif && k cargo run -q; cd ..
# 3) İzleyen toplama (işaretle ve süpür), küçük bir benzetim: köklerden erişilemeyen çevrim de toplanır
cd isaretle-supur && k cargo run -q; cd ..
# 4) Sahiplik ve RAII: kapsam bitince deterministik bırakma, oluşturma sırasının tersiyle
cd raii && k cargo run -q; cd ..
```

- Gerçek çıktı (kısaltılmış; 102 satırın tamamı `cikti.txt` içinde):

```text
$ cargo run -q
istenen: 32 bayt, hizalama 8
blok 0x55ad9b885ae0: [0, 10, 20, 30]
ikinci blok 0x55ad9b885ae0 ayrıldı
main bitti; ikinci blok hiç iade edilmedi
$ cargo +nightly miri run -q
istenen: 32 bayt, hizalama 8
blok 0x273b8: [0, 10, 20, 30]
ikinci blok 0x330f8 ayrıldı
main bitti; ikinci blok hiç iade edilmedi
error: memory leaked: alloc3654 (Rust heap, size: 32, align: 8), allocated here:
  --> src/main.rs:17:17
   |
17 |         let q = alloc(duzen); // ikinci blok: iade etmeyi "unutuyoruz"
   |                 ^^^^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

note: set `MIRIFLAGS=-Zmiri-ignore-leaks` to disable this check

error: aborting due to 1 previous error

[çıkış kodu: 1]
$ cargo run -q
a'nın sahip sayısı: 2
a'nın sahip sayısı: 1
  a bırakıldı
--- çevrim: x -> y -> x
x: 2, y: 2
drop sonrası x: 1, y: 1
main bitiyor
$ cargo +nightly miri run -q
a'nın sahip sayısı: 2
a'nın sahip sayısı: 1
  a bırakıldı
--- çevrim: x -> y -> x
x: 2, y: 2
drop sonrası x: 1, y: 1
main bitiyor
error: memory leaked: alloc2391 (Rust heap, size: 40, align: 8), allocated here:
   --> /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rcs/rc.rs:438:49
    |
438 |               Self::from_inner(Box::into_non_null(Box::new(RcInner {
    |  _________________________________________________^
439 | |                 strong: Cell::new(1),
… (13 satır atlandı)
error: memory leaked: alloc2452 (Rust heap, size: 40, align: 8), allocated here:
… (23 satır atlandı)

[çıkış kodu: 1]
$ cargo run -q
x: 1 güçlü, 1 zayıf; y: 2 güçlü
  x bırakıldı
  y bırakıldı
main bitiyor
$ cargo run -q
toplama başlıyor (kök: A)
  süpürüldü: D
  süpürüldü: E
  süpürüldü: F
kalanlar: ['A', 'B', 'C']
$ cargo run -q
iç kapsam bitiyor
  C bırakıldı
  B bırakıldı
  D bırakıldı
main bitiyor
  A bırakıldı
```

## 024-sahiplik-kurallari

- Video: `024-sahiplik-kurallari`
- Örnek yolu: `deneyler/mufredat-ornekleri/024-sahiplik-kurallari/`
- Kaynaklar: `tasima/Cargo.toml`, `tasima/src/main.rs`, `tasima-sonrasi/Cargo.toml`, `tasima-sonrasi/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: adresler (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd tasima && k cargo run -q; cd ..
cd tasima-sonrasi && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
s1: yığındaki yapı 0x7ffee0366da8, öbekteki veri 0x559880e3dd60, uzunluk 4, kapasite 4
s2: yığındaki yapı 0x7ffee0366e80, öbekteki veri 0x559880e3dd60, uzunluk 4, kapasite 4
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
- Son çalıştırma (UTC): çalıştırılmadı
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
s verisi @ 0x55fd0a8b2d60, t verisi @ 0x55fd0a8b2d80
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
- Kaynaklar: `kural/Cargo.toml`, `kural/src/main.rs`, `neden/Cargo.toml`, `neden/src/main.rs`, `sarkan-isaretci/Cargo.toml`, `sarkan-isaretci/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: adresler her çalıştırmada değişir (ASLR); bloğun yerinde büyüyüp büyümediği ayırıcıya bağlıdır; miri alloc numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Makine düzeyi: Vec dolunca ayırıcı bloğu bazen yerinde büyütür, bazen taşır
cd neden && k cargo run -q; cd ..
# unsafe Rust: eski adresi tutan ham işaretçi; bu çalıştırmada "çalışıyor" görünebilir, miri kuralın çiğnendiğini gösterir
cd sarkan-isaretci
k cargo run -q
k cargo +nightly miri run -q
cd ..
# Güvenli Rust: bir elemana referans yaşarken push reddedilir (E0502)
cd kural && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
kapasite 3, veri @ 0x55f85c7dad60
kapasite 6, veri @ 0x55f85c7dad60
kapasite 12, veri @ 0x55f85c7dada0
$ cargo run -q
önce : kapasite 3, veri @ 0x55d82391ad60
sonra: kapasite 6, veri @ 0x55d82391ad60
ilk = 1
$ cargo +nightly miri run -q
önce : kapasite 3, veri @ 0x21330
sonra: kapasite 6, veri @ 0x29e08
error: Undefined Behavior: memory access failed: alloc311 has been freed, so this pointer is dangling
 --> src/main.rs:8:26
  |
8 |     let deger = unsafe { *ilk }; // blok taşındıysa iade edilmiş belleği okur: tanımsız davranış
  |                          ^^^^ Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc311 was allocated here:
 --> src/main.rs:3:17
  |
3 |     let mut v = vec![1, 2, 3];
  |                 ^^^^^^^^^^^^^
help: alloc311 was deallocated here:
 --> src/main.rs:6:5
  |
6 |     v.push(4); // kapasite doldu: ayırıcı bloğu büyütür ya da taşır
  |     ^^^^^^^^^

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error

[çıkış kodu: 1]
$ cargo build -q
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let ilk = &v[0]; // paylaşılan ödünç
  |                - immutable borrow occurs here
4 |     v.push(4); // değiştirmek için değiştirilebilir (&mut) ödünç gerekir
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
- Son çalıştırma (UTC): çalıştırılmadı
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

thread 'main' (4696) panicked at src/main.rs:14:37:
end byte index 2 is not a char boundary; it is inside 'ğ' (bytes 1..3 of string)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
```

## 028-yasam-sureleri

- Video: `028-yasam-sureleri`
- Örnek yolu: `deneyler/mufredat-ornekleri/028-yasam-sureleri/`
- Kaynaklar: `duzeltilmis/Cargo.toml`, `duzeltilmis/src/main.rs`, `eksik-yasam-suresi/Cargo.toml`, `eksik-yasam-suresi/src/main.rs`, `sarkan/Cargo.toml`, `sarkan/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd sarkan && k cargo build -q; cd ..
cd eksik-yasam-suresi && k cargo build -q; cd ..
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
error: could not compile `eksik_yasam_suresi` (bin "eksik_yasam_suresi") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q
daha uzun: sahiplik
```

## 029-odunc-denetleyicinin-zihni

- Video: `029-odunc-denetleyicinin-zihni`
- Örnek yolu: `deneyler/mufredat-ornekleri/029-odunc-denetleyicinin-zihni/`
- Kaynaklar: `ajan-masasi/ilk-surum/Cargo.toml`, `ajan-masasi/ilk-surum/src/main.rs`, `ajan-masasi/kod/Cargo.toml`, `ajan-masasi/kod/src/bin/ajan.rs`, `ajan-masasi/kod/src/bin/duzeltilmis.rs`, `nll/Cargo.toml`, `nll/src/main.rs`, `temkinli/Cargo.toml`, `temkinli/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: valgrind satırlarındaki süreç kimliği (==N==)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd nll && k cargo run -q; cd ..
cd temkinli && k cargo build -q
# Yeni nesil denetleyici Polonius (yalnızca nightly'de, deneysel bayrakla)
ks 'rustc +nightly -Z help | grep -i polonius'
ks 'RUSTFLAGS="-Zpolonius=next" cargo +nightly run -q --target-dir target/polonius'
cd ..
# Ajan Masası (K6): ajanın ilk sürümü derlenmez; ajan her yere .clone() ekler; valgrind ayırma sayısını gösterir
cd ajan-masasi/ilk-surum && k cargo build -q; cd ../..
cd ajan-masasi/kod && k cargo build -q --release
ks 'valgrind ./target/release/ajan 2>&1 | grep -E "toplam uzunluk|total heap usage"'
ks 'valgrind ./target/release/duzeltilmis 2>&1 | grep -E "toplam uzunluk|total heap usage"'
cd ../..
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
$ cargo build -q
error[E0382]: use of moved value: `kelimeler`
  --> src/main.rs:19:27
   |
13 |     let mut kelimeler = Vec::new();
   |         ------------- move occurs because `kelimeler` has type `Vec<String>`, which does not implement the `Copy` trait
...
18 |     for _ in 0..100 {
   |     --------------- inside of this loop
19 |         toplam += en_uzun(kelimeler).len(); // ilk turda taşınır
   |                           ^^^^^^^^^ value moved here, in previous iteration of loop
   |
note: consider changing this parameter type in function `en_uzun` to borrow instead if owning the value isn't necessary
  --> src/main.rs:2:23
   |
 2 | fn en_uzun(kelimeler: Vec<String>) -> String {
   |    -------            ^^^^^^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
help: consider cloning the value if the performance cost is acceptable
   |
19 |         toplam += en_uzun(kelimeler.clone()).len(); // ilk turda taşınır
   |                                    ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ilk_surum` (bin "ilk_surum") due to 1 previous error
[çıkış kodu: 101]
$ cargo build -q --release
$ valgrind ./target/release/ajan 2>&1 | grep -E "toplam uzunluk|total heap usage"
toplam uzunluk: 900
==5189==   total heap usage: 101,119 allocs, 101,117 frees, 3,353,700 bytes allocated
$ valgrind ./target/release/duzeltilmis 2>&1 | grep -E "toplam uzunluk|total heap usage"
toplam uzunluk: 900
==5192==   total heap usage: 1,019 allocs, 1,017 frees, 64,700 bytes allocated
```

## 030-structlar-ve-bellek-yerlesimi

- Video: `030-structlar-ve-bellek-yerlesimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/030-structlar-ve-bellek-yerlesimi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
repr(Rust): boyut  8  a@6 b@0 c@4
repr(C)   : boyut 12  a@0 b@4 c@8
repr(C) büyükten küçüğe: boyut  8  a@6 b@0 c@4
```

## 031-metotlar-ve-impl

- Video: `031-metotlar-ve-impl`
- Örnek yolu: `deneyler/mufredat-ornekleri/031-metotlar-ve-impl/`
- Kaynaklar: `hesap/Cargo.toml`, `hesap/src/main.rs`, `kapali-hesap/Cargo.toml`, `kapali-hesap/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd eksik-kol && k cargo build -q; cd ..
cd dilim-oruntuleri && k cargo run -q; cd ..
# Karar ağacı: derleyici dilim örüntülerini uzunluk üzerinde dallanmaya çevirir (MIR'daki switchInt)
mkdir -p target
k rustc --edition 2024 -C opt-level=0 --emit=mir -o target/dilim.mir dilim-oruntuleri/src/main.rs
ks 'awk "/^fn siniflandir/,/^}/" target/dilim.mir | grep -E "switchInt|Ge\(|Eq\(|PtrMetadata|Len\("'
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
$ rustc --edition 2024 -C opt-level=0 --emit=mir -o target/dilim.mir dilim-oruntuleri/src/main.rs
$ awk "/^fn siniflandir/,/^}/" target/dilim.mir | grep -E "switchInt|Ge\(|Eq\(|PtrMetadata|Len\("
        _6 = PtrMetadata(copy _1);
        _7 = Eq(copy _6, const 0_usize);
        switchInt(move _7) -> [0: bb1, otherwise: bb6];
        _4 = PtrMetadata(copy _1);
        _5 = Eq(copy _4, const 1_usize);
        switchInt(move _5) -> [0: bb3, otherwise: bb5];
        _2 = PtrMetadata(copy _1);
        _3 = Ge(copy _2, const 2_usize);
        switchInt(move _3) -> [0: bb2, otherwise: bb4];
        switchInt(move _22) -> [0: bb12, otherwise: bb11];
        _38 = PtrMetadata(copy _32);
```

## 035-tiplerin-matematigi

- Video: `035-tiplerin-matematigi`
- Örnek yolu: `deneyler/mufredat-ornekleri/035-tiplerin-matematigi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreç kimliği (PID) ve panik satırındaki iş parçacığı kimliği (ana iş parçacığında aynı sayı); geri izdeki adres/çerçeve sayısı
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
süreç kimliği (PID): 5672
10 / 2 = 5
$ cargo run -q -- 0
süreç kimliği (PID): 5734

thread 'main' (5734) panicked at src/main.rs:3:9:
sıfıra bölme: 10 / 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
$ RUST_BACKTRACE=1 cargo run -q -- 0 2>&1 | grep -E -A1 "panicked at|panik::"
thread 'main' (5746) panicked at src/main.rs:3:9:
sıfıra bölme: 10 / 0
--
   2: panik::bol
             at ./src/main.rs:3:9
   3: panik::main
             at ./src/main.rs:11:31
$ cargo run -q --release -- 0
süreç kimliği (PID): 5758

thread 'main' (5758) panicked at src/main.rs:3:9:
sıfıra bölme: 10 / 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 134 = 128 + 6 (SIGABRT)]
```

## 037-result-ve-soru-isareti

- Video: `037-result-ve-soru-isareti`
- Örnek yolu: `deneyler/mufredat-ornekleri/037-result-ve-soru-isareti/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
- Kaynaklar: `Cargo.toml`, `ajan-masasi/Cargo.toml`, `ajan-masasi/src/bin/ajan.rs`, `ajan-masasi/src/bin/duzeltilmis.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `anyhow 1.0.104`, `proc-macro2 1.0.107`, `quote 1.0.47`, `syn 3.0.10`, `thiserror 2.0.21`, `thiserror-impl 2.0.21`, `unicode-ident 1.0.27`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: panik satırındaki iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/ayar ayarlar/iyi.conf
k ./target/debug/ayar ayarlar/bozuk.conf
k ./target/debug/ayar ayarlar/yok.conf
# Ajan Masası (K8): unwrap() ile ayrıştıran ajan kodu; bozuk girdi ve u16 taşması panik verir
cd ajan-masasi && k cargo build -q --release
k ./target/release/ajan 8080
k ./target/release/ajan abc
k ./target/release/ajan 70000
# Düzeltilmiş sürüm: aynı girdiler, panik yerine bağlamlı hata ve çıkış kodu 1
k ./target/release/duzeltilmis abc
k ./target/release/duzeltilmis 70000
cd ..
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
$ cargo build -q --release
$ ./target/release/ajan 8080
sunucu 8080 numaralı portta başlıyor
$ ./target/release/ajan abc

thread 'main' (6323) panicked at src/bin/ajan.rs:4:42:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
$ ./target/release/ajan 70000

thread 'main' (6324) panicked at src/bin/ajan.rs:4:42:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: PosOverflow }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
$ ./target/release/duzeltilmis abc
Error: port numarası okunamadı: "abc"

Caused by:
    invalid digit found in string
[çıkış kodu: 1]
$ ./target/release/duzeltilmis 70000
Error: port numarası okunamadı: "70000"

Caused by:
    number too large to fit in target type
[çıkış kodu: 1]
```

## 039-karmasiklik-analizi

- Video: `039-karmasiklik-analizi`
- Örnek yolu: `deneyler/mufredat-ornekleri/039-karmasiklik-analizi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# Kaynak (nightly rust-src): std'nin ikili araması eşitlikte erken çıkmaz; döngü sayısı yalnız uzunluğa bağlıdır
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "intentionally doesn.t have an early exit\|on the size of the input slice" $S/core/src/slice/mod.rs | head -2'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
n =        1000, son eleman: doğrusal        1000 karşılaştırma | ikili 11 (log2 n = 10.0)
n =        1000, ilk eleman: doğrusal           1 karşılaştırma | ikili 11 (log2 n = 10.0)
n =     1000000, son eleman: doğrusal     1000000 karşılaştırma | ikili 21 (log2 n = 19.9)
n =     1000000, ilk eleman: doğrusal           1 karşılaştırma | ikili 21 (log2 n = 19.9)
n =   100000000, son eleman: doğrusal   100000000 karşılaştırma | ikili 28 (log2 n = 26.6)
n =   100000000, ilk eleman: doğrusal           1 karşılaştırma | ikili 28 (log2 n = 26.6)
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "intentionally doesn.t have an early exit\|on the size of the input slice" $S/core/src/slice/mod.rs | head -2
2990:        // This loop intentionally doesn't have an early exit if the comparison
2992:        // on the size of the input slice so that the CPU can reliably predict
```

## 040-vec-dinamik-dizi

- Video: `040-vec-dinamik-dizi`
- Örnek yolu: `deneyler/mufredat-ornekleri/040-vec-dinamik-dizi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: "(taşındı)" / "(yerinde büyüdü)" işaretleri ve "gerçekte taşınan" sayısı ayırıcıya ve süreçteki öteki ayırmalara bağlıdır
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# Kaynak (nightly rust-src): büyütme kapasiteyi ikiye katlar; ilk ayırmanın en küçük kapasitesi eleman boyutuna bağlı
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "let cap = cmp::max(self.cap.as_inner() \* 2, required_cap)" $S/alloc/src/raw_vec/mod.rs; grep -n -A8 "^const fn min_non_zero_cap" $S/alloc/src/raw_vec/mod.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
size_of::<Vec<u64>>() = 24 bayt: işaretçi + kapasite + uzunluk
başlangıç: len 0, cap 0
len    1 → cap    4  (ilk ayırma)
len    5 → cap    8  (taşındı)
len    9 → cap   16  (yerinde büyüdü)
len   17 → cap   32  (yerinde büyüdü)
len   33 → cap   64  (yerinde büyüdü)
len   65 → cap  128  (yerinde büyüdü)
len  129 → cap  256  (yerinde büyüdü)
len  257 → cap  512  (yerinde büyüdü)
len  513 → cap 1024  (yerinde büyüdü)
1000 push: üst sınır 1020 eleman (≈ push başına 1.02), bu çalıştırmada gerçekte taşınan 4
Vec::with_capacity(1000): cap 1000
ilk push sonrası cap: u8 → 8, u64 → 4, [u8; 2048] → 1
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "let cap = cmp::max(self.cap.as_inner() \* 2, required_cap)" $S/alloc/src/raw_vec/mod.rs; grep -n -A8 "^const fn min_non_zero_cap" $S/alloc/src/raw_vec/mod.rs
544:        let cap = cmp::max(self.cap.as_inner() * 2, required_cap);
168:const fn min_non_zero_cap(size: usize) -> usize {
169-    if size == 1 {
170-        8
171-    } else if size <= 1024 {
172-        4
173-    } else {
174-        1
175-    }
176-}
```

## 041-hash-tablolari

- Video: `041-hash-tablolari`
- Örnek yolu: `deneyler/mufredat-ornekleri/041-hash-tablolari/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: HashMap sırası ve hash değeri her süreçte değişir (rastgele tohum); BTreeMap ve sabit tohumlu HashMap sırası sabittir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/hash
k ./target/debug/hash
# Kaynak (nightly rust-src): HashMap hashbrown (SwissTable) üzerine kurulu; varsayılan algoritma SipHash 1-3, rastgele tohumlu
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "use hashbrown\|Rust port of Google\|SipHash 1-3\|HashDoS attacks. The" $S/std/src/collections/hash/map.rs'
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "k0.wrapping_add(1)\|SipHasher13::new_with_keys(self" $S/std/src/hash/random.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/hash
HashMap sırası : ["Muş", "Van", "İzmir", "Ankara", "Kars", "Bursa", "Rize"]
BTreeMap sırası: ["Ankara", "Bursa", "Kars", "Muş", "Rize", "Van", "İzmir"]
hash("Ankara") = 29ed4e8a2ccdab7c  (tablonun kendi hasher'ıyla)
aynı süreçte yeni bir RandomState aynı hash'i verdi mi: false
sabit tohumlu HashMap sırası: ["Ankara", "Rize", "Van", "İzmir", "Kars", "Bursa", "Muş"]
ilk bayt: "İzmir" 0xc4, "Van" 0x56
$ ./target/debug/hash
HashMap sırası : ["Rize", "Van", "Kars", "İzmir", "Muş", "Bursa", "Ankara"]
BTreeMap sırası: ["Ankara", "Bursa", "Kars", "Muş", "Rize", "Van", "İzmir"]
hash("Ankara") = 919a6126159dc497  (tablonun kendi hasher'ıyla)
aynı süreçte yeni bir RandomState aynı hash'i verdi mi: false
sabit tohumlu HashMap sırası: ["Ankara", "Rize", "Van", "İzmir", "Kars", "Bursa", "Muş"]
ilk bayt: "İzmir" 0xc4, "Van" 0x56
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "use hashbrown\|Rust port of Google\|SipHash 1-3\|HashDoS attacks. The" $S/std/src/collections/hash/map.rs
4:use hashbrown::hash_map::{self as base, RustcOccupiedError};
18:/// resistance against HashDoS attacks. The algorithm is randomly seeded, and a
26:/// The default hashing algorithm is currently SipHash 1-3, though this is
59:/// The hash table implementation is a Rust port of Google's [SwissTable].
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "k0.wrapping_add(1)\|SipHasher13::new_with_keys(self" $S/std/src/hash/random.rs
73:            keys.set((k0.wrapping_add(1), k1));
84:        DefaultHasher(SipHasher13::new_with_keys(self.k0, self.k1))
```

## 042-agaclar-ve-btreemap

- Video: `042-agaclar-ve-btreemap`
- Örnek yolu: `deneyler/mufredat-ornekleri/042-agaclar-ve-btreemap/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# Kaynak (nightly rust-src): dallanma sabiti B = 6, düğüm en çok 2B − 1 = 11 anahtar tutar
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^const B: usize\|const CAPACITY: usize" $S/alloc/src/collections/btree/node.rs'
# Düğümün içinde anahtarlar soldan sağa tek tek karşılaştırılır (doğrusal arama)
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "for (offset, k) in\|Ordering::Greater => {}" $S/alloc/src/collections/btree/search.rs'
# std belgesinin kendi açıklaması: geniş düğüm daha az ayırma ve daha iyi önbellek kullanımı, ama ortalamada daha çok karşılaştırma
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A2 "A B-Tree instead makes each node contain" $S/alloc/src/collections/btree/map.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
sıralı      : {"Ada": 64, "Ali": 72, "Can": 85, "Ece": 91, "Naz": 78}
'B'..'D'    : [("Can", 85)]
ilk / son   : Some(("Ada", 64)) / Some(("Naz", 78))
1000000 arama, arama başına karşılaştırma: BTreeMap 32.8 | sıralı dizide ikili arama 21.0
1000000 arama   : HashMap   45.2 ms, BTreeMap  115.9 ms
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^const B: usize\|const CAPACITY: usize" $S/alloc/src/collections/btree/node.rs
43:const B: usize = 6;
44:pub(super) const CAPACITY: usize = 2 * B - 1;
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "for (offset, k) in\|Ordering::Greater => {}" $S/alloc/src/collections/btree/search.rs
234:        for (offset, k) in unsafe { keys.get_unchecked(start_index..) }.iter().enumerate() {
236:                Ordering::Greater => {}
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A2 "A B-Tree instead makes each node contain" $S/alloc/src/collections/btree/map.rs
166:/// A B-Tree instead makes each node contain B-1 to 2B-1 elements in a contiguous array. By doing
167-/// this, we reduce the number of allocations by a factor of B, and improve cache efficiency in
168-/// searches. However, this does mean that searches will have to do *more* comparisons on average.
```

## 043-siralama-ve-arama

- Video: `043-siralama-ve-arama`
- Örnek yolu: `deneyler/mufredat-ornekleri/043-siralama-ve-arama/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# Kaynak (nightly rust-src): std'nin sıralama algoritmaları
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "is based on \[driftsort\]" $S/alloc/src/slice.rs | head -1; grep -n "is based on \[ipnsort\]" $S/core/src/slice/mod.rs | head -1'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
sort_by_key (kararlı): [("Ayşe", 1), ("Ece", 1), ("Deniz", 2), ("Ali", 3), ("Can", 3)]
rastgele     sort (kararlı) :  20832462 karşılaştırma
rastgele     sort_unstable  :  20565895 karşılaştırma
zaten sıralı sort (kararlı) :    999999 karşılaştırma
zaten sıralı sort_unstable  :    999999 karşılaştırma
n log2 n                    :  19931569
log2(n!) (alt sınır)        :  18488885
n - 1                       :    999999
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "is based on \[driftsort\]" $S/alloc/src/slice.rs | head -1; grep -n "is based on \[ipnsort\]" $S/core/src/slice/mod.rs | head -1
95:    /// The current implementation is based on [driftsort] by Orson Peters and Lukas Bergdoll, which
3118:    /// The current implementation is based on [ipnsort] by Lukas Bergdoll and Orson Peters, which
```

## 044-kuyruk-yigin-ve-graflar

- Video: `044-kuyruk-yigin-ve-graflar`
- Örnek yolu: `deneyler/mufredat-ornekleri/044-kuyruk-yigin-ve-graflar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# Kaynak (nightly rust-src): VecDeque bir dairesel tampon, BinaryHeap bir ikili öbek
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^/// A double-ended queue implemented with a growable ring buffer" $S/alloc/src/collections/vec_deque/mod.rs; grep -n "^/// A priority queue implemented with a binary heap" $S/alloc/src/collections/binary_heap/mod.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
BFS (kuyruk): S → E yolu 12 adım, 21 hücre ziyaret edildi
DFS (yığın) : S → E yolu 14 adım, 20 hücre ziyaret edildi
Dijkstra (öncelik kuyruğu): İstanbul → Ankara 450 km, yol: İstanbul → Bolu → Ankara
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "^/// A double-ended queue implemented with a growable ring buffer" $S/alloc/src/collections/vec_deque/mod.rs; grep -n "^/// A priority queue implemented with a binary heap" $S/alloc/src/collections/binary_heap/mod.rs
75:/// A double-ended queue implemented with a growable ring buffer.
159:/// A priority queue implemented with a binary heap.
```

## 045-onbellek-dostu-veri-yapilari

- Video: `045-onbellek-dostu-veri-yapilari`
- Örnek yolu: `deneyler/mufredat-ornekleri/045-onbellek-dostu-veri-yapilari/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler donanıma ve yüke bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# Kaynak (nightly rust-src): std'nin kendi notu, bağlı liste yerine dizi tabanlı yapıları önerir
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A2 "^//! NOTE: It is almost always better to use" $S/alloc/src/collections/linked_list.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
matris satır öncelikli gezinme             3.12 ms
matris sütun öncelikli gezinme            74.46 ms
Vec<u64> toplamı                           1.94 ms
LinkedList<u64> toplamı                    6.85 ms
rastgele zincirin tur uzunluğu: 8000000 (eleman sayısı 8000000)
kovalama: sonraki eleman yanda             9.29 ms
kovalama: sonraki eleman rastgele yerde   477.01 ms
AoS: Vec<Parcacik> içinden kütle           3.50 ms
SoA: Vec<f32> kütle dizisi                 0.63 ms
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A2 "^//! NOTE: It is almost always better to use" $S/alloc/src/collections/linked_list.rs
6://! NOTE: It is almost always better to use [`Vec`] or [`VecDeque`] because
7-//! array-based containers are generally faster,
8-//! more memory efficient, and make better use of CPU cache.
```

## 046-traitler-davranis-sozlesmesi

- Video: `046-traitler-davranis-sozlesmesi`
- Örnek yolu: `deneyler/mufredat-ornekleri/046-traitler-davranis-sozlesmesi/`
- Kaynaklar: `eksik/Cargo.toml`, `eksik/src/main.rs`, `sozlesme/Cargo.toml`, `sozlesme/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd sozlesme && k cargo run -q; cd ..
# Sözleşmenin bir maddesi (alan) yazılmazsa derleyici kabul etmez
cd eksik && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
daire (alan 12.57)
3 kenarlı kare
Daire { r: 2.0 } == Daire { r: 2.0 } → true
$ cargo build -q
error[E0046]: not all trait items implemented, missing: `alan`
 --> src/main.rs:8:1
  |
2 |     fn alan(&self) -> f64;
  |     ---------------------- `alan` from trait
...
8 | impl Sekil for Ucgen {
  | ^^^^^^^^^^^^^^^^^^^^ missing `alan` in implementation

For more information about this error, try `rustc --explain E0046`.
error: could not compile `eksik` (bin "eksik") due to 1 previous error
[çıkış kodu: 101]
```

## 047-jenerikler

- Video: `047-jenerikler`
- Örnek yolu: `deneyler/mufredat-ornekleri/047-jenerikler/`
- Kaynaklar: `sinirli/Cargo.toml`, `sinirli/src/main.rs`, `sinirsiz/Cargo.toml`, `sinirsiz/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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

## 048-tek-bicimlestirme

- Video: `048-tek-bicimlestirme`
- Örnek yolu: `deneyler/mufredat-ornekleri/048-tek-bicimlestirme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/tek_bicim
# Tek bir jenerik fonksiyon, ikilide üç ayrı makine kodu kopyası
ks 'nm -C target/release/tek_bicim | grep ikiye_katla'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/tek_bicim | grep -A3 "<tek_bicim::ikiye_katla::<"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/tek_bicim
42 2.5 14
$ nm -C target/release/tek_bicim | grep ikiye_katla
0000000000015e40 t tek_bicim::ikiye_katla::<f64>
0000000000015e48 t tek_bicim::ikiye_katla::<u8>
0000000000015e4c t tek_bicim::ikiye_katla::<i32>
$ objdump -d -M intel --no-show-raw-insn -C target/release/tek_bicim | grep -A3 "<tek_bicim::ikiye_katla::<"
0000000000015e40 <tek_bicim::ikiye_katla::<f64>>:
   15e40:	addsd  xmm0,xmm0
   15e44:	ret
   15e45:	int3
--
0000000000015e48 <tek_bicim::ikiye_katla::<u8>>:
   15e48:	lea    eax,[rdi+rdi*1]
   15e4b:	ret

0000000000015e4c <tek_bicim::ikiye_katla::<i32>>:
   15e4c:	lea    eax,[rdi+rdi*1]
   15e4f:	ret

--
   15e95:	call   15e4c <tek_bicim::ikiye_katla::<i32>>
   15e9a:	mov    DWORD PTR [rsp+0xc],eax
   15e9e:	movabs rax,0x3ff4000000000000
   15ea8:	mov    QWORD PTR [rsp+0x10],rax
--
   15eb8:	call   15e40 <tek_bicim::ikiye_katla::<f64>>
   15ebd:	movsd  QWORD PTR [rsp+0x40],xmm0
   15ec3:	mov    BYTE PTR [rsp+0x10],0x7
   15ec8:	lea    rax,[rsp+0x10]
--
   15ed2:	call   15e48 <tek_bicim::ikiye_katla::<u8>>
   15ed7:	mov    BYTE PTR [rsp+0xb],al
   15edb:	lea    rax,[rsp+0xc]
   15ee0:	mov    QWORD PTR [rsp+0x10],rax
```

## 049-trait-nesneleri-ve-vtable

- Video: `049-trait-nesneleri-ve-vtable`
- Örnek yolu: `deneyler/mufredat-ornekleri/049-trait-nesneleri-ve-vtable/`
- Kaynaklar: `dyn-uyumsuz/Cargo.toml`, `dyn-uyumsuz/src/main.rs`, `vtable/Cargo.toml`, `vtable/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: adresler (ASLR; Miri'nin sanal adresleri de sürüme bağlı)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd vtable && k cargo run -q; cd ..
# Aynı program Miri'de (Rust'ın soyut makinesini yorumlayan araç): vtable'ı sıradan bellek gibi okumak tanımsız davranış
cd vtable && k cargo +nightly miri run -q; cd ..
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
veri işaretçisi : 0x55dfe0719500
  Daire'nin adresiyle aynı mı: true
vtable          : 0x55dfe076ab78
  [0] drop      : 0x0
  [1] boyut     : 8
  [2] hizalama  : 8
  [3] alan()    : 0x55dfe072a260
<Daire as Sekil>::alan adresi: 0x55dfe072a260
$ cargo +nightly miri run -q
&Daire          :  8 bayt
&dyn Sekil      : 16 bayt (veri işaretçisi + vtable işaretçisi)
Box<dyn Sekil>  : 16 bayt
alan = 3.142
alan = 4.000
veri işaretçisi : 0x51ca8
  Daire'nin adresiyle aynı mı: true
vtable          : 0x51cb8
error: Undefined Behavior: constructing invalid value of type &usize: encountered a dangling reference (going beyond the bounds of its allocation)
  --> src/main.rs:35:45
   |
35 |         println!("  [0] drop      : {:#x}", *vtablo);
   |                                             ^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error

[çıkış kodu: 1]
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

## 050-statik-ve-dinamik-yonlendirme

- Video: `050-statik-ve-dinamik-yonlendirme`
- Örnek yolu: `deneyler/mufredat-ornekleri/050-statik-ve-dinamik-yonlendirme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/yonlendirme
# Üç fonksiyon yazdık, ikilide iki sembol var: &dyn ve Box<dyn> sürümleri aynı makine koduna derlendi ve birleştirildi
ks 'nm -C target/release/yonlendirme | grep toplam'
# Karşılaştırma için nightly'nin varsayılan derlemesi: yine iki sembol (fark araç zincirinden değil bayraktan)
ks 'cargo +nightly build -q --release --target-dir target/nightly && nm -C target/nightly/release/yonlendirme | grep toplam'
# Nedeni LLVM'in fonksiyon birleştirme (MergeFunctions) geçişi: nightly derleyicide bu geçiş kapatılınca üç sembol kalır
ks 'RUSTFLAGS="-Z merge-functions=disabled" cargo +nightly build -q --release --target-dir target/birlestirmesiz && nm -C target/birlestirmesiz/release/yonlendirme | grep toplam'
# Dinamik sürümde dolaylı çağrı: adres vtable'ın 3 numaralı girdisinden (+0x18) okunup çağrılıyor
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/yonlendirme | awk "/^0000.*<yonlendirme::toplam_kutulu>:/{f=1} f&&/^$/{exit} f" | grep -E "call|ret"'
# Statik sürümde hiç call yok: alan() satır içine alınmış, döngü açılmış (çarpma ve toplama komutları sayılıyor)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/yonlendirme | awk "/^0000.*<yonlendirme::toplam_statik::<yonlendirme::Kare>>:/{f=1} f&&/^$/{exit} f" | grep -oE "\b(call|mulsd|addsd)\b" | sort | uniq -c'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/yonlendirme
statik   7.11 ms | &dyn  48.58 ms | Box<dyn>  48.39 ms | eşit: true
statik   7.02 ms | &dyn  48.24 ms | Box<dyn>  48.76 ms | eşit: true
$ nm -C target/release/yonlendirme | grep toplam
0000000000016be0 t yonlendirme::toplam_statik::<yonlendirme::Kare>
0000000000016e30 t yonlendirme::toplam_kutulu
$ cargo +nightly build -q --release --target-dir target/nightly && nm -C target/nightly/release/yonlendirme | grep toplam
0000000000016a50 t yonlendirme::toplam_statik::<yonlendirme::Kare>
0000000000016ca0 t yonlendirme::toplam_kutulu
$ RUSTFLAGS="-Z merge-functions=disabled" cargo +nightly build -q --release --target-dir target/birlestirmesiz && nm -C target/birlestirmesiz/release/yonlendirme | grep toplam
0000000000016a80 t yonlendirme::toplam_statik::<yonlendirme::Kare>
0000000000016cd0 t yonlendirme::toplam_kutulu
0000000000016d50 t yonlendirme::toplam_dinamik
$ objdump -d -M intel --no-show-raw-insn -C target/release/yonlendirme | awk "/^0000.*<yonlendirme::toplam_kutulu>:/{f=1} f&&/^$/{exit} f" | grep -E "call|ret"
   16e6f:	call   QWORD PTR [rax+0x18]
   16e9a:	ret
   16ea3:	ret
$ objdump -d -M intel --no-show-raw-insn -C target/release/yonlendirme | awk "/^0000.*<yonlendirme::toplam_statik::<yonlendirme::Kare>>:/{f=1} f&&/^$/{exit} f" | grep -oE "\b(call|mulsd|addsd)\b" | sort | uniq -c
      5 addsd
      5 mulsd
```

## 051-standart-traitler-ve-donusumler

- Video: `051-standart-traitler-ve-donusumler`
- Örnek yolu: `deneyler/mufredat-ornekleri/051-standart-traitler-ve-donusumler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: 1000 kümenin kaçının 1 eleman tuttuğu (rastgele tohum; beklenen ≈ 1/128); panik satırındaki iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# Kaynak (nightly rust-src): Hash ile Eq birlikte uygulanınca sağlanması gereken özellik
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -B3 "^/// k1 == k2 -> hash(k1) == hash(k2)" $S/core/src/hash/mod.rs'
# Kaynak: bu özelliği bozmak bir "mantık hatası"dır, sonucu belirtilmemiştir
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A1 "^/// Violating this property is a logic error" $S/core/src/hash/mod.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
eşit mi: true
1000 yeni küme: 2 elemanlı 992, 1 elemanlı 8
100 °C = 212 °F
total_cmp ile: [-NaN, -1, 0, 2.5, +NaN]

thread 'main' (7607) panicked at src/main.rs:57:46:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -B3 "^/// k1 == k2 -> hash(k1) == hash(k2)" $S/core/src/hash/mod.rs
138-/// property holds:
139-///
140-/// ```text
141:/// k1 == k2 -> hash(k1) == hash(k2)
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A1 "^/// Violating this property is a logic error" $S/core/src/hash/mod.rs
150:/// Violating this property is a logic error. The behavior resulting from a logic error is not
151-/// specified, but users of the trait must ensure that such logic errors do *not* result in
```

## 052-closurelar

- Video: `052-closurelar`
- Örnek yolu: `deneyler/mufredat-ornekleri/052-closurelar/`
- Kaynaklar: `fnonce/Cargo.toml`, `fnonce/src/main.rs`, `yakalama/Cargo.toml`, `yakalama/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
&String yakalayan      :  8 bayt
1 11 1 2 5
String taşıyan (move)  : 24 bayt
Deniz
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

## 053-yineleyici-protokolu

- Video: `053-yineleyici-protokolu`
- Örnek yolu: `deneyler/mufredat-ornekleri/053-yineleyici-protokolu/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
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
ks 'diff <(objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) <(objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) && echo "aynı komut kümesi, yalnızca sıralama farklı"'
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
$ diff <(objdump -d -M intel --no-show-raw-insn --disassemble=elle_dongu target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) <(objdump -d -M intel --no-show-raw-insn --disassemble=iterator_zinciri target/iki_yol.o | grep -P "^\s+[0-9a-f]+:" | cut -f2- | sed "s/<[^>]*>//" | sort) && echo "aynı komut kümesi, yalnızca sıralama farklı"
aynı komut kümesi, yalnızca sıralama farklı
```

## 055-fonksiyonel-desenler

- Video: `055-fonksiyonel-desenler`
- Örnek yolu: `deneyler/mufredat-ornekleri/055-fonksiyonel-desenler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
{"açalım": 1, "beraber": 1, "berber": 2, "berbere": 1, "bir": 3, "demiş": 1, "dükkânı": 1, "gel": 1}
ayrıştırılıyor: 10
ayrıştırılıyor: 20
ayrıştırılıyor: otuz
hepsi ya da hiç : Err(ParseIntError { kind: InvalidDigit })
geçerli olanlar : [10, 20, 40]
çift [2, 4, 6, 8, 10], tek [1, 3, 5, 7, 9]
```

## 056-moduller-ve-gorunurluk

- Video: `056-moduller-ve-gorunurluk`
- Örnek yolu: `deneyler/mufredat-ornekleri/056-moduller-ve-gorunurluk/`
- Kaynaklar: `acik-alan/Cargo.toml`, `acik-alan/src/main.rs`, `gizli-alan/Cargo.toml`, `gizli-alan/src/main.rs`, `kapsulleme/Cargo.toml`, `kapsulleme/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd kapsulleme && k cargo run -q; cd ..
# Gizli alana modülün dışından dokunmak derlenmez
cd gizli-alan && k cargo build -q; cd ..
# Alan pub olsaydı: program derlenir, değişmez bozulur ve ikili arama yanlış cevap verir
cd acik-alan && k cargo run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
liste: [1, 2, 3, 4, 5]
0 var mı: false, 4 var mı: true
$ cargo build -q
error[E0616]: field `ogeler` of struct `SiraliListe` is private
  --> src/main.rs:31:11
   |
31 |     liste.ogeler.push(0); // değişmezi dışarıdan bozmaya çalış: sona 0 ekle
   |           ^^^^^^ private field
   |
help: a method `ogeler` also exists, call it with parentheses
   |
31 |     liste.ogeler().push(0); // değişmezi dışarıdan bozmaya çalış: sona 0 ekle
   |                 ++

For more information about this error, try `rustc --explain E0616`.
error: could not compile `gizli_alan` (bin "gizli_alan") due to 1 previous error
[çıkış kodu: 101]
$ cargo run -q
liste: [1, 2, 3, 4, 5, 0]
0 var mı: false, 4 var mı: true
```

## 057-crateler-ve-bagimliliklar

- Video: `057-crateler-ve-bagimliliklar`
- Örnek yolu: `deneyler/mufredat-ornekleri/057-crateler-ve-bagimliliklar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `itoa 1.0.19`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.10`, `unicode-ident 1.0.27`, `zmij 1.0.23`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: cargo update --dry-run çıktısı crates.io'da yayımlanan yeni sürümlere göre değişir (ağ gerekir)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cat Cargo.toml
k cargo run -q
# "1" gereksinimi hangi sürüme çözüldü? Bağımlılık ağacı:
k cargo tree
# Geçişli bağımlılık: memchr'i projeye kim getirdi?
k cargo tree -e normal -i memchr
# Cargo.lock: tam sürüm + .crate dosyasının SHA-256 özeti (checksum alanı)
ks 'grep -A3 "name = \"serde_json\"" Cargo.lock'
# Özeti kendimiz hesaplayalım: cargo'nun önbelleğindeki .crate dosyasının SHA-256'sı
ks 'sha256sum /usr/local/cargo/registry/cache/*/serde_json-1.0.154.crate | cut -d" " -f1'
# Kilit dosyası olmasaydı bugün hangi sürümler seçilirdi? (--dry-run: dosyaya yazmaz)
k cargo update --dry-run
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
{"ad":"Deniz","kol":8,"tur":"ahtapot"}
$ cargo tree
json_ornek v0.1.0 (/work/mufredat-ornekleri/057-crateler-ve-bagimliliklar)
└── serde_json v1.0.154
    ├── itoa v1.0.19
    ├── memchr v2.8.3
    ├── serde_core v1.0.229
    └── zmij v1.0.23
$ cargo tree -e normal -i memchr
memchr v2.8.3
└── serde_json v1.0.154
    └── json_ornek v0.1.0 (/work/mufredat-ornekleri/057-crateler-ve-bagimliliklar)
$ grep -A3 "name = \"serde_json\"" Cargo.lock
name = "serde_json"
version = "1.0.154"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e7e9cc8b1b85264074fbcc02a88680c4096b1e47df8f739dceb03bf482f04bd6"
$ sha256sum /usr/local/cargo/registry/cache/*/serde_json-1.0.154.crate | cut -d" " -f1
e7e9cc8b1b85264074fbcc02a88680c4096b1e47df8f739dceb03bf482f04bd6
$ cargo update --dry-run
    Updating crates.io index
     Locking 0 packages to highest Rust 1.100.0 compatible versions
warning: not updating lockfile due to dry run
```

## 058-tedarik-zinciri-guvenligi

- Video: `058-tedarik-zinciri-guvenligi`
- Örnek yolu: `deneyler/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/`
- Kaynaklar: `asgari/Cargo.toml`, `asgari/src/main.rs`, `derleme-betigi/Cargo.toml`, `derleme-betigi/src/main.rs`, `derleme-betigi/yardimci/Cargo.toml`, `derleme-betigi/yardimci/build.rs`, `derleme-betigi/yardimci/src/lib.rs`, `tam/Cargo.toml`, `tam/src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: cargo search sürümleri crates.io'ya göre değişir (ağ gerekir); build.rs'nin gördüğü girdi sayısı ortama bağlıdır
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# 1) Aynı crate, farklı özellik (feature) seçimi: kaç paket derlemeye girer?
cd tam
k grep tokio Cargo.toml
k cargo fetch -q
# Farklı paket sayısı (kendi paketimiz dahil); --no-dedupe tekrar edenleri "(*)" ile kısaltmadan yazar
ks 'cargo tree --prefix none -e normal --no-dedupe | sort -u | wc -l'
k cargo tree --depth 1 -e normal
# Bunların bir kısmı prosedürel makro ve onun bağımlılıkları: ikiliye girmez, derleme anında çalışır
k cargo tree -e normal -i proc-macro2
cd ../asgari
k grep tokio Cargo.toml
k cargo fetch -q
ks 'cargo tree --prefix none -e normal --no-dedupe | sort -u | wc -l'
k cargo tree -e normal
cd ..
# 2) Bir bağımlılığın kodu derleme anında, bizim yetkilerimizle çalışabilir: derleme betiği (build.rs)
cd derleme-betigi
cargo clean -q 2>/dev/null
ks 'cargo build 2>&1 | grep -v "^ *Finished"'
k ./target/debug/uygulama
cd ..
# 3) Denetim araçları crates.io'da yayımlanıyor (bu imajda kurulu değil)
ks 'for a in cargo-audit cargo-deny cargo-vet; do cargo search --limit 1 $a | head -n 1; done'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ grep tokio Cargo.toml
tokio = { version = "1", features = ["full"] }
$ cargo fetch -q
$ cargo tree --prefix none -e normal --no-dedupe | sort -u | wc -l
20
$ cargo tree --depth 1 -e normal
tam v0.1.0 (/work/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/tam)
└── tokio v1.54.1
$ cargo tree -e normal -i proc-macro2
proc-macro2 v1.0.107
├── quote v1.0.47
│   ├── syn v3.0.10
│   │   └── tokio-macros v2.7.2 (proc-macro)
│   │       └── tokio v1.54.1
│   │           └── tam v0.1.0 (/work/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/tam)
│   └── tokio-macros v2.7.2 (proc-macro) (*)
├── syn v3.0.10 (*)
└── tokio-macros v2.7.2 (proc-macro) (*)
$ grep tokio Cargo.toml
tokio = { version = "1", default-features = false, features = ["rt"] }
$ cargo fetch -q
$ cargo tree --prefix none -e normal --no-dedupe | sort -u | wc -l
3
$ cargo tree -e normal
asgari v0.1.0 (/work/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/asgari)
└── tokio v1.54.1
    └── pin-project-lite v0.2.18
$ cargo build 2>&1 | grep -v "^ *Finished"
   Compiling yardimci v0.1.0 (/work/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/derleme-betigi/yardimci)
warning: yardimci@0.1.0: build.rs çalıştı: HOME = /root, orada 2 girdi görebiliyorum
   Compiling uygulama v0.1.0 (/work/mufredat-ornekleri/058-tedarik-zinciri-guvenligi/derleme-betigi)
$ ./target/debug/uygulama
merhaba, ben yardimci crate'iyim
$ for a in cargo-audit cargo-deny cargo-vet; do cargo search --limit 1 $a | head -n 1; done
cargo-audit = "0.23.0"    # Audit Cargo.lock for crates with security vulnerabilities
note: to learn more about a package, run `cargo info <name>`
cargo-deny = "0.22.1"    # Cargo plugin to help you manage large dependency graphs
note: to learn more about a package, run `cargo info <name>`
cargo-vet = "0.10.3"    # Supply-chain security for Rust
note: to learn more about a package, run `cargo info <name>`
```

## 059-birim-ve-entegrasyon-testleri

- Video: `059-birim-ve-entegrasyon-testleri`
- Örnek yolu: `deneyler/mufredat-ornekleri/059-birim-ve-entegrasyon-testleri/`
- Kaynaklar: `duzeltilmis/Cargo.toml`, `duzeltilmis/src/lib.rs`, `duzeltilmis/tests/disaridan.rs`, `hatali/Cargo.toml`, `hatali/src/lib.rs`, `hatali/tests/disaridan.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: test süreleri, iş parçacığı kimliği
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Birim + entegrasyon + dokümantasyon testleri (ilk hatada durmadan)
cd hatali && k cargo test -q --no-fail-fast; cd ..
# Düzeltme: fark (dosya tarihleri yazılmasın diye etiketle)
ks 'diff -u --label hatali/src/lib.rs --label duzeltilmis/src/lib.rs hatali/src/lib.rs duzeltilmis/src/lib.rs'
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

thread 'testler::cift_uzunlukta_medyan' (8978) panicked at src/lib.rs:40:9:
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

all doctests ran in 0.12s; merged doctests compilation took 0.11s
error: 1 target failed:
    `--lib`
[çıkış kodu: 101]
$ diff -u --label hatali/src/lib.rs --label duzeltilmis/src/lib.rs hatali/src/lib.rs duzeltilmis/src/lib.rs
--- hatali/src/lib.rs
+++ duzeltilmis/src/lib.rs
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
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: proptest rastgele girdilerle başlar; küçültülmüş (shrink) çiftin sayıları ve süreler her çalıştırmada değişir, toplamları değişmez
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
rm -rf proptest-regressions
ks 'cargo test -q > /tmp/test.txt 2>&1; grep -m1 -B1 "attempt to add with overflow" /tmp/test.txt; grep -E "minimal failing input|^test result" /tmp/test.txt'
# Beklenen satırlar yoksa (ör. derleme ya da ağ hatası) test çıktısının sonu kayda düşsün
ks 'grep -q "minimal failing input" /tmp/test.txt || tail -n 20 /tmp/test.txt'
# Küçültülmüş çiftin toplamı: taşma sınırı 2^32 = 4294967296
ks 'grep -o "a = [0-9]*, b = [0-9]*" /tmp/test.txt | head -1 | tr -dc "0-9 \n" | awk "{print \"a + b =\", \$1 + \$2}"'
# proptest bulduğu karşı örneğin tohumunu bir regresyon dosyasına yazar; sonraki çalıştırma önce onu dener
ks 'ls proptest-regressions && grep -c "^cc " proptest-regressions/*.txt'
rm -rf proptest-regressions
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q > /tmp/test.txt 2>&1; grep -m1 -B1 "attempt to add with overflow" /tmp/test.txt; grep -E "minimal failing input|^test result" /tmp/test.txt
thread 'testler::saf_surum_aralikta' (9697) panicked at src/lib.rs:3:5:
attempt to add with overflow
minimal failing input: a = 1313757965, b = 2981209331
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ grep -q "minimal failing input" /tmp/test.txt || tail -n 20 /tmp/test.txt
$ grep -o "a = [0-9]*, b = [0-9]*" /tmp/test.txt | head -1 | tr -dc "0-9 \n" | awk "{print \"a + b =\", \$1 + \$2}"
a + b = 4294967296
$ ls proptest-regressions && grep -c "^cc " proptest-regressions/*.txt
lib.txt
1
```

## 061-clippy-rustfmt-ve-ci

- Video: `061-clippy-rustfmt-ve-ci`
- Örnek yolu: `deneyler/mufredat-ornekleri/061-clippy-rustfmt-ve-ci/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
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
kök ortada gezinme (sol, kök, sağ): [20, 30, 35, 40, 50, 60, 70, 80], derinlik 4
size_of::<Box<Agac>>() = 8 bayt, size_of::<Agac>() = 24 bayt
```

## 063-rc-ve-arc-referans-sayma

- Video: `063-rc-ve-arc-referans-sayma`
- Örnek yolu: `deneyler/mufredat-ornekleri/063-rc-ve-arc-referans-sayma/`
- Kaynaklar: `rc-is-parcacigi/Cargo.toml`, `rc-is-parcacigi/src/main.rs`, `sayac/Cargo.toml`, `sayac/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd sayac && k cargo run -q; cd ..
# Rc'yi başka bir iş parçacığına göndermek derlenmez (Rc: !Send); Arc'ın atomik sayacı bunun için var
cd rc-is-parcacigi && k cargo build -q; cd ..
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
$ cargo build -q
error[E0277]: `Rc<Vec<i32>>` cannot be sent between threads safely
 --> src/main.rs:8:27
  |
8 |     let t = thread::spawn(move || kopya.len());
  |             ------------- -------^^^^^^^^^^^^
  |             |             |
  |             |             `Rc<Vec<i32>>` cannot be sent between threads safely
  |             |             within this `{closure@src/main.rs:8:27: 8:34}`
  |             required by a bound introduced by this call
  |
  = help: within `{closure@src/main.rs:8:27: 8:34}`, the trait `Send` is not implemented for `Rc<Vec<i32>>`
note: required because `kopya` is used within this closure
 --> src/main.rs:8:35
  |
8 |     let t = thread::spawn(move || kopya.len());
  |                                   ^^^^^
note: required by a bound in `spawn`
 --> /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/thread/functions.rs:125:0

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rc_is_parcacigi` (bin "rc_is_parcacigi") due to 1 previous error
[çıkış kodu: 101]
```

## 064-ic-degistirilebilirlik

- Video: `064-ic-degistirilebilirlik`
- Örnek yolu: `deneyler/mufredat-ornekleri/064-ic-degistirilebilirlik/`
- Kaynaklar: `hucreler/Cargo.toml`, `hucreler/src/main.rs`, `sizinti/Cargo.toml`, `sizinti/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: panik satırındaki iş parçacığı kimliği; Miri'nin ayırma numaraları sürüme bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd hucreler && k cargo run -q; cd ..
# Sızıntıyı bir araçla görelim: Miri program bitince iade edilmemiş öbek bloklarını raporlar
cd sizinti && k cargo +nightly miri run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
Cell sayaç: 2
RefCell: [1, 2, 3]
--- Rc + RefCell ile döngü:
x sayaç 2, y sayaç 2
kapsam bitti: x'in güçlü sayacı hâlâ 1, hiçbiri bırakılmadı: bellek sızdı (güvenli ama sızıntı)
--- Weak geri bağlantı ile:
ebeveyn güçlü 1, zayıf 1
  ebeveyn bırakıldı
kapsam bitti
--- aynı anda iki borrow_mut:

thread 'main' (10098) panicked at src/main.rs:64:25:
RefCell already borrowed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
$ cargo +nightly miri run -q
x sayaç 2, y sayaç 2
error: memory leaked: alloc385 (Rust heap, size: 32, align: 8), allocated here:
   --> /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rcs/rc.rs:438:49
    |
438 |               Self::from_inner(Box::into_non_null(Box::new(RcInner {
    |  _________________________________________________^
439 | |                 strong: Cell::new(1),
440 | |                 weak: Cell::new(1),
441 | |                 value,
442 | |             })))
    | |______________^
    |
    = note: stack backtrace:
            0: std::rc::Rc::<Dugum>::new
                at /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rcs/rc.rs:438:49: 442:15
            1: main
                at src/main.rs:10:13: 10:72

error: memory leaked: alloc319 (Rust heap, size: 32, align: 8), allocated here:
   --> /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rcs/rc.rs:438:49
    |
438 |               Self::from_inner(Box::into_non_null(Box::new(RcInner {
    |  _________________________________________________^
439 | |                 strong: Cell::new(1),
440 | |                 weak: Cell::new(1),
441 | |                 value,
442 | |             })))
    | |______________^
    |
    = note: stack backtrace:
            0: std::rc::Rc::<Dugum>::new
                at /usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rcs/rc.rs:438:49: 442:15
            1: main
                at src/main.rs:9:13: 9:57

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

note: set `MIRIFLAGS=-Zmiri-ignore-leaks` to disable this check

error: aborting due to 2 previous errors

[çıkış kodu: 1]
```

## 065-bellek-ayiricilar

- Video: `065-bellek-ayiricilar`
- Örnek yolu: `deneyler/mufredat-ornekleri/065-bellek-ayiricilar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: blok adımları ve yeniden kullanım sırası sistemin ayırıcısına (burada glibc malloc) bağlıdır
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# Kaynak (nightly rust-src): System ayırıcısı Unix'te küçük hizalamalar için libc'nin malloc'unu çağırır
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "libc::malloc(layout.size())\|libc::realloc(ptr as \*mut libc::c_void, new_size)" $S/std/src/sys/alloc/unix.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
istenen   8 bayt → bloklar 32 bayt arayla
istenen  25 bayt → bloklar 48 bayt arayla
istenen 100 bayt → bloklar 112 bayt arayla
24 baytlık yeni kutular, iade edilen 8 baytlıkların adreslerini ters sırayla aldı mı: true
String::new + 1000 × push_str                 ayırma     1  büyütme   8
String::with_capacity(2000) + 1000 × push_str ayırma     1  büyütme   0
1000 × format!("{i}") → Vec<String>           ayırma  1001  büyütme   0
1000 × Box::new(i)                            ayırma  1001  büyütme   0
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "libc::malloc(layout.size())\|libc::realloc(ptr as \*mut libc::c_void, new_size)" $S/std/src/sys/alloc/unix.rs
31:        unsafe { libc::malloc(layout.size()) as *mut u8 }
71:        unsafe { libc::realloc(ptr as *mut libc::c_void, new_size) as *mut u8 }
```

## 066-eszamanlilik-ve-is-parcaciklari

- Video: `066-eszamanlilik-ve-is-parcaciklari`
- Örnek yolu: `deneyler/mufredat-ornekleri/066-eszamanlilik-ve-is-parcaciklari/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: görülen sıralar, süreler, hızlanma oranları ve bağlam değiştirme sayıları her çalıştırmada ve her makinede değişir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Bu makinenin işlemcisi: kaç çekirdek, çekirdek başına kaç donanım iş parçacığı?
ks 'lscpu | grep -E "^(CPU\(s\)|Thread\(s\) per core|Core\(s\) per socket):"'
k cargo build -q --release
# 1) Sıra garantisi yok
k ./target/release/is_parcacigi sira
# 2) İşi bölmek: blok, döngüsel ve dinamik dağıtım (son satır: çekirdeğin bu süreçte yaptığı bağlam değiştirmeler)
ks '/usr/bin/time -f "bağlam değiştirme: gönüllü %w, zorunlu %c" ./target/release/is_parcacigi hizlanma 2>&1'
# Kendin dene: programı tek bir mantıksal işlemciye kısıtla (eşzamanlı ama paralel değil)
ks '/usr/bin/time -f "bağlam değiştirme: gönüllü %w, zorunlu %c" taskset -c 0 ./target/release/is_parcacigi hizlanma 2>&1'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ lscpu | grep -E "^(CPU\(s\)|Thread\(s\) per core|Core\(s\) per socket):"
CPU(s):                                  12
Thread(s) per core:                      2
Core(s) per socket:                      6
$ cargo build -q --release
$ ./target/release/is_parcacigi sira
1000 denemede 10 farklı sıra görüldü; en sık: [("0123", 972), ("0132", 15), ("0213", 4), ("0231", 3)]
$ /usr/bin/time -f "bağlam değiştirme: gönüllü %w, zorunlu %c" ./target/release/is_parcacigi hizlanma 2>&1
available_parallelism = 12
4 iş parçacığının kendi süreleri (ms), blok    :  63 103 127 145
4 iş parçacığının kendi süreleri (ms), döngüsel:   1 218   1 217
4 iş parçacığının kendi süreleri (ms), dinamik : 110 110 109 110
1 iş parçacığı: 216816 asal, 446.6 ms
 2 iş parçacığı | blok 271.8 ms (1.64×) | döngüsel 442.5 ms (1.01×) | dinamik 222.5 ms (2.01×)
 4 iş parçacığı | blok 147.5 ms (3.03×) | döngüsel 220.1 ms (2.03×) | dinamik 111.6 ms (4.00×)
 6 iş parçacığı | blok  99.6 ms (4.49×) | döngüsel 218.6 ms (2.04×) | dinamik  74.0 ms (6.03×)
12 iş parçacığı | blok  87.2 ms (5.12×) | döngüsel 109.9 ms (4.06×) | dinamik  74.7 ms (5.98×)
bağlam değiştirme: gönüllü 62, zorunlu 888
$ /usr/bin/time -f "bağlam değiştirme: gönüllü %w, zorunlu %c" taskset -c 0 ./target/release/is_parcacigi hizlanma 2>&1
available_parallelism = 1
4 iş parçacığının kendi süreleri (ms), blok    : 247 367 415 433
4 iş parçacığının kendi süreleri (ms), döngüsel:   4 435   6 434
4 iş parçacığının kendi süreleri (ms), dinamik : 435 433 431 433
1 iş parçacığı: 216816 asal, 435.6 ms
 2 iş parçacığı | blok 436.6 ms (1.00×) | döngüsel 437.0 ms (1.00×) | dinamik 437.0 ms (1.00×)
 4 iş parçacığı | blok 436.4 ms (1.00×) | döngüsel 437.4 ms (1.00×) | dinamik 436.8 ms (1.00×)
bağlam değiştirme: gönüllü 27, zorunlu 1954
```

## 067-veri-yarisi-ve-send-sync

- Video: `067-veri-yarisi-ve-send-sync`
- Örnek yolu: `deneyler/mufredat-ornekleri/067-veri-yarisi-ve-send-sync/`
- Kaynaklar: `arc-mutex/Cargo.toml`, `arc-mutex/src/main.rs`, `atomik/Cargo.toml`, `atomik/src/main.rs`, `ortak-mut/Cargo.toml`, `ortak-mut/src/main.rs`, `rc-gonderilemez/Cargo.toml`, `rc-gonderilemez/src/main.rs`, `yaris/Cargo.toml`, `yaris/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: yarışlı sayacın sonucu ve adresler her çalıştırmada değişir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# 1) unsafe Rust ile bilerek yazılmış veri yarışı: kaybolan artışlar
cd yaris
k cargo build -q --release
k ./target/release/yaris
k ./target/release/yaris
# Makine kodu: belleği doğrudan artıran tek bir inc komutu (lock öneki yok)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/yaris | grep -A12 "<yaris::artir>:" | grep -m1 "inc"'
# miri (Rust'ın tanımsız davranış denetleyicisi) aynı programı yorumlayarak çalıştırır
ks 'cargo +nightly miri run -q -- 100 2>&1 | head -n 12'
cd ..
# 2) Güvenli Rust: aynı paylaşım denemeleri derlenmez
cd ortak-mut && k cargo build -q; cd ..
cd rc-gonderilemez && k cargo build -q; cd ..
# 3) Atomik sayaç: doğru sonuç; makine kodunda aynı inc komutu, bu kez lock önekiyle
cd atomik
k cargo run -q --release
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/atomik | grep -A12 "<atomik::artir>:" | grep -m1 lock'
cd ..
# Kendin dene: rc-gonderilemez/'de Rc → Arc, RefCell → Mutex, borrow_mut()/borrow() → lock().unwrap()
cd arc-mutex && k cargo run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/yaris
beklenen 20000000, bulunan 11033628
$ ./target/release/yaris
beklenen 20000000, bulunan 13847993
$ objdump -d -M intel --no-show-raw-insn -C target/release/yaris | grep -A12 "<yaris::artir>:" | grep -m1 "inc"
   18e60:	inc    QWORD PTR [rip+0x44851]        # 5d6b8 <yaris::SAYAC>
$ cargo +nightly miri run -q -- 100 2>&1 | head -n 12
error: Undefined Behavior: Data race detected between (1) non-atomic write on thread `unnamed-1` and (2) non-atomic read on thread `unnamed-2` at alloc1
  --> src/main.rs:13:24
   |
13 |             let eski = p.read_volatile(); // 1) oku
   |                        ^^^^^^^^^^^^^^^^^ (2) just happened here
   |
help: and (1) occurred earlier here
  --> src/main.rs:14:13
   |
14 |             p.write_volatile(eski + 1); //   2) bir ekle, 3) yaz
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
$ cargo build -q
error[E0499]: cannot borrow `sayac` as mutable more than once at a time
 --> src/main.rs:8:17
  |
6 |     thread::scope(|s| {
  |                    - has type `&'1 Scope<'1, '_>`
7 |         s.spawn(|| sayac += 1);
  |         ----------------------
  |         |       |  |
  |         |       |  first borrow occurs due to use of `sayac` in closure
  |         |       first mutable borrow occurs here
  |         argument requires that `sayac` is borrowed for `'1`
8 |         s.spawn(|| sayac += 1);
  |                 ^^ ----- second borrow occurs due to use of `sayac` in closure
  |                 |
  |                 second mutable borrow occurs here
  |
note: requirement that the value outlives `'1` introduced here
 --> /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/thread/scoped.rs:203:34

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ortak_mut` (bin "ortak_mut") due to 1 previous error
[çıkış kodu: 101]
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
$ objdump -d -M intel --no-show-raw-insn -C target/release/atomik | grep -A12 "<atomik::artir>:" | grep -m1 lock
   18a70:	lock inc QWORD PTR [rip+0x44718]        # 5d190 <atomik::SAYAC>
$ cargo run -q
1
```

## 068-mutex-ve-kilitler

- Video: `068-mutex-ve-kilitler`
- Örnek yolu: `deneyler/mufredat-ornekleri/068-mutex-ve-kilitler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler, futex çağrı sayıları, adresler ve iş parçacığı kimlikleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
# 1) Aynı sayaç üç yolla: Mutex, atomik, paylaşmamak
k ./target/release/kilitler sayaclar
# 2) Çekişmenin çekirdekteki izi: futex sistem çağrıları (1 iş parçacığı ve 8 iş parçacığı)
ks 'strace -f -c -e trace=futex ./target/release/kilitler mutex 1 2>&1 | grep -E "^Mutex|futex$|total$"'
ks 'strace -f -c -e trace=futex ./target/release/kilitler mutex 8 2>&1 | grep -E "^Mutex|futex$|total$"'
# Tek iş parçacığındaki tek futex çağrısı nereden? Ana iş parçacığı, alt iş parçacığının bitmesini (join) bekliyor
ks 'strace -f -e trace=futex,clone3 ./target/release/kilitler mutex 1 2>&1 | grep -E "clone3|futex"'
# 3) Kritik bölgeyi kısa tutmak: yavaş işi kilidin dışına almak
k ./target/release/kilitler kilitte-bekleme
# 4) Zehirlenme
k ./target/release/kilitler zehir
# Kendin dene: futex çağrısı sayısı 2 ve 4 iş parçacığıyla; iş 5 ms sürünce kilitte bekleme
ks 'for n in 2 4; do strace -f -c -e trace=futex ./target/release/kilitler mutex $n 2>&1 | grep -E "^Mutex|futex$"; done'
k ./target/release/kilitler kilitte-bekleme 5
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/kilitler sayaclar
Mutex           : 8000000 ( 223.4 ms)
AtomicU64       : 8000000 (  82.7 ms)
yerel + toplama : 8000000 (   0.6 ms)
$ strace -f -c -e trace=futex ./target/release/kilitler mutex 1 2>&1 | grep -E "^Mutex|futex$|total$"
Mutex, 1 iş parçacığı: sayaç = 1000000
100.00    0.000224         224         1           futex
100.00    0.000224         224         1           total
$ strace -f -c -e trace=futex ./target/release/kilitler mutex 8 2>&1 | grep -E "^Mutex|futex$|total$"
Mutex, 8 iş parçacığı: sayaç = 8000000
100.00    0.175140          14     11947      2251 futex
100.00    0.175140          14     11947      2251 total
$ strace -f -e trace=futex,clone3 ./target/release/kilitler mutex 1 2>&1 | grep -E "clone3|futex"
clone3({flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID, child_tid=0x7f2e27d5a990, parent_tid=0x7f2e27d5a990, exit_signal=0, stack=0x7f2e27b5a000, stack_size=0x1fff00, tls=0x7f2e27d5a6c0}strace: Process 14916 attached
[pid 14915] futex(0x7f2e27d5a990, FUTEX_WAIT_BITSET|FUTEX_CLOCK_REALTIME, 14916, NULL, FUTEX_BITSET_MATCH_ANY) = 0
$ ./target/release/kilitler kilitte-bekleme
iş kilidin içinde : 8 × 20 iş, iş başına 1 ms,  168.8 ms
iş kilidin dışında: 8 × 20 iş, iş başına 1 ms,   21.2 ms
$ ./target/release/kilitler zehir

thread '<unnamed>' (14935) panicked at src/main.rs:102:9:
kilit elimdeyken çöktüm
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
kilit zehirlenmiş! içerik yine de okunabilir: [1, 2, 3, 4]
$ for n in 2 4; do strace -f -c -e trace=futex ./target/release/kilitler mutex $n 2>&1 | grep -E "^Mutex|futex$"; done
Mutex, 2 iş parçacığı: sayaç = 2000000
100.00    0.009229           7      1236       211 futex
Mutex, 4 iş parçacığı: sayaç = 4000000
100.00    0.041286           9      4279       797 futex
$ ./target/release/kilitler kilitte-bekleme 5
iş kilidin içinde : 8 × 20 iş, iş başına 5 ms,  809.5 ms
iş kilidin dışında: 8 × 20 iş, iş başına 5 ms,  101.5 ms
```

## 069-kilitlenme-ve-esgudum

- Video: `069-kilitlenme-ve-esgudum`
- Örnek yolu: `deneyler/mufredat-ornekleri/069-kilitlenme-ve-esgudum/`
- Kaynaklar: `kilitlenme/Cargo.toml`, `kilitlenme/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: iki iş parçacığının yazdırma sırası; döngülü sürümlerde kaç çalıştırmanın kilitlendiği (makineye ve yüke bağlı)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd kilitlenme
k cargo build -q
# 1) Ters sıralı kilitler: sonsuza dek bekler; 3 saniye sonra timeout öldürür (124)
k timeout 3 ./target/debug/kilitlenme
# 2) Kilitlenmiş süreç ne yapıyor? Her iş parçacığının çekirdekte beklediği yer (wchan), durumu ve işlemci süresi (utime, stime; saat tıkı)
ks './target/debug/kilitlenme > /dev/null & P=$!; sleep 1; for t in /proc/$P/task/*; do echo "bekliyor: $(cat $t/wchan)  durum, utime, stime: $(cut -d" " -f3,14,15 $t/stat)"; done; kill $P'
# 3) Tek bir kilit sırası: kilitlenme yok
k timeout 3 ./target/debug/kilitlenme sirali
# 4) Beklemesiz döngü, her iş parçacığı 100, 300 ve 1000 transfer: kilitlenme yalnız bazen olur (her biri 20 çalıştırma; 0 = bitti, 124 = kilitlendi)
ks 'for n in 100 300 1000; do echo "$n transfer:"; for i in $(seq 20); do timeout 2 ./target/debug/kilitlenme dongu $n > /dev/null; echo "  çıkış kodu $?"; done | sort | uniq -c; done'
# Kendin dene: aynı döngü, tek kilit sırasıyla (1000 transfer, 20 çalıştırma)
ks 'for i in $(seq 20); do timeout 2 ./target/debug/kilitlenme dongu-sirali 1000 > /dev/null; echo "  sıralı, çıkış kodu $?"; done | sort | uniq -c'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ timeout 3 ./target/debug/kilitlenme
1. iş parçacığı A'yı aldı, B'yi bekliyor
2. iş parçacığı ilk kilidi aldı, ikincisini bekliyor
[çıkış kodu: 124]
$ ./target/debug/kilitlenme > /dev/null & P=$!; sleep 1; for t in /proc/$P/task/*; do echo "bekliyor: $(cat $t/wchan)  durum, utime, stime: $(cut -d" " -f3,14,15 $t/stat)"; done; kill $P
bekliyor: __futex_wait  durum, utime, stime: S 0 0
bekliyor: __futex_wait  durum, utime, stime: S 0 0
bekliyor: __futex_wait  durum, utime, stime: S 0 0
$ timeout 3 ./target/debug/kilitlenme sirali
1. iş parçacığı A'yı aldı, B'yi bekliyor
1. iş parçacığı transferi yaptı (A = 99, B = 101)
2. iş parçacığı ilk kilidi aldı, ikincisini bekliyor
2. iş parçacığı transferi yaptı
bitti: A = 100, B = 100
$ for n in 100 300 1000; do echo "$n transfer:"; for i in $(seq 20); do timeout 2 ./target/debug/kilitlenme dongu $n > /dev/null; echo "  çıkış kodu $?"; done | sort | uniq -c; done
100 transfer:
     20   çıkış kodu 0
300 transfer:
     11   çıkış kodu 0
      9   çıkış kodu 124
1000 transfer:
     20   çıkış kodu 124
$ for i in $(seq 20); do timeout 2 ./target/debug/kilitlenme dongu-sirali 1000 > /dev/null; echo "  sıralı, çıkış kodu $?"; done | sort | uniq -c
     20   sıralı, çıkış kodu 0
```

## 070-kanallar-ve-mesajlasma

- Video: `070-kanallar-ve-mesajlasma`
- Örnek yolu: `deneyler/mufredat-ornekleri/070-kanallar-ve-mesajlasma/`
- Kaynaklar: `Cargo.toml`, `gonderince-tasinir/Cargo.toml`, `gonderince-tasinir/src/main.rs`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: milisaniye zaman damgaları ve aynı anlara düşen satırların sırası
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
# 1) Üç aşamalı boru hattı
k ./target/debug/kanallar boru-hatti
# 2) Kapasitesi 2 olan kanal: hızlı üretici, yavaş tüketici
k ./target/debug/kanallar 2
# Kendin dene: kapasite 0 (buluşma kanalı)
k ./target/debug/kanallar 0
# 3) Gönderilen değerin sahibi değişir
cd gonderince-tasinir && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/kanallar boru-hatti
1² = 1
2² = 4
3² = 9
4² = 16
5² = 25
$ ./target/debug/kanallar 2
sync_channel(2):
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
$ ./target/debug/kanallar 0
sync_channel(0):
[ 300 ms] yavaş tüketici 1 aldı
[ 300 ms] üretici 1 gönderdi
[ 400 ms] yavaş tüketici 2 aldı
[ 400 ms] üretici 2 gönderdi
[ 500 ms] yavaş tüketici 3 aldı
[ 500 ms] üretici 3 gönderdi
[ 600 ms] yavaş tüketici 4 aldı
[ 600 ms] üretici 4 gönderdi
[ 700 ms] yavaş tüketici 5 aldı
[ 700 ms] üretici 5 gönderdi
$ cargo build -q
error[E0382]: borrow of moved value: `olcumler`
 --> src/main.rs:9:16
  |
7 |     let olcumler = vec![1, 2, 3];
  |         -------- move occurs because `olcumler` has type `Vec<u32>`, which does not implement the `Copy` trait
8 |     tx.send(olcumler).unwrap(); // değer kanala taşındı: sahibi artık alıcı
  |             -------- value moved here
9 |     println!("{olcumler:?}"); // gönderdikten sonra kullanmaya çalışmak
  |                ^^^^^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
8 |     tx.send(olcumler.clone()).unwrap(); // değer kanala taşındı: sahibi artık alıcı
  |                     ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `gonderince_tasinir` (bin "gonderince_tasinir") due to 1 previous error
[çıkış kodu: 101]
```

## 071-atomikler-ve-bellek-siralamasi

- Video: `071-atomikler-ve-bellek-siralamasi`
- Örnek yolu: `deneyler/mufredat-ornekleri/071-atomikler-ve-bellek-siralamasi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: Relaxed, Release/Acquire ve fence(AcqRel) sayımları her çalıştırmada ve makinede değişir (SeqCst ve fence(SeqCst) sürümlerinde 0 olmalı); fonksiyon adresleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/siralama_testi
# Makine kodundaki fark: yalnız yazma komutu ve bariyer değişiyor
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_oku_gevsek>:"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_oku_edin_birak>:"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_oku_sirali>:"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_bariyer_oku>:"'
# Yalnız 2 mantıksal işlemcide: program (bekleme döngüsündeki yield sayesinde) biter, ama iki işçi ve ana iş parçacığı 2 işlemciyi paylaştığı için işçiler nadiren aynı anda koşar
k taskset -c 0,1 ./target/release/siralama_testi gevsek
# Kendin dene: SeqCst yerine daha zayıf bir bariyer (AcqRel)
k ./target/release/siralama_testi zayif-bariyer
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_zayif_bariyer_oku>:"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/siralama_testi
Relaxed                : 1000000 turda r1 = r2 = 0 sonucu 412797 kez görüldü
Release/Acquire        : 1000000 turda r1 = r2 = 0 sonucu 307631 kez görüldü
SeqCst                 : 1000000 turda r1 = r2 = 0 sonucu 0 kez görüldü
Relaxed + fence(SeqCst): 1000000 turda r1 = r2 = 0 sonucu 0 kez görüldü
$ objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_oku_gevsek>:"
0000000000018780 <siralama_testi::yaz_oku_gevsek>:
   18780:	mov    DWORD PTR [rdi],0x1
   18786:	mov    eax,DWORD PTR [rsi]
   18788:	ret
$ objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_oku_edin_birak>:"
00000000000187b0 <siralama_testi::yaz_oku_edin_birak>:
   187b0:	mov    DWORD PTR [rdi],0x1
   187b6:	mov    eax,DWORD PTR [rsi]
   187b8:	ret
$ objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_oku_sirali>:"
0000000000018790 <siralama_testi::yaz_oku_sirali>:
   18790:	mov    eax,0x1
   18795:	xchg   DWORD PTR [rdi],eax
   18797:	mov    eax,DWORD PTR [rsi]
   18799:	ret
$ objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A4 "<siralama_testi::yaz_bariyer_oku>:"
00000000000187a0 <siralama_testi::yaz_bariyer_oku>:
   187a0:	mov    DWORD PTR [rdi],0x1
   187a6:	lock or DWORD PTR [rsp-0x40],0x0
   187ac:	mov    eax,DWORD PTR [rsi]
   187ae:	ret
$ taskset -c 0,1 ./target/release/siralama_testi gevsek
Relaxed                : 1000000 turda r1 = r2 = 0 sonucu 0 kez görüldü
$ ./target/release/siralama_testi zayif-bariyer
Relaxed + fence(AcqRel): 1000000 turda r1 = r2 = 0 sonucu 275178 kez görüldü
$ objdump -d -M intel --no-show-raw-insn -C target/release/siralama_testi | grep -A3 "<siralama_testi::yaz_zayif_bariyer_oku>:"
00000000000187c0 <siralama_testi::yaz_zayif_bariyer_oku>:
   187c0:	mov    DWORD PTR [rdi],0x1
   187c6:	mov    eax,DWORD PTR [rsi]
   187c8:	ret
```

## 072-veri-paralelligi

- Video: `072-veri-paralelligi`
- Örnek yolu: `deneyler/mufredat-ornekleri/072-veri-paralelligi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `crossbeam-deque 0.8.8`, `crossbeam-epoch 0.9.21`, `crossbeam-utils 0.8.23`, `either 1.19.0`, `rayon 1.12.0`, `rayon-core 1.13.0`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler, adresler ve iş parçacığı başına düşen öğe sayıları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Önbellek satırı boyutu (çekirdeğin bildirdiği, bayt)
k cat /sys/devices/system/cpu/cpu0/cache/index0/coherency_line_size
k cargo build -q --release
k ./target/release/parallel
# Kendin dene: iki sayaç arasında 48 bayt dolgu (aynı satır) ve 56 bayt dolgu (ayrı satır)
k ./target/release/parallel dolgu
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cat /sys/devices/system/cpu/cpu0/cache/index0/coherency_line_size
64
$ cargo build -q --release
$ ./target/release/parallel
rayon iş parçacığı: 12
iş parçacığı başına işlenen öğe: [452079, 454844, 500544, 231035, 449272, 452214, 445881, 451753, 445568, 448855, 217285, 450670] (toplam 5000000)
sıralı  :   109.9 ms
paralel :    15.5 ms  (hızlanma 7.10×, sonuç aynı: true)
bitişik sayaçlar: a @ 0x7fff409b1b90 (satır 0x1fffd026c6e), b @ 0x7fff409b1b98 (satır 0x1fffd026c6e)
hizalı sayaçlar : a @ 0x7fff409b1a40 (satır 0x1fffd026c69), b @ 0x7fff409b1ac0 (satır 0x1fffd026c6b)
yanlış paylaşım (aynı satır):   942.2 ms
ayrı önbellek satırları     :    83.7 ms
$ ./target/release/parallel dolgu
dolgu 48 bayt: a @ 0x7ffc5bb07540 (satır 0x1fff16ec1d5), b @ 0x7ffc5bb07578 (satır 0x1fff16ec1d5) →  920.5 ms
dolgu 56 bayt: a @ 0x7ffc5bb075c0 (satır 0x1fff16ec1d7), b @ 0x7ffc5bb07600 (satır 0x1fff16ec1d8) →   86.0 ms
```

## 073-surecler

- Video: `073-surecler`
- Örnek yolu: `deneyler/mufredat-ornekleri/073-surecler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreç kimlikleri (pid), strace'teki adresler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/surecler
# PATH: programın arandığı dizinler, bu sırayla
ks 'echo "$PATH"'
# C kütüphanesinin sürümü (posix_spawn'ı o uygular)
ks 'ldd --version | head -n 1'
# Çekirdek düzeyinde: yeni süreç (clone3), yeni program (execve), bekleme (wait4); strace'in kaydı programın çıktısına karışmasın diye dosyaya yazılır (-o), ek iletileri susturulur (-qq)
ks 'strace -f -qq -e trace=clone,clone3,vfork,execve,wait4,exit_group -o /tmp/iz.txt ./target/debug/surecler > /dev/null; cat /tmp/iz.txt'
# Zombi süreç: bitmiş ama çıkış durumu henüz alınmamış alt süreç
k ./target/debug/surecler zombi
# Kendin dene: PATH yalnız /usr/bin olunca execve ilk denemede bulur
ks 'PATH=/usr/bin strace -f -qq -e trace=execve -o /tmp/iz2.txt ./target/debug/surecler > /dev/null; grep -E "execve\(\"/usr/bin/(sh|uname)\"" /tmp/iz2.txt'
# Kendin dene: wait() hiç çağrılmazsa
k ./target/debug/surecler zombi-beklemesiz
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/surecler
üst süreç: pid 16044
alt süreç: pid 16045, üst süreci 16044
alt sürecin çıkış durumu: exit status: 3 (code = Some(3))
uname -sr çıktısı yakalandı: "Linux 7.2.15-1-omarchy"
başlatılamadı: No such file or directory (os error 2)
$ echo "$PATH"
/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
$ ldd --version | head -n 1
ldd (Debian GLIBC 2.41-12+deb13u5) 2.41
$ strace -f -qq -e trace=clone,clone3,vfork,execve,wait4,exit_group -o /tmp/iz.txt ./target/debug/surecler > /dev/null; cat /tmp/iz.txt
16056 execve("./target/debug/surecler", ["./target/debug/surecler"], 0x7ffe50a259a0 /* 15 vars */) = 0
16056 clone3({flags=CLONE_VM|CLONE_VFORK|CLONE_CLEAR_SIGHAND, exit_signal=SIGCHLD, stack=0x7fa7869a2000, stack_size=0x9000}, 88 <unfinished ...>
16057 execve("/usr/local/cargo/bin/sh", ["sh", "-c", "echo \"alt s\303\274re\303\247: pid $$, \303\274st "...], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16057 execve("/usr/local/sbin/sh", ["sh", "-c", "echo \"alt s\303\274re\303\247: pid $$, \303\274st "...], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16057 execve("/usr/local/bin/sh", ["sh", "-c", "echo \"alt s\303\274re\303\247: pid $$, \303\274st "...], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16057 execve("/usr/sbin/sh", ["sh", "-c", "echo \"alt s\303\274re\303\247: pid $$, \303\274st "...], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16057 execve("/usr/bin/sh", ["sh", "-c", "echo \"alt s\303\274re\303\247: pid $$, \303\274st "...], 0x7fff23857c38 /* 15 vars */ <unfinished ...>
16056 <... clone3 resumed>)             = 16057
16056 wait4(16057,  <unfinished ...>
16057 <... execve resumed>)             = 0
16057 exit_group(3)                     = ?
16056 <... wait4 resumed>[{WIFEXITED(s) && WEXITSTATUS(s) == 3}], 0, NULL) = 16057
16056 --- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=16057, si_uid=0, si_status=3, si_utime=0, si_stime=0} ---
16056 clone3({flags=CLONE_VM|CLONE_VFORK|CLONE_CLEAR_SIGHAND, exit_signal=SIGCHLD, stack=0x7fa7869a2000, stack_size=0x9000}, 88 <unfinished ...>
16058 execve("/usr/local/cargo/bin/uname", ["uname", "-sr"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16058 execve("/usr/local/sbin/uname", ["uname", "-sr"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16058 execve("/usr/local/bin/uname", ["uname", "-sr"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16058 execve("/usr/sbin/uname", ["uname", "-sr"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16058 execve("/usr/bin/uname", ["uname", "-sr"], 0x7fff23857c38 /* 15 vars */ <unfinished ...>
16056 <... clone3 resumed>)             = 16058
16058 <... execve resumed>)             = 0
16058 exit_group(0)                     = ?
16056 wait4(16058, [{WIFEXITED(s) && WEXITSTATUS(s) == 0}], 0, NULL) = 16058
16056 --- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=16058, si_uid=0, si_status=0, si_utime=0, si_stime=0} ---
16056 clone3({flags=CLONE_VM|CLONE_VFORK|CLONE_CLEAR_SIGHAND, exit_signal=SIGCHLD, stack=0x7fa7869a2000, stack_size=0x9000}, 88 <unfinished ...>
16059 execve("/usr/local/cargo/bin/olmayan-program", ["olmayan-program"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16059 execve("/usr/local/sbin/olmayan-program", ["olmayan-program"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16059 execve("/usr/local/bin/olmayan-program", ["olmayan-program"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16059 execve("/usr/sbin/olmayan-program", ["olmayan-program"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16059 execve("/usr/bin/olmayan-program", ["olmayan-program"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16059 execve("/sbin/olmayan-program", ["olmayan-program"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16059 execve("/bin/olmayan-program", ["olmayan-program"], 0x7fff23857c38 /* 15 vars */) = -1 ENOENT (No such file or directory)
16059 exit_group(127)                   = ?
16056 <... clone3 resumed>)             = 16059
16056 --- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=16059, si_uid=0, si_status=127, si_utime=0, si_stime=0} ---
16056 exit_group(0)                     = ?
$ ./target/debug/surecler zombi
alt süreç 16061 bitti, 200 ms sonra henüz beklenmedi: durum = Z
wait() → exit status: 0; şimdi durum = yok (/proc'ta kaydı kalmadı)
$ PATH=/usr/bin strace -f -qq -e trace=execve -o /tmp/iz2.txt ./target/debug/surecler > /dev/null; grep -E "execve\(\"/usr/bin/(sh|uname)\"" /tmp/iz2.txt
16067 execve("/usr/bin/sh", ["sh", "-c", "echo \"alt s\303\274re\303\247: pid $$, \303\274st "...], 0x7fff90cc79d8 /* 15 vars */) = 0
16068 execve("/usr/bin/uname", ["uname", "-sr"], 0x7fff90cc79d8 /* 15 vars */) = 0
$ ./target/debug/surecler zombi-beklemesiz
beklenmeyen alt süreç 16071, başlatıldıktan 200 ms sonra: durum = Z
beklenmeyen alt süreç 16071, başlatıldıktan 1200 ms sonra: durum = Z
```

## 074-dosyalar-ve-tamponlama

- Video: `074-dosyalar-ve-tamponlama`
- Örnek yolu: `deneyler/mufredat-ornekleri/074-dosyalar-ve-tamponlama/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler; dosya tanımlayıcı numarası
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
# Tamponsuz yazmanın ilk sistem çağrıları: dosya açılır (fd 3), her satır parça parça yazılır
ks 'strace -e trace=openat,write ./target/release/tamponlama tamponsuz 2>&1 | grep -A6 "tamponsuz.txt"'
# 100 000 satır: kaç write çağrısı?
ks 'strace -c -e trace=write ./target/release/tamponlama tamponsuz 2>&1 | tail -n 4'
ks 'strace -c -e trace=write ./target/release/tamponlama tamponlu 2>&1 | tail -n 4'
k hyperfine -N --warmup 2 --runs 10 --style basic "./target/release/tamponlama tamponsuz" "./target/release/tamponlama tamponlu"
ks 'wc -c /tmp/tamponsuz.txt /tmp/tamponlu.txt && cmp /tmp/tamponsuz.txt /tmp/tamponlu.txt && echo "iki dosya aynı"'
# Kaynak (nightly rust-src): BufWriter/BufReader varsayılan tampon boyutu
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A3 "pub const DEFAULT_BUF_SIZE" $S/alloc/src/io/read.rs'
# Kendin dene: 64 KiB'lık tampon
ks 'strace -c -e trace=write ./target/release/tamponlama tamponlu64 2>&1 | tail -n 4'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ strace -e trace=openat,write ./target/release/tamponlama tamponsuz 2>&1 | grep -A6 "tamponsuz.txt"
openat(AT_FDCWD, "/tmp/tamponsuz.txt", O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0666) = 3
write(3, "sat\304\261r ", 7)            = 7
write(3, "0", 1)                        = 1
write(3, "\n", 1)                       = 1
write(3, "sat\304\261r ", 7)            = 7
write(3, "1", 1)                        = 1
write(3, "\n", 1)                       = 1
$ strace -c -e trace=write ./target/release/tamponlama tamponsuz 2>&1 | tail -n 4
------ ----------- ----------- --------- --------- ----------------
100.00    1.547800           5    300000           write
------ ----------- ----------- --------- --------- ----------------
100.00    1.547800           5    300000           total
$ strace -c -e trace=write ./target/release/tamponlama tamponlu 2>&1 | tail -n 4
------ ----------- ----------- --------- --------- ----------------
100.00    0.000577           3       158           write
------ ----------- ----------- --------- --------- ----------------
100.00    0.000577           3       158           total
$ hyperfine -N --warmup 2 --runs 10 --style basic "./target/release/tamponlama tamponsuz" "./target/release/tamponlama tamponlu"
Benchmark 1: ./target/release/tamponlama tamponsuz
  Time (mean ± σ):     829.1 ms ±  12.4 ms    [User: 83.1 ms, System: 739.7 ms]
  Range (min … max):   802.6 ms … 848.2 ms    10 runs
 
Benchmark 2: ./target/release/tamponlama tamponlu
  Time (mean ± σ):       5.8 ms ±   0.1 ms    [User: 2.7 ms, System: 1.6 ms]
  Range (min … max):     5.7 ms …   6.1 ms    10 runs
 
Summary
  ./target/release/tamponlama tamponlu ran
  142.11 ± 4.01 times faster than ./target/release/tamponlama tamponsuz
$ wc -c /tmp/tamponsuz.txt /tmp/tamponlu.txt && cmp /tmp/tamponsuz.txt /tmp/tamponlu.txt && echo "iki dosya aynı"
1288890 /tmp/tamponsuz.txt
1288890 /tmp/tamponlu.txt
2577780 total
iki dosya aynı
$ S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A3 "pub const DEFAULT_BUF_SIZE" $S/alloc/src/io/read.rs
787:pub const DEFAULT_BUF_SIZE: usize = cfg_select! {
788-    target_os = "espidf" => 512,
789-    _ => 8 * 1024,
790-};
$ strace -c -e trace=write ./target/release/tamponlama tamponlu64 2>&1 | tail -n 4
------ ----------- ----------- --------- --------- ----------------
100.00    0.000333          16        20           write
------ ----------- ----------- --------- --------- ----------------
100.00    0.000333          16        20           total
```

## 075-dayaniklilik-ve-fsync

- Video: `075-dayaniklilik-ve-fsync`
- Örnek yolu: `deneyler/mufredat-ornekleri/075-dayaniklilik-ve-fsync/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler diske ve dosya sistemine bağlı; inode numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p calisma
k cargo build -q --release
# Ana makinenin diskine bağlı dizin (btrfs)
k ./target/release/dayaniklilik olc calisma
# sync_data() hangi sistem çağrısı, kaç kez? (her kayıtta 200 + iki döngünün sonundaki 2)
ks 'strace -c -e trace=fsync,fdatasync ./target/release/dayaniklilik olc calisma 2>&1 | grep -E "fdatasync$|^her|^sonda"'
# Konteynerin kendi kök dosya sistemi (/tmp burada)
k ./target/release/dayaniklilik olc /tmp
# Neden bu kadar hızlı? Kök dizinin ve /work'ün bağlama bilgileri
ks 'grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"'
ks 'grep " /work " /proc/mounts | cut -d" " -f1-3'
# fsync çağrılmazsa: çekirdek kirli sayfaları kendisi yazar; bu kadar santisaniyeden eski kirli veri geri yazmaya uygun olur
k cat /proc/sys/vm/dirty_expire_centisecs
# Atomik değiştirme: önce eski sürüm (1), sonra kalıp; dosya adı aynı kalır, dosyanın kendisi (inode) değişir
ks 'echo "{\"surum\": 1}" > calisma/ayar.json && cat calisma/ayar.json && stat -c "inode %i" calisma/ayar.json'
ks 'strace -e trace=openat,write,fsync,fdatasync,rename,renameat2 ./target/release/dayaniklilik atomik calisma 2>&1 | grep -v "/lib\|/etc\|/proc\|/sys"'
ks 'cat calisma/ayar.json && stat -c "inode %i" calisma/ayar.json'
# Kendin dene: sync_data yerine sync_all (fsync)
ks 'strace -c -e trace=fsync,fdatasync ./target/release/dayaniklilik olc-tum calisma 2>&1 | grep -E "fsync$|^her|^sonda"'
k ./target/release/dayaniklilik olc-tum calisma
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/dayaniklilik olc calisma
her kayıtta fsync : 200 kayıt   170.81 ms (0.854 ms/kayıt)
sonda tek fsync   : 200 kayıt     2.90 ms (0.015 ms/kayıt)
$ strace -c -e trace=fsync,fdatasync ./target/release/dayaniklilik olc calisma 2>&1 | grep -E "fdatasync$|^her|^sonda"
her kayıtta fsync : 200 kayıt   179.44 ms (0.897 ms/kayıt)
sonda tek fsync   : 200 kayıt    12.04 ms (0.060 ms/kayıt)
100.00    0.023037         114       202           fdatasync
$ ./target/release/dayaniklilik olc /tmp
her kayıtta fsync : 200 kayıt     1.89 ms (0.009 ms/kayıt)
sonda tek fsync   : 200 kayıt     1.69 ms (0.008 ms/kayıt)
$ grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"
overlay
fsync=volatile
$ grep " /work " /proc/mounts | cut -d" " -f1-3
/dev/mapper/root /work btrfs
$ cat /proc/sys/vm/dirty_expire_centisecs
3000
$ echo "{\"surum\": 1}" > calisma/ayar.json && cat calisma/ayar.json && stat -c "inode %i" calisma/ayar.json
{"surum": 1}
inode 80371611
$ strace -e trace=openat,write,fsync,fdatasync,rename,renameat2 ./target/release/dayaniklilik atomik calisma 2>&1 | grep -v "/lib\|/etc\|/proc\|/sys"
openat(AT_FDCWD, "calisma/ayar.json.gecici", O_WRONLY|O_CREAT|O_TRUNC|O_CLOEXEC, 0666) = 3
write(3, "{\"surum\": 2}\n", 13)        = 13
fsync(3)                                = 0
rename("calisma/ayar.json.gecici", "calisma/ayar.json") = 0
openat(AT_FDCWD, "calisma", O_RDONLY|O_CLOEXEC) = 4
fsync(4)                                = 0
+++ exited with 0 +++
$ cat calisma/ayar.json && stat -c "inode %i" calisma/ayar.json
{"surum": 2}
inode 80371612
$ strace -c -e trace=fsync,fdatasync ./target/release/dayaniklilik olc-tum calisma 2>&1 | grep -E "fsync$|^her|^sonda"
her kayıtta fsync : 200 kayıt   190.01 ms (0.950 ms/kayıt)
sonda tek fsync   : 200 kayıt    14.59 ms (0.073 ms/kayıt)
100.00    0.023794         117       202           fsync
$ ./target/release/dayaniklilik olc-tum calisma
her kayıtta fsync : 200 kayıt   164.38 ms (0.822 ms/kayıt)
sonda tek fsync   : 200 kayıt     3.00 ms (0.015 ms/kayıt)
```

## 076-borular-ve-sinyaller

- Video: `076-borular-ve-sinyaller`
- Örnek yolu: `deneyler/mufredat-ornekleri/076-borular-ve-sinyaller/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `errno 0.3.15`, `libc 0.2.191`, `linux-raw-sys 0.12.1`, `rustix 1.1.6`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: iş parçacığı kimliği; sa_restorer adresi; ilk kapasite ölçümü (varsayılan 65 536 bayt; çalıştıran kullanıcının bütün borularının sayfa toplamı pipe-user-pages-soft sınırını aşmışsa 8192 bayt; rootless Podman'da bu toplama ana makinedeki süreçlerimiz de girer)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/borular
# Rust çalışma zamanı, main'den önce SIGPIPE'ı "yok say" olarak ayarlar
ks 'strace -e trace=rt_sigaction ./target/debug/borular kapasite 2>&1 | grep SIGPIPE'
# Boru tamponunun kapasitesi: program önce çekirdeğe sorar (F_GETPIPE_SZ), sonra ölçer; okuma ucu kapanınca yazma
k ./target/debug/borular kapasite
# Yumuşak sınırı aşmış bir kullanıcı (pipe(7)). Kendi kullanıcımızın sayacını doldurmak ana makinedeki
# programlarımızı da etkilerdi; bu yüzden konteynerde başka bir kimlikle (uid 4242) çalışırız.
# uid_map: konteynerdeki 0 ana makinede bizim kullanıcımızdır, 1 ve sonrası ayrı bir kimlik aralığına eşlenir.
k cat /proc/self/uid_map
k setpriv --reuid=4242 --regid=4242 --clear-groups ./target/debug/borular sinirda
# Okuyan taraf boruyu erken kapatırsa: yazma EPIPE hatası döner, println! paniğe dönüşür
ks './target/debug/borular cok-yaz | head -n 2'
# Kendin dene: hatayı yakalayıp sessizce bitirmek
ks './target/debug/borular dikkatli-yaz | head -n 2; echo "yazanın çıkış kodu: ${PIPESTATUS[0]}"'
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
$ strace -e trace=rt_sigaction ./target/debug/borular kapasite 2>&1 | grep SIGPIPE
rt_sigaction(SIGPIPE, {sa_handler=SIG_IGN, sa_mask=[PIPE], sa_flags=SA_RESTORER|SA_RESTART, sa_restorer=0x7f474320ed70}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
$ ./target/debug/borular kapasite
yumuşak sınır (pipe-user-pages-soft): 16384 sayfa
borunun kapasitesi (F_GETPIPE_SZ): 8192 bayt
kimse okumazken yazılan: 8192 bayt (yazan şimdi bekliyor)
4096 bayt okununca     : 12288 bayt
okuma ucu kapanınca yazma: Broken pipe (os error 32) (BrokenPipe)
$ cat /proc/self/uid_map
         0       1000          1
         1     100000      65536
$ setpriv --reuid=4242 --regid=4242 --clear-groups ./target/debug/borular sinirda
yumuşak sınır (pipe-user-pages-soft): 16384 sayfa
tutulan borular: 64 tane, toplam 16384 sayfa
yeni borunun kapasitesi (F_GETPIPE_SZ): 8192 bayt
65536 bayt isteği (F_SETPIPE_SZ): Operation not permitted (os error 1)
kimse okumazken yazılan: 8192 bayt (yazan şimdi bekliyor)
4096 bayt okununca     : 12288 bayt
okuma ucu kapanınca yazma: Broken pipe (os error 32) (BrokenPipe)
$ ./target/debug/borular cok-yaz | head -n 2
satır 0
satır 1

thread 'main' (93) panicked at /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/io/stdio.rs:1209:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
$ ./target/debug/borular dikkatli-yaz | head -n 2; echo "yazanın çıkış kodu: ${PIPESTATUS[0]}"
satır 0
satır 1
yazanın çıkış kodu: 0
```

## 077-ag-katmanlari-ve-soketler

- Video: `077-ag-katmanlari-ve-soketler`
- Örnek yolu: `deneyler/mufredat-ornekleri/077-ag-katmanlari-ve-soketler/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: port numaraları (onaltılık /proc/net/tcp satırlarında da), dosya tanımlayıcı numaraları, milisaniyeler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/soketler
# Çekirdeğin istemci tarafına geçici port seçtiği aralık
k cat /proc/sys/net/ipv4/ip_local_port_range
# accept çağrılmadan önce (120. ms) çekirdeğin TCP tablosu: 0A = LISTEN, 01 = ESTABLISHED; tx_queue:rx_queue onaltılık bayt sayısı
ks './target/debug/soketler > /dev/null & sleep 0.12; awk "NR==1 || \$4==\"01\" || \$4==\"0A\"" /proc/net/tcp | cut -c1-60; wait'
# Aynı programın ağ sistem çağrıları
ks 'strace -f -e trace=socket,bind,listen,accept4,connect,sendto,recvfrom ./target/debug/soketler 2>&1 | grep -E "socket\(|bind\(|listen\(|accept4\(|connect\(|sendto\(|recvfrom\([0-9]+, \""'
# Kendin dene: accept 3 saniye geciktirilince
k ./target/debug/soketler 3000
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/soketler
sunucu dinliyor: 127.0.0.1:40555
[   0 ms] istemci bağlandı: 127.0.0.1:56676 → 127.0.0.1:40555
[   0 ms] istemci veriyi gönderdi
[ 300 ms] sunucu accept çağırıyor
sunucu: 127.0.0.1:56676 bağlandı
[ 300 ms] istemci aldı: "MERHABA AĞ"
$ cat /proc/sys/net/ipv4/ip_local_port_range
32768	60999
$ ./target/debug/soketler > /dev/null & sleep 0.12; awk "NR==1 || \$4==\"01\" || \$4==\"0A\"" /proc/net/tcp | cut -c1-60; wait
  sl  local_address rem_address   st tx_queue rx_queue tr tm
   0: 0100007F:88D3 00000000:0000 0A 00000000:00000001 00:00
   2: 0100007F:90B8 0100007F:88D3 01 00000000:00000000 00:00
   3: 0100007F:88D3 0100007F:90B8 01 00000000:0000000B 00:00
$ strace -f -e trace=socket,bind,listen,accept4,connect,sendto,recvfrom ./target/debug/soketler 2>&1 | grep -E "socket\(|bind\(|listen\(|accept4\(|connect\(|sendto\(|recvfrom\([0-9]+, \""
socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_IP) = 3
bind(3, {sa_family=AF_INET, sin_port=htons(0), sin_addr=inet_addr("127.0.0.1")}, 16) = 0
listen(3, 128)                          = 0
[pid 16555] socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_IP) = 4
[pid 16555] connect(4, {sa_family=AF_INET, sin_port=htons(38035), sin_addr=inet_addr("127.0.0.1")}, 16) = 0
[pid 16555] sendto(4, "merhaba a\304\237", 11, MSG_NOSIGNAL, NULL, 0) = 11
[pid 16556] accept4(3, {sa_family=AF_INET, sin_port=htons(33898), sin_addr=inet_addr("127.0.0.1")}, [128 => 16], SOCK_CLOEXEC) = 5
[pid 16556] recvfrom(5, "merhaba a\304\237", 64, 0, NULL, NULL) = 11
[pid 16556] sendto(5, "MERHABA A\304\236", 11, MSG_NOSIGNAL, NULL, 0 <unfinished ...>
[pid 16555] recvfrom(4, "", 32, 0, NULL, NULL) = 0
$ ./target/debug/soketler 3000
sunucu dinliyor: 127.0.0.1:42715
[   0 ms] istemci bağlandı: 127.0.0.1:55132 → 127.0.0.1:42715
[   0 ms] istemci veriyi gönderdi
[3000 ms] sunucu accept çağırıyor
sunucu: 127.0.0.1:55132 bağlandı
[3000 ms] istemci aldı: "MERHABA AĞ"
```

## 078-tcp-akis-dogasi

- Video: `078-tcp-akis-dogasi`
- Örnek yolu: `deneyler/mufredat-ornekleri/078-tcp-akis-dogasi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler; dosya tanımlayıcı numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q --release
# set_nodelay hangi sistem çağrısına dönüşüyor?
ks 'strace -f -e trace=setsockopt ./target/release/tcp_akis 2>&1 | grep TCP_NODELAY'
# Kendin dene: NODELAY'i istemci yerine sunucuda açmak
k ./target/release/tcp_akis sunucu-nodelay
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
sunucu tek read ile 13 bayt aldı: "bir|iki|üç|"
iki küçük yazma, TCP_NODELAY = false    : 20 istek  780.9 ms (istek başına 39.04 ms)
iki küçük yazma, TCP_NODELAY = true     : 20 istek    0.4 ms (istek başına 0.02 ms)
tek yazma,       TCP_NODELAY = false    : 20 istek    0.3 ms (istek başına 0.01 ms)
$ strace -f -e trace=setsockopt ./target/release/tcp_akis 2>&1 | grep TCP_NODELAY
[pid 16623] setsockopt(5, SOL_TCP, TCP_NODELAY, [0], 4) = 0
[pid 16625] setsockopt(6, SOL_TCP, TCP_NODELAY, [0], 4) = 0
iki küçük yazma, TCP_NODELAY = false    : 20 istek  781.1 ms (istek başına 39.06 ms)
[pid 16623] setsockopt(5, SOL_TCP, TCP_NODELAY, [1], 4) = 0
[pid 16626] setsockopt(6, SOL_TCP, TCP_NODELAY, [0], 4) = 0
iki küçük yazma, TCP_NODELAY = true     : 20 istek    1.4 ms (istek başına 0.07 ms)
[pid 16623] setsockopt(5, SOL_TCP, TCP_NODELAY, [0], 4) = 0
[pid 16627] setsockopt(6, SOL_TCP, TCP_NODELAY, [0], 4) = 0
tek yazma,       TCP_NODELAY = false    : 20 istek    0.9 ms (istek başına 0.04 ms)
$ ./target/release/tcp_akis sunucu-nodelay
iki küçük yazma, yalnız sunucuda NODELAY: 20 istek  781.9 ms (istek başına 39.09 ms)
```

## 079-udp-ve-dns

- Video: `079-udp-ve-dns`
- Örnek yolu: `deneyler/mufredat-ornekleri/079-udp-ve-dns/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: kaynak port numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/udp_dns
# Ad çözümleme: kaynağı /etc/hosts; hangi sırayla bakılacağını /etc/nsswitch.conf söyler
k ./target/debug/udp_dns coz
ks 'grep -E "^hosts:" /etc/nsswitch.conf'
ks 'grep -v "^#" /etc/hosts | grep localhost'
# Çözümleme sırasında açılan dosyalar
ks 'strace -e trace=openat ./target/debug/udp_dns coz 2>&1 | grep -E "nsswitch|hosts|resolv"'
# Kendin dene: /etc/hosts'a yeni bir ad eklemek (yalnız bu geçici konteynerde); ağ çağrılarına da bakalım (53 numaralı porta gidilmemeli)
ks 'echo "127.0.0.1 laboratuvar.test" >> /etc/hosts && strace -e trace=openat,connect,sendto ./target/debug/udp_dns coz laboratuvar.test 2>&1 | grep -E "nsswitch|hosts|resolv|connect|sendto|laboratuvar"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/udp_dns
3 bayt datagram (127.0.0.1:49181 kaynaklı): "bir"
3 bayt datagram (127.0.0.1:49181 kaynaklı): "iki"
4 bayt datagram (127.0.0.1:49181 kaynaklı): "üç"
dinleyeni olmayan 127.0.0.1:45336 portuna send: Ok(13)
sonraki recv: Connection refused (os error 111) (ConnectionRefused)
$ ./target/debug/udp_dns coz
localhost:80 → [[::1]:80, 127.0.0.1:80]
$ grep -E "^hosts:" /etc/nsswitch.conf
hosts:          files dns
$ grep -v "^#" /etc/hosts | grep localhost
127.0.0.1	localhost
::1	localhost
$ strace -e trace=openat ./target/debug/udp_dns coz 2>&1 | grep -E "nsswitch|hosts|resolv"
openat(AT_FDCWD, "/etc/nsswitch.conf", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/etc/resolv.conf", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/etc/hosts", O_RDONLY|O_CLOEXEC) = 3
$ echo "127.0.0.1 laboratuvar.test" >> /etc/hosts && strace -e trace=openat,connect,sendto ./target/debug/udp_dns coz laboratuvar.test 2>&1 | grep -E "nsswitch|hosts|resolv|connect|sendto|laboratuvar"
connect(3, {sa_family=AF_UNIX, sun_path="/var/run/nscd/socket"}, 110) = -1 ENOENT (No such file or directory)
connect(3, {sa_family=AF_UNIX, sun_path="/var/run/nscd/socket"}, 110) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/etc/nsswitch.conf", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/etc/resolv.conf", O_RDONLY|O_CLOEXEC) = 3
openat(AT_FDCWD, "/etc/hosts", O_RDONLY|O_CLOEXEC) = 3
laboratuvar.test:80 → [127.0.0.1:80]
```

## 080-http-elle

- Video: `080-http-elle`
- Örnek yolu: `deneyler/mufredat-ornekleri/080-http-elle/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
./target/debug/http_elle 2 2> calisma_sunucu.log &
sleep 0.5
# 1) Elle yazılmış ham istek; yanıtın baytları od -c ile (\r ve \n görünür)
ks './target/debug/http_elle istemci | od -c | head -n 12'
# 2) curl ile: > giden, < gelen satırlar
ks 'curl -sv http://127.0.0.1:8099/ 2>&1 | grep -E "^[<>] |^Merhaba"'
wait
k cat calisma_sunucu.log
rm -f calisma_sunucu.log
# Kendin dene: Content-Length 15 yerine 10
./target/debug/http_elle 1 10 2> /dev/null &
sleep 0.5
ks 'curl -s http://127.0.0.1:8099/; echo " ← curl çıktısının sonu"'
wait
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/http_elle istemci | od -c | head -n 12
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
$ curl -s http://127.0.0.1:8099/; echo " ← curl çıktısının sonu"
Merhaba, H ← curl çıktısının sonu
```

## 081-protokol-tasarimi

- Video: `081-protokol-tasarimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/081-protokol-tasarimi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/cerceve
# Kendin dene: uzunluk alanı little-endian yazılır, big-endian okunur
k ./target/debug/cerceve kendin-dene
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/cerceve
tel üzerindeki baytlar: [00, 00, 00, 07, 6d, 65, 72, 68, 61, 62, 61]
saf sunucu tek read'de 48 bayt aldı (3 mesaj yapışık geldi)
çerçeveli sunucu mesaj aldı: "GET anahtar"
çerçeveli sunucu mesaj aldı: "SET anahtar 42"
çerçeveli sunucu mesaj aldı: "DEL anahtar"
saf sunucu tek read'de 7 bayt aldı (mesajın yalnız başı)
çerçeveli sunucu kalanı bekledi, mesajı tam aldı: "GET anahtar"
dev uzunluklu çerçeve: "çerçeve çok büyük: 4294967295 bayt"
$ ./target/debug/cerceve kendin-dene
tel üzerindeki baytlar: [07, 00, 00, 00, 6d, 65, 72, 68, 61, 62, 61]
big-endian okuyan alıcının gördüğü uzunluk: 117440512
çerçeve okuma: "çerçeve çok büyük: 117440512 bayt"
```

## 082-serilestirme

- Video: `082-serilestirme`
- Örnek yolu: `deneyler/mufredat-ornekleri/082-serilestirme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `itoa 1.0.19`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.10`, `unicode-ident 1.0.27`, `zmij 1.0.23`
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/serilestirme
# Kendin dene: varsayılansız yeni alan, Option'lı yeni alan, alanları ters sırayla çözmek
k ./target/debug/serilestirme kendin-dene
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/serilestirme
JSON  (59 bayt): {"sensor":7,"sicaklik":21.5,"zaman":1763640000,"birim":"C"}
ikili (16 bayt): [07, 00, 00, 00, ac, 41, c0, 02, 1f, 69, 00, 00, 00, 00, 01, 43]
JSON'dan geri okundu, aynı mı: true
ikiliden geri okundu, aynı mı: true
eski kayıt okundu: Olcum { sensor: 3, sicaklik: 19.0, zaman: 1700000000, birim: "" }
"invalid type: string \"yedi\", expected u16 at line 1 column 16"
ilk 10 baytı gelen ikili kayıt: Err("zaman için 8 bayt gerekli, 4 bayt kaldı")
$ ./target/debug/serilestirme kendin-dene
varsayılansız yeni alan: "missing field `konum` at line 1 column 47"
Option'lı yeni alan    : OlcumKonumSecimli { sensor: 3, sicaklik: 19.0, zaman: 1700000000, konum: None }
ters sırayla çözülen: sicaklik = 1e-44, sensor = 16812
```

## 083-neden-async

- Video: `083-neden-async`
- Örnek yolu: `deneyler/mufredat-ornekleri/083-neden-async/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler ve bellek ölçümleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Bu konteynerde açılabilecek en fazla süreç + iş parçacığı sayısı (cgroup sınırı)
k cat /sys/fs/cgroup/pids.max
k cargo build -q --release
k ./target/release/neden_async is-parcacigi 1000
k ./target/release/neden_async gorev 1000
k ./target/release/neden_async gorev 100000
# Sanal bellek nereden geliyor? C kütüphanesinin iş parçacığı başına bellek alanlarını (arena) teke indir: geriye yığınlar kalır
ks 'MALLOC_ARENA_MAX=1 ./target/release/neden_async is-parcacigi 1000'
# Kendin dene: iş parçacığı yığınını 64 KiB'a indir (arenalar teke indirilmiş olarak ve tek başına)
ks 'MALLOC_ARENA_MAX=1 RUST_MIN_STACK=65536 ./target/release/neden_async is-parcacigi 1000'
ks 'RUST_MIN_STACK=65536 ./target/release/neden_async is-parcacigi 1000'
# Sınıra çarpmak: 5000 iş parçacığı açılamaz
ks './target/release/neden_async is-parcacigi 5000 2>&1 | grep -E "panicked|failed|Resource" | head -n 3'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cat /sys/fs/cgroup/pids.max
2048
$ cargo build -q --release
$ ./target/release/neden_async is-parcacigi 1000
1000 iş parçacığı : başlatma   19.9 ms, RSS 12716 kB, sanal 8294860 kB
    toplam süre 1.02 s
$ ./target/release/neden_async gorev 1000
1000 async görev   : başlatma    0.9 ms, RSS 3588 kB, sanal 816016 kB
    toplam süre 1.00 s
$ ./target/release/neden_async gorev 100000
100000 async görev   : başlatma   34.9 ms, RSS 41668 kB, sanal 853892 kB
    toplam süre 1.04 s
$ MALLOC_ARENA_MAX=1 ./target/release/neden_async is-parcacigi 1000
1000 iş parçacığı : başlatma   21.4 ms, RSS 12112 kB, sanal 2069864 kB
    toplam süre 1.02 s
$ MALLOC_ARENA_MAX=1 RUST_MIN_STACK=65536 ./target/release/neden_async is-parcacigi 1000
1000 iş parçacığı : başlatma   19.7 ms, RSS 12148 kB, sanal 85864 kB
    toplam süre 1.02 s
$ RUST_MIN_STACK=65536 ./target/release/neden_async is-parcacigi 1000
1000 iş parçacığı : başlatma   19.9 ms, RSS 12540 kB, sanal 6310860 kB
    toplam süre 1.02 s
$ ./target/release/neden_async is-parcacigi 5000 2>&1 | grep -E "panicked|failed|Resource" | head -n 3
thread 'main' (21862) panicked at /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/thread/functions.rs:131:29:
failed to spawn thread: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }
```

## 084-epoll-ve-olay-dongusu

- Video: `084-epoll-ve-olay-dongusu`
- Örnek yolu: `deneyler/mufredat-ornekleri/084-epoll-ve-olay-dongusu/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: dosya tanımlayıcı numaraları, port numaraları, data= alanındaki adresler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
# std ile engellemeyen soket (ve aynı çağrının çekirdekten dönen makbuzu)
k ./target/debug/olay_dongusu std
ks 'strace -e trace=accept4 ./target/debug/olay_dongusu std 2>&1 | grep accept4'
k ./target/debug/olay_dongusu
# Olay döngüsünün çekirdekle konuşması: epoll (fcntl satırı: epoll tanımlayıcısının kopyası)
ks 'strace -f -e trace=epoll_create1,epoll_ctl,epoll_wait,epoll_pwait,epoll_pwait2,accept4,connect,fcntl ./target/debug/olay_dongusu 2>&1 | grep -v "^yankı" | grep -vE "fcntl\([0-9]+, F_(GETFL|SETFL|GETFD|SETFD)"'
# Kendin dene: aynı accept engelleyen sokette (bağlantı gelmediği için bekler; 2 saniye sonra timeout)
k timeout 2 ./target/debug/olay_dongusu std-engelleyen
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/olay_dongusu std
engellemeyen accept: Resource temporarily unavailable (os error 11) (WouldBlock)
$ strace -e trace=accept4 ./target/debug/olay_dongusu std 2>&1 | grep accept4
accept4(3, 0x7ffce217fd08, [128], SOCK_CLOEXEC) = -1 EAGAIN (Resource temporarily unavailable)
$ ./target/debug/olay_dongusu
yankı: yanki
$ strace -f -e trace=epoll_create1,epoll_ctl,epoll_wait,epoll_pwait,epoll_pwait2,accept4,connect,fcntl ./target/debug/olay_dongusu 2>&1 | grep -v "^yankı" | grep -vE "fcntl\([0-9]+, F_(GETFL|SETFL|GETFD|SETFD)"
epoll_create1(EPOLL_CLOEXEC)            = 3
epoll_ctl(3, EPOLL_CTL_ADD, 4, {events=EPOLLIN|EPOLLRDHUP|EPOLLET, data=0}) = 0
fcntl(3, F_DUPFD_CLOEXEC, 3)            = 5
fcntl(6, F_DUPFD_CLOEXEC, 3)            = 8
epoll_ctl(5, EPOLL_CTL_ADD, 8, {events=EPOLLIN|EPOLLRDHUP|EPOLLET, data=0x1}) = 0
epoll_ctl(5, EPOLL_CTL_ADD, 9, {events=EPOLLIN|EPOLLOUT|EPOLLRDHUP|EPOLLET, data=0x556b82b0e380}) = 0
connect(10, {sa_family=AF_INET, sin_port=htons(43115), sin_addr=inet_addr("127.0.0.1")}, 16) = -1 EINPROGRESS (Operation now in progress)
epoll_ctl(5, EPOLL_CTL_ADD, 10, {events=EPOLLIN|EPOLLOUT|EPOLLRDHUP|EPOLLET, data=0x556b82b0e780}) = 0
epoll_wait(3, [{events=EPOLLIN, data=0x556b82b0e380}, {events=EPOLLOUT, data=0x556b82b0e780}], 1024, -1) = 2
accept4(9, {sa_family=AF_INET, sin_port=htons(55874), sin_addr=inet_addr("127.0.0.1")}, [128 => 16], SOCK_CLOEXEC|SOCK_NONBLOCK) = 11
epoll_ctl(5, EPOLL_CTL_ADD, 11, {events=EPOLLIN|EPOLLOUT|EPOLLRDHUP|EPOLLET, data=0x556b82b0e900}) = 0
epoll_ctl(5, EPOLL_CTL_DEL, 11, NULL)   = 0
epoll_ctl(5, EPOLL_CTL_DEL, 9, NULL)    = 0
epoll_wait(3, [{events=EPOLLIN|EPOLLOUT|EPOLLRDHUP, data=0x556b82b0e780}], 1024, -1) = 1
epoll_ctl(5, EPOLL_CTL_DEL, 10, NULL)   = 0
+++ exited with 0 +++
$ timeout 2 ./target/debug/olay_dongusu std-engelleyen
[çıkış kodu: 124]
```

## 085-future-durum-makinesi

- Video: `085-future-durum-makinesi`
- Örnek yolu: `deneyler/mufredat-ornekleri/085-future-durum-makinesi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/durum_makinesi
# Derleyicinin ürettiği durum makinesi: varyantlar ve her varyantta saklanan yerel değişkenler (nightly 1.102.0 -Zprint-type-sizes; artımlı derleme kapalı ki tablo her seferinde yazılsın)
ks 'touch src/main.rs && CARGO_INCREMENTAL=0 cargo +nightly rustc -q -- -Zprint-type-sizes > target/tip-boyutlari.txt 2>&1; grep -A7 "async fn body of tampon_await_boyunca()}" target/tip-boyutlari.txt'
ks 'grep -A8 "async fn body of tampon_await_oncesi()}" target/tip-boyutlari.txt'
# Kendin dene: ikinci bir await (yeni varyant Suspend1) ve 64 baytlık tampon
k ./target/debug/durum_makinesi kendin-dene
ks 'grep -A13 "async fn body of tampon_iki_await()}" target/tip-boyutlari.txt'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/durum_makinesi
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
$ touch src/main.rs && CARGO_INCREMENTAL=0 cargo +nightly rustc -q -- -Zprint-type-sizes > target/tip-boyutlari.txt 2>&1; grep -A7 "async fn body of tampon_await_boyunca()}" target/tip-boyutlari.txt
print-type-size type: `{async fn body of tampon_await_boyunca()}`: 4098 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Suspend0`: 4097 bytes
print-type-size         local `.tampon`: 4096 bytes
print-type-size         local `.__awaitee`: 1 bytes, type: {async fn body of bos()}
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
$ grep -A8 "async fn body of tampon_await_oncesi()}" target/tip-boyutlari.txt
print-type-size type: `{async fn body of tampon_await_oncesi()}`: 16 bytes, alignment: 8 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Suspend0`: 9 bytes
print-type-size         local `.n`: 8 bytes, offset: 0 bytes, alignment: 8 bytes
print-type-size         local `.__awaitee`: 1 bytes, type: {async fn body of bos()}
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
print-type-size     end padding: 6 bytes
$ ./target/debug/durum_makinesi kendin-dene
size_of_val(tampon_iki_await())          = 24 bayt
size_of_val(kucuk_tampon_await_boyunca()) = 66 bayt
$ grep -A13 "async fn body of tampon_iki_await()}" target/tip-boyutlari.txt
print-type-size type: `{async fn body of tampon_iki_await()}`: 24 bytes, alignment: 8 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Suspend0`: 16 bytes
print-type-size         local `.n`: 8 bytes, offset: 0 bytes, alignment: 8 bytes
print-type-size         padding: 8 bytes
print-type-size         local `.__awaitee`: 1 bytes, alignment: 1 bytes, type: {async fn body of bos()}
print-type-size     variant `Suspend1`: 16 bytes
print-type-size         local `.n`: 8 bytes, offset: 0 bytes, alignment: 8 bytes
print-type-size         padding: 8 bytes
print-type-size         local `.__awaitee`: 1 bytes, alignment: 1 bytes, type: {async fn body of bos()}
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
print-type-size     end padding: 7 bytes
```

## 086-kendi-yurutucumuz

- Video: `086-kendi-yurutucumuz`
- Örnek yolu: `deneyler/mufredat-ornekleri/086-kendi-yurutucumuz/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: milisaniye değerleri ±1-2 ms oynayabilir; işlemci süreleri ve meşgul sürümdeki yoklama sayısı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q
k ./target/debug/yurutucu
# İşlemci ne kadar çalıştı? Uyuyan yürütücü ile hiç uyumadan yoklayan (meşgul) yürütücü
ks '/usr/bin/time -f "uyuyan yürütücü: işlemci kullanıcı %U s, sistem %S s; geçen süre %e s" ./target/debug/yurutucu 2>&1 | tail -n 1'
ks '/usr/bin/time -f "meşgul yürütücü: işlemci kullanıcı %U s, sistem %S s; geçen süre %e s" ./target/debug/yurutucu mesgul 2>&1 | tail -n 3'
# Kendin dene: uyandırıcı unpark yapmazsa (2 saniye sonra timeout)
k timeout 2 ./target/debug/yurutucu unpark-yok
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
$ ./target/debug/yurutucu
[görev] başladı          (  0 ms)
[görev] 1. bekleme bitti (200 ms)
[görev] 2. bekleme bitti (500 ms)
[yürütücü] 3 yoklamada tamamlandı
sonuç = 42
$ /usr/bin/time -f "uyuyan yürütücü: işlemci kullanıcı %U s, sistem %S s; geçen süre %e s" ./target/debug/yurutucu 2>&1 | tail -n 1
uyuyan yürütücü: işlemci kullanıcı 0.00 s, sistem 0.00 s; geçen süre 0.50 s
$ /usr/bin/time -f "meşgul yürütücü: işlemci kullanıcı %U s, sistem %S s; geçen süre %e s" ./target/debug/yurutucu mesgul 2>&1 | tail -n 3
[yürütücü] 9315507 yoklamada tamamlandı
sonuç = 42
meşgul yürütücü: işlemci kullanıcı 0.49 s, sistem 0.00 s; geçen süre 0.50 s
$ timeout 2 ./target/debug/yurutucu unpark-yok
[görev] başladı          (  0 ms)
[çıkış kodu: 124]
```

## 087-pin-ve-kendine-referans

- Video: `087-pin-ve-kendine-referans`
- Örnek yolu: `deneyler/mufredat-ornekleri/087-pin-ve-kendine-referans/`
- Kaynaklar: `sabitlenmis/Cargo.toml`, `sabitlenmis/src/main.rs`, `tasima-sorunu/Cargo.toml`, `tasima-sorunu/src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: adresler (ASLR); miri'nin bellek bloğu numarası
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd tasima-sorunu
k cargo run -q
# Eski yeri okumak: miri tanımsız davranışı yakalar
ks 'cargo +nightly miri run -q -- oku 2>&1 | head -n 12'
# Kendin dene: aynı program miri'de oku kipi olmadan (işaretçi yalnız yazdırılır, okunmaz)
ks 'cargo +nightly miri run -q 2>&1 | head -n 6'
cd ..
cd sabitlenmis && k cargo build -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
a.veri @ 0x7ffcefd44c00   a.isaretci = 0x7ffcefd44c00
b.veri @ 0x7ffcefd44c60   b.isaretci = 0x7ffcefd44c00  ← hâlâ eski yeri gösteriyor
sabitlenmiş değer @ 0x5613c0990d68, isaretci = 0x5613c0990d68
kutu taşındıktan sonra @ 0x5613c0990d68, isaretci = 0x5613c0990d68  ← aynı yer
$ cargo +nightly miri run -q -- oku 2>&1 | head -n 12
error: Undefined Behavior: memory access failed: alloc869 has been freed, so this pointer is dangling
  --> src/main.rs:24:28
   |
24 |         let ilk = unsafe { *b.isaretci }; // eski (artık geçersiz) yeri okumak
   |                            ^^^^^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
help: alloc869 was allocated here:
  --> src/main.rs:10:9
   |
10 |     let mut k = KendineBakan { veri: *b"abcd", isaretci: std::ptr::null() };
$ cargo +nightly miri run -q 2>&1 | head -n 6
a.veri @ 0x237a0   a.isaretci = 0x237a0
b.veri @ 0x2c8f8   b.isaretci = 0x237a0  ← hâlâ eski yeri gösteriyor
sabitlenmiş değer @ 0x34608, isaretci = 0x34608
kutu taşındıktan sonra @ 0x34608, isaretci = 0x34608  ← aynı yer
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

error[E0277]: `{async block@src/main.rs:13:23: 13:28}` cannot be unpinned
  --> src/main.rs:19:23
   |
19 |     let _q = Pin::new(&mut gelecek);
   |              -------- ^^^^^^^^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:13:23: 13:28}`
   |              |
   |              required by a bound introduced by this call
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
note: required by a bound in `Pin::<Ptr>::new`
  --> /rustc/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/pin.rs:1181:4

For more information about this error, try `rustc --explain E0277`.
error: could not compile `sabitlenmis` (bin "sabitlenmis") due to 2 previous errors
[çıkış kodu: 101]
```

## 088-tokio-ile-async

- Video: `088-tokio-ile-async`
- Örnek yolu: `deneyler/mufredat-ornekleri/088-tokio-ile-async/`
- Kaynaklar: `Cargo.toml`, `sayac.sh`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler, kovalara düşen istemci sayıları ve taşma sayaçları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k bash sayac.sh başta
k ./target/release/tokio_sunucu 100
k ./target/release/tokio_sunucu 1000
k bash sayac.sh "1000 istemciden sonra"
# tokio'nun bind() ile kurduğu kuyruk uzunluğu ve çekirdeğin üst sınırı
ks 'strace -f -e trace=listen ./target/release/tokio_sunucu 10 2>&1 | grep "listen("'
k cat /proc/sys/net/core/somaxconn
# Kuyruk doluyken çekirdek ret mi gönderir (1), adımı sessizce mi düşürür (0)?
k cat /proc/sys/net/ipv4/tcp_abort_on_overflow
# Kendin dene: kuyruğu 1024 yap
k ./target/release/tokio_sunucu 1000 1024
k bash sayac.sh "kuyruk 1024 ile sonra"
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ bash sayac.sh başta
başta: ListenOverflows 0
$ ./target/release/tokio_sunucu 100
çalışma zamanı işçi iş parçacığı: 12
100 istemci, kuyruk 128 (bind varsayılanı): toplam 122 ms | süreler <0,5 s: 100  ~1 s: 0  ~2 s: 0  ≥2,5 s: 0
zaman aşımı: yavaş iş iptal edildi
$ ./target/release/tokio_sunucu 1000
çalışma zamanı işçi iş parçacığı: 12
1000 istemci, kuyruk 128 (bind varsayılanı): toplam 1356 ms | süreler <0,5 s: 371  ~1 s: 629  ~2 s: 0  ≥2,5 s: 0
zaman aşımı: yavaş iş iptal edildi
$ bash sayac.sh "1000 istemciden sonra"
1000 istemciden sonra: ListenOverflows 1213
$ strace -f -e trace=listen ./target/release/tokio_sunucu 10 2>&1 | grep "listen("
[pid 25258] listen(9, 128)              = 0
$ cat /proc/sys/net/core/somaxconn
4096
$ cat /proc/sys/net/ipv4/tcp_abort_on_overflow
0
$ ./target/release/tokio_sunucu 1000 1024
çalışma zamanı işçi iş parçacığı: 12
1000 istemci, kuyruk 1024: toplam 155 ms | süreler <0,5 s: 1000  ~1 s: 0  ~2 s: 0  ≥2,5 s: 0
zaman aşımı: yavaş iş iptal edildi
$ bash sayac.sh "kuyruk 1024 ile sonra"
kuyruk 1024 ile sonra: ListenOverflows 1213
```

## 089-async-tuzaklari

- Video: `089-async-tuzaklari`
- Örnek yolu: `deneyler/mufredat-ornekleri/089-async-tuzaklari/`
- Kaynaklar: `bloklama/Cargo.toml`, `bloklama/src/main.rs`, `kilit-await/Cargo.toml`, `kilit-await/src/main.rs`, `kilit-await-duzeltilmis/Cargo.toml`, `kilit-await-duzeltilmis/src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: milisaniye değerleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
cd bloklama && k cargo run -q; cd ..
cd kilit-await && k cargo build -q; cd ..
# Kendin dene: kilidi await'ten önce bırakmak
cd kilit-await-duzeltilmis && k cargo run -q; cd ..
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
std::thread::sleep ile iki görev : 1000 ms
tokio::time::sleep ile iki görev :  501 ms
spawn_blocking ile iki görev     :  500 ms
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
$ cargo run -q
sayaç = 1
```

## 090-unsafe-ne-demek

- Video: `090-unsafe-ne-demek`
- Örnek yolu: `deneyler/mufredat-ornekleri/090-unsafe-ne-demek/`
- Kaynaklar: `ham-isaretci/Cargo.toml`, `ham-isaretci/src/main.rs`, `sozlesme/Cargo.toml`, `sozlesme/src/main.rs`, `surum-2024/Cargo.toml`, `surum-2024/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Ham işaretçi oluşturmak güvenli; gösterdiği yere erişmek unsafe ister (E0133)
cd ham-isaretci && k cargo build -q; cd ..
# Hangi işlemler unsafe gerektirir? Derleyicinin kendi açıklamasındaki örnekler
ks 'rustc --explain E0133 | sed -n "/^Using unsafe/,/^\* Calling functions marked/p"'
cd sozlesme && k cargo run -q; cd ..
# 2024 sürümünün kuralları: no_mangle ve extern blokları da "unsafe" diye işaretlenmeli
cd surum-2024 && k cargo build -q; cd ..
# Kendin dene: aynı dosya 2021 baskısıyla derlenir; bu iki kural 2024 baskısıyla geldi
ks 'rustc --edition 2021 -o /tmp/surum2021 surum-2024/src/main.rs && /tmp/surum2021'
# Kendin dene: sozlesme'de *p'yi unsafe { *p } içine almak; 2024 uyarısı kaybolur
ks 'rm -rf /tmp/sozlesme-kopya && mkdir /tmp/sozlesme-kopya && cp -r sozlesme/Cargo.toml sozlesme/Cargo.lock sozlesme/src /tmp/sozlesme-kopya/ && cd /tmp/sozlesme-kopya && sed -i "s/^    \*p \/\/.*/    unsafe { *p }/" src/main.rs && grep -n "unsafe { \*p }" src/main.rs && cargo run -q 2>&1'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q
error[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> src/main.rs:4:20
  |
4 |     println!("{}", *p); // içeriğine erişmek (dereference) ayrı iş
  |                    ^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `ham_isaretci` (bin "ham_isaretci") due to 1 previous error
[çıkış kodu: 101]
$ rustc --explain E0133 | sed -n "/^Using unsafe/,/^\* Calling functions marked/p"
Using unsafe functionality is potentially dangerous and disallowed by safety
checks. Examples:

* Dereferencing raw pointers
* Calling functions via FFI
* Calling functions marked unsafe
$ cargo run -q
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
 --> src/main.rs:4:5
  |
4 |     *p // 2024 baskısında: unsafe fn gövdesi de unsafe blok ister (uyarı)
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
$ rustc --edition 2021 -o /tmp/surum2021 surum-2024/src/main.rs && /tmp/surum2021
4
$ rm -rf /tmp/sozlesme-kopya && mkdir /tmp/sozlesme-kopya && cp -r sozlesme/Cargo.toml sozlesme/Cargo.lock sozlesme/src /tmp/sozlesme-kopya/ && cd /tmp/sozlesme-kopya && sed -i "s/^    \*p \/\/.*/    unsafe { *p }/" src/main.rs && grep -n "unsafe { \*p }" src/main.rs && cargo run -q 2>&1
4:    unsafe { *p }
7
```

## 091-tanimsiz-davranis-ve-miri

- Video: `091-tanimsiz-davranis-ve-miri`
- Örnek yolu: `deneyler/mufredat-ornekleri/091-tanimsiz-davranis-ve-miri/`
- Kaynaklar: `Cargo.toml`, `src/bin/birak_sonra_kullan.rs`, `src/bin/gecersiz_bool.rs`, `src/bin/sinir_disi.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: sınır dışı ve bırakılmış bellekten okunan değerler tanımsızdır, her derlemede/çalıştırmada değişebilir; Miri'nin ayırma numaraları (alloc…)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Normal çalıştırma: "çalışıyor gibi" görünür
k cargo run -q --bin sinir_disi
k cargo run -q --release --bin birak_sonra_kullan
# Aynı program, iki derleme, iki farklı sonuç: tanımsız davranışlı programın anlamı yoktur
k cargo run -q --bin gecersiz_bool
k cargo run -q --release --bin gecersiz_bool
# İki ikilide de karşılaştırma komutu yok: hangi kolun çalışacağına derleme anında karar verilmiş
ks 'objdump -d -M intel --no-show-raw-insn -C target/debug/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/ret/"'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/jmp/"'
# Seçilen metin: lea'nın gösterdiği adreste duran sabit (debug 9 bayt "yanlış\n", release 7 bayt "doğru\n")
ks 'a=$(objdump -d -M intel --no-show-raw-insn -C target/debug/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/ret/" | grep -oE "# [0-9a-f]+ <" | head -n 1 | grep -oE "[0-9a-f]+"); objdump -s -j .rodata --start-address=0x$a --stop-address=$((0x$a + 9)) target/debug/gecersiz_bool | tail -n 1'
ks 'a=$(objdump -d -M intel --no-show-raw-insn -C target/release/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/jmp/" | grep -oE "# [0-9a-f]+ <" | head -n 1 | grep -oE "[0-9a-f]+"); objdump -s -j .rodata --start-address=0x$a --stop-address=$((0x$a + 7)) target/release/gecersiz_bool | tail -n 1'
# Miri: programı soyut makine üzerinde yorumlayarak her bellek erişimini denetler
ks 'cargo +nightly miri run -q --bin sinir_disi 2>&1 | head -n 12'
ks 'cargo +nightly miri run -q --bin birak_sonra_kullan 2>&1 | head -n 12'
ks 'cargo +nightly miri run -q --bin gecersiz_bool 2>&1 | head -n 10'
# Kendin dene 1: p.add(3) yerine p.add(2) (etiket de v[2]); tanımlı okuma, Miri sessiz (çıkış kodu 0)
ks 'rm -rf /tmp/ub-kopya && mkdir /tmp/ub-kopya && cp -r Cargo.toml Cargo.lock src /tmp/ub-kopya/ && cd /tmp/ub-kopya && sed -i "s/p.add(3)/p.add(2)/; s/v\[3\] = /v[2] = /" src/bin/sinir_disi.rs && cargo run -q --bin sinir_disi && cargo +nightly miri run -q --bin sinir_disi && echo "Miri: tanımsız davranış yok"'
# Kendin dene 2: transmute(2u8) yerine transmute(1u8); iki derleme aynı sonucu verir
ks 'cd /tmp/ub-kopya && sed -i "s/transmute(2u8)/transmute(1u8)/" src/bin/gecersiz_bool.rs && cargo run -q --bin gecersiz_bool && cargo run -q --release --bin gecersiz_bool'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --bin sinir_disi
v[3] = 0
$ cargo run -q --release --bin birak_sonra_kullan
değer = 32767
$ cargo run -q --bin gecersiz_bool
yanlış
$ cargo run -q --release --bin gecersiz_bool
doğru
$ objdump -d -M intel --no-show-raw-insn -C target/debug/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/ret/"
0000000000014c20 <gecersiz_bool::main>:
   14c20:	push   rax
   14c21:	mov    BYTE PTR [rsp+0x7],0x0
   14c26:	lea    rdi,[rip+0xffffffffffff01d3]        # 4e00 <__rustc_debug_gdb_scripts_section__+0x28>
   14c2d:	mov    esi,0x9
   14c32:	call   14c70 <<core::fmt::Arguments>::from_str>
   14c37:	mov    rdi,rax
   14c3a:	mov    rsi,rdx
   14c3d:	call   QWORD PTR [rip+0x3dd8d]        # 529d0 <_DYNAMIC+0x238>
   14c43:	pop    rax
   14c44:	ret
$ objdump -d -M intel --no-show-raw-insn -C target/release/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/jmp/"
0000000000014b20 <gecersiz_bool::main>:
   14b20:	lea    rdi,[rip+0xffffffffffff0299]        # 4dc0 <__abi_tag+0x4ac4>
   14b27:	mov    esi,0xf
   14b2c:	jmp    QWORD PTR [rip+0x3dce6]        # 52818 <_DYNAMIC+0x230>
$ a=$(objdump -d -M intel --no-show-raw-insn -C target/debug/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/ret/" | grep -oE "# [0-9a-f]+ <" | head -n 1 | grep -oE "[0-9a-f]+"); objdump -s -j .rodata --start-address=0x$a --stop-address=$((0x$a + 9)) target/debug/gecersiz_bool | tail -n 1
 4e00 79616e6c c4b1c59f 0a                 yanl.....       
$ a=$(objdump -d -M intel --no-show-raw-insn -C target/release/gecersiz_bool | awk "/<gecersiz_bool::main>:/,/jmp/" | grep -oE "# [0-9a-f]+ <" | head -n 1 | grep -oE "[0-9a-f]+"); objdump -s -j .rodata --start-address=0x$a --stop-address=$((0x$a + 7)) target/release/gecersiz_bool | tail -n 1
 4dc0 646fc49f 72750a                      do..ru.         
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
$ rm -rf /tmp/ub-kopya && mkdir /tmp/ub-kopya && cp -r Cargo.toml Cargo.lock src /tmp/ub-kopya/ && cd /tmp/ub-kopya && sed -i "s/p.add(3)/p.add(2)/; s/v\[3\] = /v[2] = /" src/bin/sinir_disi.rs && cargo run -q --bin sinir_disi && cargo +nightly miri run -q --bin sinir_disi && echo "Miri: tanımsız davranış yok"
v[2] = 30
v[2] = 30
Miri: tanımsız davranış yok
$ cd /tmp/ub-kopya && sed -i "s/transmute(2u8)/transmute(1u8)/" src/bin/gecersiz_bool.rs && cargo run -q --bin gecersiz_bool && cargo run -q --release --bin gecersiz_bool
doğru
doğru
```

## 092-guvenli-soyutlama-yazmak

- Video: `092-guvenli-soyutlama-yazmak`
- Örnek yolu: `deneyler/mufredat-ornekleri/092-guvenli-soyutlama-yazmak/`
- Kaynaklar: `Cargo.toml`, `src/lib.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: Miri'nin etiket (<…>) ve ayırma (alloc…) numaraları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Normal testler: ikisi de "geçer"; hatalı sürümün tanımsız davranışı görünmez
k cargo test -q
# Miri altında: doğru sürüm temiz
ks 'cargo +nightly miri test -q dogru_surum 2>&1 | grep -E "test result|running"'
# Hatalı sürüm, varsayılan ödünç modeli (Stacked Borrows): çakışan &mut daha oluşurken yakalanır
ks 'cargo +nightly miri test -q hatali_surum 2>&1 | sed -n "/^error: Undefined Behavior/,/this is on thread/p"'
# Kendin dene: Tree Borrows modeliyle aynı test; ihlal ilk çakışan yazmada yakalanır
ks 'MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri test -q hatali_surum 2>&1 | sed -n "/^error: Undefined Behavior/,/this is on thread/p"'
# Kendin dene: hatalı sürümün testinden iki yazmayı ve assert_eq! satırını silmek; yalnız dilimler oluşturuluyor
ks 'rm -rf /tmp/bolme-kopya && mkdir /tmp/bolme-kopya && cp -r Cargo.toml Cargo.lock src /tmp/bolme-kopya/ && cd /tmp/bolme-kopya && sed -i "/a\[1\] = 20;/d; /b\[0\] = 99;/d; /assert_eq!(v\[1\], 99);/d" src/lib.rs && cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "^error: Undefined Behavior|^test result"'
ks 'cd /tmp/bolme-kopya && MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "^error: Undefined Behavior|^test result"'
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
$ cargo +nightly miri test -q hatali_surum 2>&1 | sed -n "/^error: Undefined Behavior/,/this is on thread/p"
error: Undefined Behavior: trying to retag from <130716> for Unique permission at alloc40299[0x4], but that tag does not exist in the borrow stack for this location
  --> src/lib.rs:23:9
   |
23 | /         (
24 | |             std::slice::from_raw_parts_mut(p, orta),
25 | |             std::slice::from_raw_parts_mut(p.add(orta - 1), n - orta + 1),
26 | |         )
   | |         ^
   | |         |
   | |_________this error occurs as part of retag at alloc40299[0x0..0x8]
   |           while retagging field .0
   |
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <130716> was created by a Unique retag at offsets [0x0..0x8]
  --> src/lib.rs:24:13
   |
24 |             std::slice::from_raw_parts_mut(p, orta),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: <130716> was later invalidated at offsets [0x4..0x14] by a Unique retag
  --> src/lib.rs:25:13
   |
25 |             std::slice::from_raw_parts_mut(p.add(orta - 1), n - orta + 1),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this is on thread `testler::hatali`
$ MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri test -q hatali_surum 2>&1 | sed -n "/^error: Undefined Behavior/,/this is on thread/p"
error: Undefined Behavior: write access through <128158> at alloc40274[0x4] is forbidden
  --> src/lib.rs:48:9
   |
48 |         b[0] = 99;
   |         ^^^^^^^^^ Undefined Behavior occurred here
   |
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Tree Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/tree-borrows.md for further information
   = help: the accessed tag <128158> has state Disabled which forbids this child write access
help: the accessed tag <128158> was created here, in the initial state Reserved
  --> src/lib.rs:46:17
   |
46 |         let (a, b) = ikiye_bol_hatali(&mut v, 2);
   |                 ^
help: the accessed tag <128158> later transitioned to Disabled due to a foreign write access at offsets [0x4..0x8]
  --> src/lib.rs:47:9
   |
47 |         a[1] = 20; // aynı elemana iki ayrı &mut üzerinden yazılıyor
   |         ^^^^^^^^^
   = help: this transition corresponds to a loss of read and write permissions
   = note: this is on thread `testler::hatali`
$ rm -rf /tmp/bolme-kopya && mkdir /tmp/bolme-kopya && cp -r Cargo.toml Cargo.lock src /tmp/bolme-kopya/ && cd /tmp/bolme-kopya && sed -i "/a\[1\] = 20;/d; /b\[0\] = 99;/d; /assert_eq!(v\[1\], 99);/d" src/lib.rs && cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "^error: Undefined Behavior|^test result"
error: Undefined Behavior: trying to retag from <130716> for Unique permission at alloc40938[0x4], but that tag does not exist in the borrow stack for this location
$ cd /tmp/bolme-kopya && MIRIFLAGS=-Zmiri-tree-borrows cargo +nightly miri test -q hatali_surum 2>&1 | grep -E "^error: Undefined Behavior|^test result"
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.13s
```

## 093-ffi-c-ile-konusmak

- Video: `093-ffi-c-ile-konusmak`
- Örnek yolu: `deneyler/mufredat-ornekleri/093-ffi-c-ile-konusmak/`
- Kaynaklar: `Cargo.toml`, `build.rs`, `c/hesap.c`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `cc 1.11.3`, `find-msvc-tools 0.1.15`, `shlex 2.0.1`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: sembol adresleri; v0 adındaki crate özeti (Cs…_) derleme ayarlarına göre değişebilir; yanlış beyanla okunan topla değeri tanımsızdır
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# İkilide iki tarafın sembolleri yan yana: no_mangle ile adlar olduğu gibi kalır
ks 'nm target/debug/ffi | grep -E " T (topla|selamla|karesi_arti_bir|rust_kare)$"'
# no_mangle olmayan bir Rust fonksiyonu (main): ad karıştırma, v0 biçimi (_R ile başlar)
ks 'nm target/debug/ffi | grep -E "_R[A-Za-z0-9_]*3ffi4main$"'
# Çağırma kuralı makine kodunda (x86-64 System V): ilk tamsayı argüman edi'de, dönüş değeri eax'te
k cargo build -q --release
ks 'objdump -d -M intel --no-show-raw-insn target/release/ffi | awk "/<rust_kare>:/,/ret/"'
ks 'objdump -d -M intel --no-show-raw-insn target/release/ffi | awk "/<karesi_arti_bir>:/,/ret/"'
# Kendin dene: no_mangle satırı silinince C tarafı rust_kare adını bulamaz (bağlama hatası)
ks 'rm -rf /tmp/ffi-kopya && mkdir /tmp/ffi-kopya && cp -r Cargo.toml Cargo.lock build.rs c src /tmp/ffi-kopya/ && cd /tmp/ffi-kopya && sed -i "/no_mangle/d" src/main.rs && cargo build -q 2>&1 | grep -E "undefined symbol|^error: linking"'
# Kendin dene: Rust tarafındaki topla beyanını f64 döndürür yapmak. rustc C kaynağını görmediği için itiraz etmez;
# kayan noktalı dönüş değeri xmm0'dan okunur, C ise sonucu eax'e koymuştu: anlamsız sayı (tanımsız davranış, sizde farklı)
ks 'rm -rf /tmp/ffi-kopya2 && mkdir /tmp/ffi-kopya2 && cp -r Cargo.toml Cargo.lock build.rs c src /tmp/ffi-kopya2/ && cd /tmp/ffi-kopya2 && sed -i "s/fn topla(a: c_int, b: c_int) -> c_int;/fn topla(a: c_int, b: c_int) -> f64;/" src/main.rs && cargo run -q 2>&1 | head -n 1'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
topla(2, 3) = 5
C diyor ki: merhaba Atölye
strlen("Atölye") = 7 bayt, Rust'ta 6 karakter
karesi_arti_bir(7) = 50 (C, Rust'taki rust_kare'yi çağırdı)
C dizesinin baytları: [65, 116, 195, 182, 108, 121, 101, 0]
$ nm target/debug/ffi | grep -E " T (topla|selamla|karesi_arti_bir|rust_kare)$"
0000000000016e76 T karesi_arti_bir
00000000000167d0 T rust_kare
0000000000016e3a T selamla
0000000000016e26 T topla
$ nm target/debug/ffi | grep -E "_R[A-Za-z0-9_]*3ffi4main$"
0000000000016330 t _RNvCsdQBekWUDKTL_3ffi4main
$ cargo build -q --release
$ objdump -d -M intel --no-show-raw-insn target/release/ffi | awk "/<rust_kare>:/,/ret/"
00000000000153f8 <rust_kare>:
   153f8:	mov    eax,edi
   153fa:	imul   eax,edi
   153fd:	ret
$ objdump -d -M intel --no-show-raw-insn target/release/ffi | awk "/<karesi_arti_bir>:/,/ret/"
0000000000015490 <karesi_arti_bir>:
   15490:	sub    rsp,0x8
   15494:	call   153f8 <rust_kare>
   15499:	add    rsp,0x8
   1549d:	add    eax,0x1
   154a0:	ret
$ rm -rf /tmp/ffi-kopya && mkdir /tmp/ffi-kopya && cp -r Cargo.toml Cargo.lock build.rs c src /tmp/ffi-kopya/ && cd /tmp/ffi-kopya && sed -i "/no_mangle/d" src/main.rs && cargo build -q 2>&1 | grep -E "undefined symbol|^error: linking"
error: linking with `cc` failed: exit status: 1
  = note: rust-lld: error: undefined symbol: rust_kare
$ rm -rf /tmp/ffi-kopya2 && mkdir /tmp/ffi-kopya2 && cp -r Cargo.toml Cargo.lock build.rs c src /tmp/ffi-kopya2/ && cd /tmp/ffi-kopya2 && sed -i "s/fn topla(a: c_int, b: c_int) -> c_int;/fn topla(a: c_int, b: c_int) -> f64;/" src/main.rs && cargo run -q 2>&1 | head -n 1
topla(2, 3) = 847448912897781200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

## 094-bellek-esleme

- Video: `094-bellek-esleme`
- Örnek yolu: `deneyler/mufredat-ornekleri/094-bellek-esleme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `libc 0.2.191`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: sayfa hatası sayıları (çekirdek sürümü, dosya sistemi ve önbellek durumuna bağlı) ve eşleme adresleri (ASLR)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
# Aynı 64 MiB'lık seyrek dosya üç kez eşlenir; her seferinde dosya baştan yazılır (sayfa önbelleği soğuk)
k ./target/release/mmap
# Programın çekirdekten istediği: mmap (MAP_POPULATE bayrağı) ve madvise (MADV_RANDOM)
ks 'strace -e trace=mmap,madvise,munmap ./target/release/mmap 2>&1 | grep -E "67108864"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/mmap
varsayılan    16384 sayfaya dokunuldu:   485 hafif, 1 ağır sayfa hatası (ilk 7 bayt "MERHABA", toplam 77)
              smaps: Rss 65536 kB, FilePmdMapped 0 kB; diskten okunan: 0 bayt
MADV_RANDOM   16384 sayfaya dokunuldu:     1 hafif, 16383 ağır sayfa hatası (ilk 7 bayt "MERHABA", toplam 77)
              smaps: Rss 65536 kB, FilePmdMapped 0 kB; diskten okunan: 0 bayt
MAP_POPULATE  16384 sayfaya dokunuldu:     0 hafif, 0 ağır sayfa hatası (ilk 7 bayt "MERHABA", toplam 77)
              smaps: Rss 65536 kB, FilePmdMapped 0 kB; diskten okunan: 0 bayt
$ strace -e trace=mmap,madvise,munmap ./target/release/mmap 2>&1 | grep -E "67108864"
mmap(NULL, 67108864, PROT_READ, MAP_PRIVATE, 4, 0) = 0x7f8d25b53000
munmap(0x7f8d25b53000, 67108864)        = 0
mmap(NULL, 67108864, PROT_READ, MAP_PRIVATE, 4, 0) = 0x7f8d25b53000
madvise(0x7f8d25b53000, 67108864, MADV_RANDOM) = 0
munmap(0x7f8d25b53000, 67108864)        = 0
mmap(NULL, 67108864, PROT_READ, MAP_PRIVATE|MAP_POPULATE, 4, 0) = 0x7f8d25b53000
munmap(0x7f8d25b53000, 67108864)        = 0
```

## 095-bildirimsel-makrolar

- Video: `095-bildirimsel-makrolar`
- Örnek yolu: `deneyler/mufredat-ornekleri/095-bildirimsel-makrolar/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: hijyen işaretlerindeki numaralar (ad kimliği ve söz dizimi bağlamı) derleyici sürümüne göre değişebilir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# Genişletilmiş kod ve hijyen işaretleri (yalnızca nightly): aynı ad x, iki farklı söz dizimi bağlamı (#…)
ks 'cargo +nightly rustc -q -- -Zunpretty=expanded,hygiene 2>&1 | grep -A4 "iki_kati!(x) ="'
# Kendin dene: adı çağıran versin ($ad:ident); iki x de bizim bağlamımızda (#0), sonuç 2 * 2
ks 'rm -rf /tmp/makro-kopya && mkdir /tmp/makro-kopya && cp -r Cargo.toml Cargo.lock src /tmp/makro-kopya/ && cd /tmp/makro-kopya && sed -i "s/(\$e:expr) => {{/(\$ad:ident, \$e:expr) => {{/; s/let x = 2;/let \$ad = 2;/; s/x \* \$e/\$ad * \$e/; s/iki_kati!(x))/iki_kati!(x, x))/; s/(makronun x.i 2, bizim x.imiz 10)/(adı çağıran verdi: tek bir x)/" src/main.rs && cargo run -q | grep iki_kati && cargo +nightly rustc -q -- -Zunpretty=expanded,hygiene 2>&1 | grep -A4 "iki_kati!(x) ="'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
{"armut": 5, "ayva": 1, "elma": 3}
en_buyuk!(3, 9, 4, 7) = 9
iki_kati!(x) = 20 (makronun x'i 2, bizim x'imiz 10)
$ cargo +nightly rustc -q -- -Zunpretty=expanded,hygiene 2>&1 | grep -A4 "iki_kati!(x) ="
                */(format_args!("iki_kati!(x) = {0} (makronun x\'i 2, bizim x\'imiz 10)\n",
                {
                    let x /* 2417#15 */ = 2;
                    x /* 2417#15 */ * x /* 2417#0 */
                }));
$ rm -rf /tmp/makro-kopya && mkdir /tmp/makro-kopya && cp -r Cargo.toml Cargo.lock src /tmp/makro-kopya/ && cd /tmp/makro-kopya && sed -i "s/(\$e:expr) => {{/(\$ad:ident, \$e:expr) => {{/; s/let x = 2;/let \$ad = 2;/; s/x \* \$e/\$ad * \$e/; s/iki_kati!(x))/iki_kati!(x, x))/; s/(makronun x.i 2, bizim x.imiz 10)/(adı çağıran verdi: tek bir x)/" src/main.rs && cargo run -q | grep iki_kati && cargo +nightly rustc -q -- -Zunpretty=expanded,hygiene 2>&1 | grep -A4 "iki_kati!(x) ="
warning: unused variable: `x`
  --> src/main.rs:32:9
   |
32 |     let x = 10;
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

iki_kati!(x) = 4 (adı çağıran verdi: tek bir x)
                */(format_args!("iki_kati!(x) = {0} (adı çağıran verdi: tek bir x)\n",
                { let x /* 2417#0 */ = 2; x /* 2417#0 */ * x /* 2417#0 */ }));
    };
}
```

## 096-makro-genisletme

- Video: `096-makro-genisletme`
- Örnek yolu: `deneyler/mufredat-ornekleri/096-makro-genisletme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
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
    let ad = "Atölye";
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
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Bağımlılıklar (Cargo.lock'ta çözülen): `proc-macro2 1.0.107`, `quote 1.0.47`, `syn 3.0.10`, `unicode-ident 1.0.27`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: derleme dizinindeki özet (hash) klasör adları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q -p kullanici
# Prosedürel makro, derleyicinin yüklediği bir paylaşılan kütüphane (.so) olarak derlenir
ks 'file $(find target/debug -name "libtanit_turet-*.so") | sed -E "s#/[0-9a-f]{16}/#/…/#; s#-[0-9a-f]{16}\.so#.so#; s#, BuildID[^,]*##"'
# türetme makrosunun ürettiği kod (genişletilmiş hâl, yalnızca nightly)
ks 'cargo +nightly rustc -q -p kullanici --target-dir target/nightly -- -Zunpretty=expanded 2>&1 | sed -n "/^impl Siparis/,/^}/p"'
k cargo build -q -p hatali-kullanim
ks 'cargo tree -p tanit-turet -e normal --depth 1'
# Derleme anında makronun .so dosyasını açan süreç (strace -Y süreç adını gösterir: <rustc>); özetler … ile kısaltıldı
ks 'touch kullanici/src/main.rs && strace -f -Y -e trace=openat cargo build -q -p kullanici 2>&1 | grep -E "libtanit_turet-[0-9a-f]+\.so\", O_RDONLY" | head -n 1 | sed -E "s#^\[pid +[0-9]+(<[^>]+>)\] #\1 #; s#/[0-9a-f]{16}/#/…/#; s#-[0-9a-f]{16}\.so#-….so#; s#= [0-9]+\$#= (fd)#"'
# Kendin dene: adsız alanlı struct için de hata üretmek (makronun bir kopyasında); hata Nokta satırını gösterir
ks 'rm -rf /tmp/tanit-kopya && mkdir /tmp/tanit-kopya && cp -r Cargo.toml Cargo.lock tanit-turet kullanici hatali-kullanim /tmp/tanit-kopya/ && cd /tmp/tanit-kopya && sed -i "s/            _ => Vec::new(),/            _ => return syn::Error::new_spanned(ad, \"Tanit adlı alanları olan struct ister\").to_compile_error().into(),/" tanit-turet/src/lib.rs && printf "\n#[derive(Tanit)]\nstruct Nokta(i32, i32);\n" >> kullanici/src/main.rs && cargo build -q -p kullanici 2>&1 | grep -A4 "^error: Tanit adlı"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q -p kullanici
Siparis alanları: ["numara", "musteri", "tutar_kurus"]
$ file $(find target/debug -name "libtanit_turet-*.so") | sed -E "s#/[0-9a-f]{16}/#/…/#; s#-[0-9a-f]{16}\.so#.so#; s#, BuildID[^,]*##"
target/debug/build/tanit-turet/…/out/libtanit_turet.so: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, with debug_info, not stripped
$ cargo +nightly rustc -q -p kullanici --target-dir target/nightly -- -Zunpretty=expanded 2>&1 | sed -n "/^impl Siparis/,/^}/p"
impl Siparis {
    pub fn tip_adi() -> &'static str { "Siparis" }
    pub fn alan_adlari() -> &'static [&'static str] {
        &["numara", "musteri", "tutar_kurus"]
    }
}
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
$ touch kullanici/src/main.rs && strace -f -Y -e trace=openat cargo build -q -p kullanici 2>&1 | grep -E "libtanit_turet-[0-9a-f]+\.so\", O_RDONLY" | head -n 1 | sed -E "s#^\[pid +[0-9]+(<[^>]+>)\] #\1 #; s#/[0-9a-f]{16}/#/…/#; s#-[0-9a-f]{16}\.so#-….so#; s#= [0-9]+\$#= (fd)#"
<rustc> openat(AT_FDCWD, "/work/mufredat-ornekleri/097-prosedurel-makrolar/target/debug/build/tanit-turet/…/out/libtanit_turet-….so", O_RDONLY|O_CLOEXEC) = (fd)
$ rm -rf /tmp/tanit-kopya && mkdir /tmp/tanit-kopya && cp -r Cargo.toml Cargo.lock tanit-turet kullanici hatali-kullanim /tmp/tanit-kopya/ && cd /tmp/tanit-kopya && sed -i "s/            _ => Vec::new(),/            _ => return syn::Error::new_spanned(ad, \"Tanit adlı alanları olan struct ister\").to_compile_error().into(),/" tanit-turet/src/lib.rs && printf "\n#[derive(Tanit)]\nstruct Nokta(i32, i32);\n" >> kullanici/src/main.rs && cargo build -q -p kullanici 2>&1 | grep -A4 "^error: Tanit adlı"
error: Tanit adlı alanları olan struct ister
  --> kullanici/src/main.rs:16:8
   |
16 | struct Nokta(i32, i32);
   |        ^^^^^
```

## 098-lexer-ve-parser

- Video: `098-lexer-ve-parser`
- Örnek yolu: `deneyler/mufredat-ornekleri/098-lexer-ve-parser/`
- Kaynaklar: `belirtecler.rs`, `birlesme.rs`, `ifade.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Sözcüksel çözümleme: kaynak metin belirteçlere (token) bölünür; parantezli grup tek bir belirteç ağacıdır
ks 'rustc --edition 2024 -o /tmp/belirtecler belirtecler.rs && /tmp/belirtecler'
# Ayrıştırıcının ürettiği soyut söz dizimi ağacı (AST): öncelik ağacın biçiminde
ks 'rustc +nightly --edition 2024 -Zunpretty=ast-tree ifade.rs | grep -E "kind: (Binary|Lit)|Add|Mul|symbol" '
# Kendin dene: birleşme özelliği. 10 - 4 - 3 soldan birleşir: (10 - 4) - 3
ks 'rustc +nightly --edition 2024 -Zunpretty=ast-tree birlesme.rs | grep -E "kind: (Binary|Lit)|Sub|symbol" '
# Söz dizimi hatası: ayrıştırıcı nerede takıldığını söyler
ks 'echo "fn main() { let x = (1 + 2; }" > /tmp/bozuk.rs && rustc --edition 2024 /tmp/bozuk.rs'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ rustc --edition 2024 -o /tmp/belirtecler belirtecler.rs && /tmp/belirtecler
let
toplam_2
=
0x2A
+
1_000u32
*
(3 - x)
;
a
>>=
b
;
"bir dize"
---
a
>
>=
b
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
$ rustc +nightly --edition 2024 -Zunpretty=ast-tree birlesme.rs | grep -E "kind: (Binary|Lit)|Sub|symbol" 
                                                    kind: Binary(
                                                            node: Sub,
                                                            kind: Binary(
                                                                    node: Sub,
                                                                    kind: Lit(
                                                                            symbol: "10",
                                                                    kind: Lit(
                                                                            symbol: "4",
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
- Kaynaklar: `seker.rs`, `tip_hatasi.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# HIR: sözdizimsel şekerler (for, ?) açılmış hâli
ks 'rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn topla/,/^}/p"'
ks 'rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn oku/,/^}/p"'
# Tip denetimi açılmış kod üzerinde: hata mesajları açılımdaki trait'leri (IntoIterator, FromResidual) anıyor
ks 'rustc --edition 2024 --crate-type=lib -o /tmp/tip_hatasi.rlib tip_hatasi.rs 2>&1'
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
$ rustc --edition 2024 --crate-type=lib -o /tmp/tip_hatasi.rlib tip_hatasi.rs 2>&1
error[E0277]: `{integer}` is not an iterator
 --> tip_hatasi.rs:2:14
  |
2 |     for x in 5 {
  |              ^ `{integer}` is not an iterator
  |
  = help: the trait `Iterator` is not implemented for `{integer}`
  = note: if you want to iterate between `start` until a value `end`, use the exclusive range syntax `start..end` or the inclusive range syntax `start..=end`
  = note: required for `{integer}` to implement `IntoIterator`

error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
  --> tip_hatasi.rs:8:31
   |
 7 | fn oku(metin: &str) {
   | ------------------- this function should return `Result` or `Option` to accept `?`
 8 |     let n: i32 = metin.parse()?;
   |                               ^ cannot use the `?` operator in a function that returns `()`
   |
help: consider adding return type
   |
 7 ~ fn oku(metin: &str) -> Result<(), Box<dyn std::error::Error>> {
 8 |     let n: i32 = metin.parse()?;
 9 |     println!("{n}");
10 +     Ok(())
   |

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
[çıkış kodu: 1]
```

## 100-mir-ve-odunc-denetimi

- Video: `100-mir-ve-odunc-denetimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/100-mir-ve-odunc-denetimi/`
- Kaynaklar: `mir.rs`, `odunc.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p target
# MIR: temel bloklardan (bb0, bb1...) oluşan kontrol akış grafı
k rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=mir -o target/mir.mir mir.rs
ks 'sed -n "/fn uzunluk/,/^}/p" target/mir.mir'
# Ödünç denetimi akışa duyarlı: üç fonksiyondan yalnız sira_yanlis reddedilir
ks 'rustc --edition 2024 --crate-type=lib --out-dir target odunc.rs 2>&1'
# Görsel için: yola_gore'nin MIR'i. Reddedilen kodun MIR'i üretilmez; sira_yanlis çıkarılmış bir kopyadan
ks 'sed "/^pub fn sira_yanlis/,/^}/d" odunc.rs > target/odunc_mir.rs && rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=mir -o target/odunc.mir target/odunc_mir.rs && sed -n "/fn yola_gore/,/^}/p" target/odunc.mir'
```

- Gerçek çıktı (kısaltılmış; 112 satırın tamamı `cikti.txt` içinde):

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
$ rustc --edition 2024 --crate-type=lib --out-dir target odunc.rs 2>&1
error[E0502]: cannot borrow `*v` as mutable because it is also borrowed as immutable
  --> odunc.rs:19:5
   |
18 |     let ilk = &v[0];
   |                - immutable borrow occurs here
19 |     v.push(4); // ilk hâlâ canlı: push için &mut gerekir
   |     ^^^^^^^^^ mutable borrow occurs here
20 |     println!("{ilk}");
   |                --- immutable borrow later used here

error: aborting due to 1 previous error
… (1 satır atlandı)
For more information about this error, try `rustc --explain E0502`.
[çıkış kodu: 1]
$ sed "/^pub fn sira_yanlis/,/^}/d" odunc.rs > target/odunc_mir.rs && rustc --edition 2024 --crate-type=lib -C opt-level=0 --emit=mir -o target/odunc.mir target/odunc_mir.rs && sed -n "/fn yola_gore/,/^}/p" target/odunc.mir
fn yola_gore(_1: &mut Vec<i32>, _2: bool) -> () {
    debug v => _1;
    debug kosul => _2;
    let mut _0: ();
    let _3: &i32;
    let _4: &i32;
    let mut _5: &std::vec::Vec<i32>;
    let _6: ();
    let _7: ();
    let mut _8: std::fmt::Arguments<'_>;
    let mut _10: &&i32;
    let mut _12: core::fmt::rt::Argument<'_>;
    let mut _13: &[u8; 4];
    let _14: &[core::fmt::rt::Argument<'_>; 1];
    let mut _15: &&i32;
    scope 1 {
        debug ilk => _3;
        let _9: (&&i32,);
        scope 2 {
            debug args => _9;
            let _11: [core::fmt::rt::Argument<'_>; 1];
            scope 3 {
                debug args => _11;
            }
        }
    }
… (1 satır atlandı)
    bb0: {
        _5 = &(*_1);
        _4 = <Vec<i32> as Index<usize>>::index(move _5, const 0_usize) -> [return: bb1, unwind continue];
    }
… (1 satır atlandı)
    bb1: {
        _3 = copy _4;
        switchInt(copy _2) -> [0: bb3, otherwise: bb2];
    }
… (1 satır atlandı)
    bb2: {
        _6 = Vec::<i32>::push(copy _1, const 4_i32) -> [return: bb6, unwind continue];
    }
… (1 satır atlandı)
    bb3: {
        _10 = &_3;
        _9 = (move _10,);
        _15 = no_retag copy (_9.0: &&i32);
        _12 = core::fmt::rt::Argument::<'_>::new_display::<&i32>(copy _15) -> [return: bb4, unwind continue];
    }

    bb4: {
        _11 = [move _12];
        _13 = const b"\xc0\x01\n\x00";
        _14 = &_11;
        _8 = Arguments::<'_>::new::<4, 1>(move _13, copy _14) -> [return: bb5, unwind continue];
    }

    bb5: {
        _7 = std::io::_print(move _8) -> [return: bb6, unwind continue];
    }

    bb6: {
        return;
    }
}
```

## 101-llvm-ve-optimizasyon

- Video: `101-llvm-ve-optimizasyon`
- Örnek yolu: `deneyler/mufredat-ornekleri/101-llvm-ve-optimizasyon/`
- Kaynaklar: `dongu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
mkdir -p target
k rustc --edition 2024 --crate-type=lib -C opt-level=0 -C debug-assertions=off -C overflow-checks=off --emit=llvm-ir -o target/O0.ll dongu.rs
k rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/O3.ll dongu.rs
ks 'echo "O0 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O0.ll | wc -l)  O3 satır: $(awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll | wc -l)"'
# O3: döngü (br/phi) kalmadı mı? Hangi komutlar var?
ks 'awk "/^define.*@kareler_toplami/,/^}/" target/O3.ll'
# Hangi geçişler bu fonksiyonu değiştirdi? (LLVM'in kendi raporu: -print-changed)
ks 'rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/O3b.ll -C llvm-args=-print-changed=quiet dongu.rs 2> target/gecisler.txt; grep -E "IR Dump After .*kareler_toplami" target/gecisler.txt | sed -E "s/^\*\*\* IR Dump After //; s/ \*\*\*$//"'
# Döngü geçişlerinden önce (LoopRotate sonrası): iki phi düğümlü döngü
ks 'awk "/IR Dump After LoopRotatePass/,/IR Dump After SimplifyCFGPass/" target/gecisler.txt | grep -v "IR Dump After SimplifyCFGPass"'
# IndVarSimplify sonrası: döngü gövdesi boşaldı (br i1 false), sonuç kapalı biçimle çıkışta hesaplanıyor
ks 'awk "/IR Dump After IndVarSimplifyPass/,/IR Dump After GVNPass/" target/gecisler.txt | grep -v "IR Dump After GVNPass"'
# Kendin dene: kareler yerine küpler (i * i * i); döngü yine kalkıyor mu?
ks 'sed "s/i \* i)/i * i * i)/" dongu.rs > target/kup.rs && rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/kup.ll target/kup.rs && awk "/^define.*@kareler_toplami/,/^}/" target/kup.ll | grep -E "^[a-z0-9.]+:|phi|ret"'
# Küp sürümünde ara hesap genişlikleri: 65 ve 67 bitlik tam sayılar
ks 'awk "/^define.*@kareler_toplami/,/^}/" target/kup.ll | grep -oE "i6[57]" | sort | uniq -c'
```

- Gerçek çıktı (kısaltılmış; 108 satırın tamamı `cikti.txt` içinde):

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
$ rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/O3b.ll -C llvm-args=-print-changed=quiet dongu.rs 2> target/gecisler.txt; grep -E "IR Dump After .*kareler_toplami" target/gecisler.txt | sed -E "s/^\*\*\* IR Dump After //; s/ \*\*\*$//"
SROAPass on kareler_toplami
LCSSAPass on kareler_toplami
LoopRotatePass on loop %bb2 in function kareler_toplami
SimplifyCFGPass on kareler_toplami
InstCombinePass on kareler_toplami
LoopSimplifyPass on kareler_toplami
LCSSAPass on kareler_toplami
IndVarSimplifyPass on loop %bb2 in function kareler_toplami
GVNPass on kareler_toplami
SCCPPass on kareler_toplami
PostOrderFunctionAttrsPass on (kareler_toplami)
$ awk "/IR Dump After LoopRotatePass/,/IR Dump After SimplifyCFGPass/" target/gecisler.txt | grep -v "IR Dump After SimplifyCFGPass"
*** IR Dump After LoopRotatePass on loop %bb2 in function kareler_toplami ***
… (1 satır atlandı)
; Preheader:
bb2.lr.ph:                                        ; preds = %start
  br label %bb2
… (1 satır atlandı)
; Loop:
bb2:                                              ; preds = %bb2.lr.ph, %bb2
  %s.sroa.0.05 = phi i64 [ 0, %bb2.lr.ph ], [ %_4, %bb2 ]
  %iter.sroa.0.04 = phi i64 [ 0, %bb2.lr.ph ], [ %0, %bb2 ]
  %0 = add nuw i64 %iter.sroa.0.04, 1
  %_5 = mul i64 %iter.sroa.0.04, %iter.sroa.0.04
  %_4 = add i64 %s.sroa.0.05, %_5
  %_6 = icmp ult i64 %0, %n
  br i1 %_6, label %bb2, label %bb1.bb3_crit_edge
… (1 satır atlandı)
; Exit blocks
bb1.bb3_crit_edge:                                ; preds = %bb2
  %split = phi i64 [ %_4, %bb2 ]
  br label %bb3
$ awk "/IR Dump After IndVarSimplifyPass/,/IR Dump After GVNPass/" target/gecisler.txt | grep -v "IR Dump After GVNPass"
*** IR Dump After IndVarSimplifyPass on loop %bb2 in function kareler_toplami ***
… (1 satır atlandı)
; Preheader:
bb2.preheader:                                    ; preds = %start
  br label %bb2
… (1 satır atlandı)
; Loop:
bb2:                                              ; preds = %bb2.preheader, %bb2
  br i1 false, label %bb2, label %bb3.loopexit
… (1 satır atlandı)
; Exit blocks
bb3.loopexit:                                     ; preds = %bb2
  %0 = add i64 %n, -1
  %1 = zext i64 %0 to i65
  %2 = add i64 %n, -2
  %3 = zext i64 %2 to i65
  %4 = mul i65 %1, %3
  %5 = add i64 %n, -3
  %6 = zext i64 %5 to i65
  %7 = mul i65 %4, %6
  %8 = lshr i65 %7, 1
  %9 = trunc i65 %8 to i64
  %10 = mul i64 %9, 6148914691236517206
  %11 = add i64 %n, %10
  %12 = mul i65 %1, %3
  %13 = lshr i65 %12, 1
  %14 = trunc i65 %13 to i64
  %15 = mul i64 %14, 3
  %16 = add i64 %11, %15
  %17 = add i64 %16, -1
  br label %bb3
$ sed "s/i \* i)/i * i * i)/" dongu.rs > target/kup.rs && rustc --edition 2024 --crate-type=lib -C opt-level=3 --emit=llvm-ir -o target/kup.ll target/kup.rs && awk "/^define.*@kareler_toplami/,/^}/" target/kup.ll | grep -E "^[a-z0-9.]+:|phi|ret"
start:
bb2.preheader:                                    ; preds = %start
bb3:                                              ; preds = %bb2.preheader, %start
  %s.sroa.0.0.lcssa = phi i64 [ 0, %start ], [ %25, %bb2.preheader ]
  ret i64 %s.sroa.0.0.lcssa
$ awk "/^define.*@kareler_toplami/,/^}/" target/kup.ll | grep -oE "i6[57]" | sort | uniq -c
      6 i65
      9 i67
```

## 102-kod-uretimi-ve-artimli-derleme

- Video: `102-kod-uretimi-ve-artimli-derleme`
- Örnek yolu: `deneyler/mufredat-ornekleri/102-kod-uretimi-ve-artimli-derleme/`
- Kaynaklar: `uret.sh`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: derleme süreleri; artımlı önbellekteki .o dosyalarının adları
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
rm -rf calisma && mkdir -p calisma && cd calisma && bash ../uret.sh && cd buyuk
k wc -l src/main.rs
ks '/usr/bin/time -f "temiz derleme (debug): %e s" cargo build -q'
# Artımlı önbellekte her kod üretim birimi (CGU) bir .o dosyası
ks 'cd target/debug/incremental/buyuk-*/s-*/ && md5sum *.o > /tmp/buyuk-once.txt && echo "kod üretim birimi (.o): $(ls *.o | wc -l)" && ls -S *.o | head -n 1 | xargs stat -c "en büyüğü: %s bayt"'
ks 'cd target/debug/incremental/buyuk-*/s-*/ && ls -l *.o | awk "{print \$5}" | sort -n | tr "\n" " " | sed "s/^/bütün birimler (bayt): /"; echo'
ks '/usr/bin/time -f "değişiklik yok: %e s" cargo build -q'
ks 'sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "tek satır değişti (artımlı): %e s" cargo build -q'
ks 'cd target/debug/incremental/buyuk-*/s-*/ && echo "yeniden üretilen .o: $(md5sum *.o | grep -vxFf /tmp/buyuk-once.txt | wc -l)" && md5sum *.o | grep -vxFf /tmp/buyuk-once.txt | awk "{print \$2}" | xargs stat -c "yeniden üretilenin boyutu: %s bayt"'
ks 'CARGO_INCREMENTAL=0 /usr/bin/time -f "aynı değişiklik, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz'
# Aynı 20000 fonksiyon, 100 modüle bölünmüş
cd ../moduler
ks 'grep -c "^mod " src/main.rs'
ks '/usr/bin/time -f "100 modül, temiz derleme (debug): %e s" cargo build -q'
ks 'cd target/debug/incremental/moduler-*/s-*/ && md5sum *.o > /tmp/moduler-once.txt && echo "kod üretim birimi (.o): $(ls *.o | wc -l)" && ls -S *.o | head -n 1 | xargs stat -c "en büyüğü: %s bayt"'
ks 'cd target/debug/incremental/moduler-*/s-*/ && ls -l *.o | awk "{print \$5}" | sort -n | awk "{a[NR]=\$1} END {printf \"en küçük 7 birim (bayt): \"; for (i = 1; i <= 7; i++) printf \"%d \", a[i]; printf \"\\nsonraki %d birim: %d – %d bayt\\nen büyük birim: %d bayt\\n\", NR - 8, a[8], a[NR - 1], a[NR]}"'
ks 'sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "100 modül, tek satır değişti (artımlı): %e s" cargo build -q'
ks 'cd target/debug/incremental/moduler-*/s-*/ && echo "yeniden üretilen .o: $(md5sum *.o | grep -vxFf /tmp/moduler-once.txt | wc -l)" && md5sum *.o | grep -vxFf /tmp/moduler-once.txt | awk "{print \$2}" | xargs stat -c "yeniden üretilenin boyutu: %s bayt"'
cd ../buyuk
# Kod üretim birimleri (codegen-units): paralel kod üretimi ↔ optimizasyon fırsatı
ks 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 /usr/bin/time -f "release, codegen-units=16: %e s" cargo build -q --release --target-dir target/cgu16'
ks 'CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 /usr/bin/time -f "release, codegen-units=1 : %e s" cargo build -q --release --target-dir target/cgu1'
ks './target/cgu16/release/buyuk; ./target/cgu1/release/buyuk'
# Zaman nereye gidiyor? Artımlı yeniden derlemede derleyici aşamaları (nightly, -Ztime-passes), en uzun 8 aşama
k cargo +nightly build -q --target-dir target/nightly
ks 'sed -i "s/(x ^ 15001)/(x ^ 15002)/" src/main.rs && cargo +nightly rustc -q --target-dir target/nightly -- -Ztime-passes 2>&1 | grep -E "time: +[0-9.]+;" | sort -k2 -n -r | head -n 8'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ wc -l src/main.rs
40004 src/main.rs
$ /usr/bin/time -f "temiz derleme (debug): %e s" cargo build -q
temiz derleme (debug): 4.20 s
$ cd target/debug/incremental/buyuk-*/s-*/ && md5sum *.o > /tmp/buyuk-once.txt && echo "kod üretim birimi (.o): $(ls *.o | wc -l)" && ls -S *.o | head -n 1 | xargs stat -c "en büyüğü: %s bayt"
kod üretim birimi (.o): 8
en büyüğü: 18065424 bayt
$ cd target/debug/incremental/buyuk-*/s-*/ && ls -l *.o | awk "{print \$5}" | sort -n | tr "\n" " " | sed "s/^/bütün birimler (bayt): /"; echo
bütün birimler (bayt): 3088 3288 3936 5728 5928 6272 7344 18065424 
$ /usr/bin/time -f "değişiklik yok: %e s" cargo build -q
değişiklik yok: 0.01 s
$ sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "tek satır değişti (artımlı): %e s" cargo build -q
tek satır değişti (artımlı): 2.69 s
$ cd target/debug/incremental/buyuk-*/s-*/ && echo "yeniden üretilen .o: $(md5sum *.o | grep -vxFf /tmp/buyuk-once.txt | wc -l)" && md5sum *.o | grep -vxFf /tmp/buyuk-once.txt | awk "{print \$2}" | xargs stat -c "yeniden üretilenin boyutu: %s bayt"
yeniden üretilen .o: 1
yeniden üretilenin boyutu: 18065424 bayt
$ CARGO_INCREMENTAL=0 /usr/bin/time -f "aynı değişiklik, artımlı kapalı: %e s" cargo build -q --target-dir target/artimsiz
aynı değişiklik, artımlı kapalı: 3.57 s
$ grep -c "^mod " src/main.rs
100
$ /usr/bin/time -f "100 modül, temiz derleme (debug): %e s" cargo build -q
100 modül, temiz derleme (debug): 3.18 s
$ cd target/debug/incremental/moduler-*/s-*/ && md5sum *.o > /tmp/moduler-once.txt && echo "kod üretim birimi (.o): $(ls *.o | wc -l)" && ls -S *.o | head -n 1 | xargs stat -c "en büyüğü: %s bayt"
kod üretim birimi (.o): 108
en büyüğü: 2273808 bayt
$ cd target/debug/incremental/moduler-*/s-*/ && ls -l *.o | awk "{print \$5}" | sort -n | awk "{a[NR]=\$1} END {printf \"en küçük 7 birim (bayt): \"; for (i = 1; i <= 7; i++) printf \"%d \", a[i]; printf \"\\nsonraki %d birim: %d – %d bayt\\nen büyük birim: %d bayt\\n\", NR - 8, a[8], a[NR - 1], a[NR]}"
en küçük 7 birim (bayt): 3096 3304 3944 5744 5944 6280 7368 
sonraki 100 birim: 174264 – 177680 bayt
en büyük birim: 2273808 bayt
$ sed -i "s/(x ^ 15000)/(x ^ 15001)/" src/main.rs && /usr/bin/time -f "100 modül, tek satır değişti (artımlı): %e s" cargo build -q
100 modül, tek satır değişti (artımlı): 1.10 s
$ cd target/debug/incremental/moduler-*/s-*/ && echo "yeniden üretilen .o: $(md5sum *.o | grep -vxFf /tmp/moduler-once.txt | wc -l)" && md5sum *.o | grep -vxFf /tmp/moduler-once.txt | awk "{print \$2}" | xargs stat -c "yeniden üretilenin boyutu: %s bayt"
yeniden üretilen .o: 1
yeniden üretilenin boyutu: 177680 bayt
$ CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 /usr/bin/time -f "release, codegen-units=16: %e s" cargo build -q --release --target-dir target/cgu16
release, codegen-units=16: 4.26 s
$ CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 /usr/bin/time -f "release, codegen-units=1 : %e s" cargo build -q --release --target-dir target/cgu1
release, codegen-units=1 : 6.11 s
$ ./target/cgu16/release/buyuk; ./target/cgu1/release/buyuk
6347231679757235248
6347231679757235248
$ cargo +nightly build -q --target-dir target/nightly
$ sed -i "s/(x ^ 15001)/(x ^ 15002)/" src/main.rs && cargo +nightly rustc -q --target-dir target/nightly -- -Ztime-passes 2>&1 | grep -E "time: +[0-9.]+;" | sort -k2 -n -r | head -n 8
time:   2.493; rss:   26MB ->  288MB ( +262MB)	total
time:   1.029; rss:  554MB ->  536MB (  -17MB)	LLVM_passes
time:   0.933; rss:  484MB ->  516MB (  +32MB)	finish_ongoing_codegen
time:   0.344; rss:  441MB ->  565MB ( +124MB)	codegen_crate
time:   0.261; rss:  474MB ->  566MB (  +92MB)	codegen_to_LLVM_IR
time:   0.257; rss:  474MB ->  554MB (  +80MB)	compile_first_CGU_batch
time:   0.148; rss:  372MB ->  441MB (  +68MB)	misc_checking_3
time:   0.125; rss:   36MB ->  151MB ( +115MB)	parse_crate
```

## 103-olcum-bilimi

- Video: `103-olcum-bilimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/103-olcum-bilimi/`
- Kaynaklar: `black-box/Cargo.toml`, `black-box/src/main.rs`, `tuzak/Cargo.toml`, `tuzak/src/main.rs`, `tuzak-n-gizli/Cargo.toml`, `tuzak-n-gizli/src/main.rs`
- Toolchain: stable 1.100.0, python3 (konteyner)
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Tuzak 1: ilk sürümümüzde satır/sütun gezinme süreleri neredeyse aynı çıktı (045'in ilk hâli)
cd tuzak && k cargo run -q --release; cd ..
# Aynı kod, tek fark: matris boyutu n derleme anında bilinmiyor (black_box)
cd tuzak-n-gizli && ks 'cargo run -q --release | grep matris'; cd ..
# Neden? LLVM'in kendi raporu: n bilinince döngü yer değiştirme (LoopInterchange) geçişi kodu değiştirdi, gizlenince değiştirmedi
cd tuzak && ks 'RUSTFLAGS="-C llvm-args=-print-changed=quiet" cargo build -q --release --target-dir target/gecisler 2>&1 | grep -c "IR Dump After LoopInterchangePass" | xargs echo "n bilinirken LoopInterchangePass değişikliği:"'; cd ..
cd tuzak-n-gizli && ks 'RUSTFLAGS="-C llvm-args=-print-changed=quiet" cargo build -q --release --target-dir target/gecisler 2>&1 | grep -c "IR Dump After LoopInterchangePass" | xargs echo "n gizlenince LoopInterchangePass değişikliği:"'; cd ..
# Kendin dene: n bilinirken geçişi kapatmak (LLVM seçeneği); sütun gezinme yine yavaşlar
cd tuzak && ks 'RUSTFLAGS="-C llvm-args=-enable-loopinterchange=false" cargo run -q --release --target-dir target/donusumsuz | grep matris'; cd ..
# Tuzak 2: sonucu kullanılmayan hesap ölçülemez
cd black-box && k cargo run -q --release
# İstatistik: tek ölçüm değil, dağılım (hyperfine: ısınma, ortalama ± σ, aralık)
k cargo build -q --release
k hyperfine -N --warmup 3 --runs 30 --style basic --export-json target/olcum.json ./target/release/black_box
# Otuz ölçümün hepsi (ms, küçükten büyüğe): dağılımı kendimiz görelim
ks 'python3 -c "import json; t=sorted(x*1000 for x in json.load(open(\"target/olcum.json\"))[\"results\"][0][\"times\"]); print(\" \".join(f\"{x:.1f}\" for x in t))"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q --release
matris satır-öncelikli gezinme         3.13 ms
matris sütun-öncelikli gezinme         2.38 ms
Vec<u64> toplamı                       1.98 ms
LinkedList<u64> toplamı                7.38 ms
AoS: Vec<Parcacik> içinden kütle       4.26 ms
SoA: Vec<f32> kütle dizisi             2.72 ms
$ cargo run -q --release | grep matris
matris satır-öncelikli gezinme         3.48 ms
matris sütun-öncelikli gezinme        74.74 ms
$ RUSTFLAGS="-C llvm-args=-print-changed=quiet" cargo build -q --release --target-dir target/gecisler 2>&1 | grep -c "IR Dump After LoopInterchangePass" | xargs echo "n bilinirken LoopInterchangePass değişikliği:"
n bilinirken LoopInterchangePass değişikliği: 1
$ RUSTFLAGS="-C llvm-args=-print-changed=quiet" cargo build -q --release --target-dir target/gecisler 2>&1 | grep -c "IR Dump After LoopInterchangePass" | xargs echo "n gizlenince LoopInterchangePass değişikliği:"
n gizlenince LoopInterchangePass değişikliği: 0
$ RUSTFLAGS="-C llvm-args=-enable-loopinterchange=false" cargo run -q --release --target-dir target/donusumsuz | grep matris
matris satır-öncelikli gezinme         3.20 ms
matris sütun-öncelikli gezinme        98.70 ms
$ cargo run -q --release
sonuç atılınca      :      0.000 ms
black_box ile       :     32.332 ms
$ cargo build -q --release
$ hyperfine -N --warmup 3 --runs 30 --style basic --export-json target/olcum.json ./target/release/black_box
Benchmark 1: ./target/release/black_box
  Time (mean ± σ):      33.3 ms ±   1.0 ms    [User: 32.5 ms, System: 0.5 ms]
  Range (min … max):    32.2 ms …  36.2 ms    30 runs
 
$ python3 -c "import json; t=sorted(x*1000 for x in json.load(open(\"target/olcum.json\"))[\"results\"][0][\"times\"]); print(\" \".join(f\"{x:.1f}\" for x in t))"
32.2 32.2 32.2 32.3 32.3 32.4 32.4 32.7 32.7 32.7 32.8 33.0 33.1 33.1 33.2 33.3 33.3 33.3 33.3 33.3 33.4 33.6 33.6 33.8 34.2 34.7 34.7 34.8 35.3 36.2
```

## 104-profil-cikarma

- Video: `104-profil-cikarma`
- Örnek yolu: `deneyler/mufredat-ornekleri/104-profil-cikarma/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: valgrind süreç numarası (==NNNN==) ve toplam komut sayısındaki küçük oynama (birkaç yüz komut); yüzdeler aynı kalır
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/profil
# callgrind: her fonksiyonda yürütülen komut sayısı (zamanlamadan bağımsız, tekrarlanabilir)
ks 'valgrind --tool=callgrind --callgrind-out-file=/tmp/cg.out ./target/release/profil 2>&1 | grep -E "Collected|bulunan"'
# main'in çağırdıkları ve kapsayıcı (inclusive) maliyetleri; dosya yolları kısaltıldı
ks 'callgrind_annotate --inclusive=yes /tmp/cg.out 2>/dev/null | grep -E "PROGRAM TOTALS|=> .*profil::(yavas_arama|hizli_hazirlik|hizli_arama) " | sed -E "s#=> .*:(profil::[a-z_]+) #=> \1 #"'
# Kendi maliyeti (dahil olmayan görünüm): satır içine alınan kod çağıranın kendi maliyetinde görünür, sıralama ise çağrılan fonksiyonlarda
ks 'callgrind_annotate /tmp/cg.out 2>/dev/null | sed -n "/file:function/,+8p" | sed -E "s#/rustc/[0-9a-f]+/library/##; s#<u32, <u32 as core::cmp::PartialOrd>::lt>#<u32, …>#g; s# \[/work/[^]]*\]##"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/profil
bulunan: 2000 / 2000
$ valgrind --tool=callgrind --callgrind-out-file=/tmp/cg.out ./target/release/profil 2>&1 | grep -E "Collected|bulunan"
bulunan: 2000 / 2000
==30646== Collected : 24592551
$ callgrind_annotate --inclusive=yes /tmp/cg.out 2>/dev/null | grep -E "PROGRAM TOTALS|=> .*profil::(yavas_arama|hizli_hazirlik|hizli_arama) " | sed -E "s#=> .*:(profil::[a-z_]+) #=> \1 #"
24,592,551 (100.0%)  PROGRAM TOTALS
21,276,515 (86.52%)  => profil::yavas_arama (1x)
2,671,669 (10.86%)  => profil::hizli_hazirlik (1x)
292,013 ( 1.19%)  => profil::hizli_arama (1x)
$ callgrind_annotate /tmp/cg.out 2>/dev/null | sed -n "/file:function/,+8p" | sed -E "s#/rustc/[0-9a-f]+/library/##; s#<u32, <u32 as core::cmp::PartialOrd>::lt>#<u32, …>#g; s# \[/work/[^]]*\]##"
Ir                   file:function
--------------------------------------------------------------------------------
12,505,000 (50.85%)  core/src/slice/cmp.rs:profil::yavas_arama
 5,012,013 (20.38%)  core/src/slice/iter/macros.rs:profil::yavas_arama
 3,755,500 (15.27%)  core/src/slice/mod.rs:profil::yavas_arama
   619,513 ( 2.52%)  core/src/slice/sort/unstable/quicksort.rs:core::slice::sort::unstable::quicksort::quicksort::<u32, …>'2
   559,442 ( 2.27%)  core/src/ptr/mod.rs:core::slice::sort::unstable::quicksort::quicksort::<u32, …>'2
   423,948 ( 1.72%)  core/src/cmp.rs:core::slice::sort::unstable::quicksort::quicksort::<u32, …>'2
   219,935 ( 0.89%)  core/src/hint.rs:core::slice::sort::unstable::quicksort::quicksort::<u32, …>'2
```

## 105-dallanma-tahmini

- Video: `105-dallanma-tahmini`
- Örnek yolu: `deneyler/mufredat-ornekleri/105-dallanma-tahmini/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/dallanma
# Dallanan sürüm: döngünün içinde koşullu atlama (jns: işaret biti 0 ise atla → x < 128)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dallanan>:/,/^$/" | grep -E "\bj(ns|s)\b"'
# Dallanmasız sürüm: vektör karşılaştırma (pcmpgtb) + maske (pand); döngüde veriye bağlı atlama yok
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dallanmasiz>:/,/^$/" | grep -oE "\b(pcmpgtb|pand|paddq)\b" | sort | uniq -c'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/dallanma
dallanan,    karışık veri     282.4 ms   (toplam 9577481740)
dallanmasız, karışık veri      30.7 ms   (toplam 9577481740)
dallanan,    sıralı veri       34.2 ms   (toplam 9577481740)
dallanmasız, sıralı veri       30.2 ms   (toplam 9577481740)
$ objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dallanan>:/,/^$/" | grep -E "\bj(ns|s)\b"
   182ae:	jns    182a0 <dallanma::dallanan+0x20>
$ objdump -d -M intel --no-show-raw-insn -C target/release/dallanma | awk "/<dallanma::dallanmasiz>:/,/^$/" | grep -oE "\b(pcmpgtb|pand|paddq)\b" | sort | uniq -c
      4 paddq
      2 pand
      2 pcmpgtb
```

## 106-simd-ve-vektorlestirme

- Video: `106-simd-ve-vektorlestirme`
- Örnek yolu: `deneyler/mufredat-ornekleri/106-simd-ve-vektorlestirme/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k ./target/release/simd
# Hangi toplama komutları? addss: tek (skaler) f32, addps: 4'lü paket (SSE)
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_basit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c'
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c'
# f64 de skaler: addsd (tek f64); derleyici kayan nokta toplamının sırasını kendiliğinden değiştirmez
ks 'objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_basit_f64>:/,/^$/" | grep -oE "\b(addsd|addpd)\b" | sort | uniq -c'
# Bu işlemciye özel derleme: AVX2 ile 8'li paketler (vaddps ymm)
ks 'RUSTFLAGS="-C target-cpu=native" cargo build -q --release --target-dir target/native && ./target/native/release/simd'
ks 'objdump -d -M intel --no-show-raw-insn -C target/native/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -E "vaddps +ymm" | head -n 2'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ ./target/release/simd
f32, basit toplam        10.87 ms  sonuç 21960720
f32, 8 şeritli toplam     2.06 ms  sonuç 23999998
f64, basit toplam        11.05 ms  sonuç 23999997.5
i32, toplam               1.98 ms  sonuç 47999995
anlamlı bit: f32 24, f64 53; 23999997.5 sayısı f32'de 23999998 olur
$ objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_basit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c
      9 addss
$ objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -oE "\b(addss|addps|vaddps)\b" | sort | uniq -c
     10 addps
      9 addss
$ objdump -d -M intel --no-show-raw-insn -C target/release/simd | awk "/<simd::toplam_basit_f64>:/,/^$/" | grep -oE "\b(addsd|addpd)\b" | sort | uniq -c
      9 addsd
$ RUSTFLAGS="-C target-cpu=native" cargo build -q --release --target-dir target/native && ./target/native/release/simd
f32, basit toplam        10.85 ms  sonuç 21960720
f32, 8 şeritli toplam     1.68 ms  sonuç 23999998
f64, basit toplam        11.05 ms  sonuç 23999997.5
i32, toplam               1.75 ms  sonuç 47999995
anlamlı bit: f32 24, f64 53; 23999997.5 sayısı f32'de 23999998 olur
$ objdump -d -M intel --no-show-raw-insn -C target/native/release/simd | awk "/<simd::toplam_8_serit>:/,/^$/" | grep -E "vaddps +ymm" | head -n 2
   17a80:	vaddps ymm0,ymm0,YMMWORD PTR [rcx]
   17ad0:	vaddps ymm0,ymm0,YMMWORD PTR [rcx+rdx*4]
```

## 107-derleyici-ayarlari-lto-pgo

- Video: `107-derleyici-ayarlari-lto-pgo`
- Örnek yolu: `deneyler/mufredat-ornekleri/107-derleyici-ayarlari-lto-pgo/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `itoa 1.0.19`, `memchr 2.8.3`, `proc-macro2 1.0.107`, `quote 1.0.47`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `serde_json 1.0.154`, `syn 3.0.10`, `unicode-ident 1.0.27`, `zmij 1.0.23`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo build -q --release
k cargo build -q --profile kucuk
k cargo build -q --profile hizli
k cargo build -q --profile kucuk-semboller
ks 'stat -c "%8s bayt  %n" target/release/ayarlar target/kucuk/ayarlar target/kucuk-semboller/ayarlar target/hizli/ayarlar'
k hyperfine -N --warmup 3 --runs 30 --style basic ./target/release/ayarlar ./target/kucuk/ayarlar ./target/hizli/ayarlar
# PGO (profil güdümlü optimizasyon) için llvm-profdata gerekir: bu imajda kurulu mu?
ks 'rustup component list --installed | grep -c llvm-tools; command -v llvm-profdata || echo "llvm-profdata yok"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo build -q --release
$ cargo build -q --profile kucuk
$ cargo build -q --profile hizli
$ cargo build -q --profile kucuk-semboller
$ stat -c "%8s bayt  %n" target/release/ayarlar target/kucuk/ayarlar target/kucuk-semboller/ayarlar target/hizli/ayarlar
  544360 bayt  target/release/ayarlar
  318808 bayt  target/kucuk/ayarlar
  404624 bayt  target/kucuk-semboller/ayarlar
  440712 bayt  target/hizli/ayarlar
$ hyperfine -N --warmup 3 --runs 30 --style basic ./target/release/ayarlar ./target/kucuk/ayarlar ./target/hizli/ayarlar
Benchmark 1: ./target/release/ayarlar
  Time (mean ± σ):      15.0 ms ±   0.8 ms    [User: 14.3 ms, System: 0.5 ms]
  Range (min … max):    14.2 ms …  18.8 ms    30 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 2: ./target/kucuk/ayarlar
  Time (mean ± σ):      19.9 ms ±   0.8 ms    [User: 19.0 ms, System: 0.7 ms]
  Range (min … max):    19.0 ms …  23.0 ms    30 runs
 
Benchmark 3: ./target/hizli/ayarlar
  Time (mean ± σ):      12.8 ms ±   0.4 ms    [User: 12.2 ms, System: 0.4 ms]
  Range (min … max):    12.2 ms …  13.6 ms    30 runs
 
Summary
  ./target/hizli/ayarlar ran
    1.17 ± 0.08 times faster than ./target/release/ayarlar
    1.55 ± 0.08 times faster than ./target/kucuk/ayarlar
$ rustup component list --installed | grep -c llvm-tools; command -v llvm-profdata || echo "llvm-profdata yok"
0
llvm-profdata yok
```

## 108-tip-gudumlu-tasarim

- Video: `108-tip-gudumlu-tasarim`
- Örnek yolu: `deneyler/mufredat-ornekleri/108-tip-gudumlu-tasarim/`
- Kaynaklar: `durum-tipleri/Cargo.toml`, `durum-tipleri/src/main.rs`, `yanlis-gecis/Cargo.toml`, `yanlis-gecis/src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
boyutlar: Taslak 24, Onayli 24, Gonderildi 24, Vec 24 bayt
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
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
defter × 3: 200 sipariş alındı, kalan stok 9
defter × 50: 409 defter: 50 istendi, stokta 12 var
defter × 0: 400 adet en az 1 olmalı
silgi × 1: 500 geçici bir sorun oluştu, lütfen tekrar deneyin
    [günlük] altyapı hatası: No such file or directory (os error 2)
```

## 110-api-tasarimi

- Video: `110-api-tasarimi`
- Örnek yolu: `deneyler/mufredat-ornekleri/110-api-tasarimi/`
- Kaynaklar: `Cargo.toml`, `istemci/Cargo.toml`, `istemci/src/bin/duzgun.rs`, `istemci/src/main.rs`, `kutuphane/Cargo.toml`, `kutuphane/src/lib.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
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
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: iş parçacığı kimlikleri, test süreleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Ajanın kendi testleri geçiyor:
cd ajanin-yazdigi
ks 'cargo test -q ajanin_testleri 2>&1 | grep "test result"'
# İncelemede eklediğimiz sınır testleri:
ks 'cargo test -q inceleme_testleri -- --test-threads=1 2>&1 | grep -E "^---- |panicked|attempt|ParseIntError|test result"'
cd ../incelenmis
ks 'cargo test -q 2>&1 | grep "test result" | head -n 1'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q ajanin_testleri 2>&1 | grep "test result"
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
$ cargo test -q inceleme_testleri -- --test-threads=1 2>&1 | grep -E "^---- |panicked|attempt|ParseIntError|test result"
---- inceleme_testleri::bozuk_girdi stdout ----
thread 'inceleme_testleri::bozuk_girdi' (32528) panicked at src/lib.rs:4:54:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
---- inceleme_testleri::buyuk_fiyat stdout ----
thread 'inceleme_testleri::buyuk_fiyat' (32529) panicked at src/lib.rs:5:19:
attempt to multiply with overflow
---- inceleme_testleri::yuzden_fazla stdout ----
thread 'inceleme_testleri::yuzden_fazla' (32530) panicked at src/lib.rs:5:5:
attempt to subtract with overflow
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
$ cargo test -q 2>&1 | grep "test result" | head -n 1
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 114-inceleme-listesi

- Video: `114-inceleme-listesi`
- Örnek yolu: `deneyler/mufredat-ornekleri/114-inceleme-listesi/`
- Kaynaklar: `Cargo.toml`, `src/main.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
k cargo run -q
# Ajanın testi geçiyor (testte yalnız kısa kayıtlar var)
ks 'cargo test -q 2>&1 | grep "test result"'
# Varsayılan clippy ne diyor? (lint adları uyarıların yardım bağlantısından)
ks 'cargo clippy -q 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c'
# İnceleme için sıkı mod: pedantic + unwrap_used
ks 'cargo clippy -q -- -W clippy::pedantic -W clippy::unwrap_used 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c | sort -rn'
# Kendin dene: deneysel (nursery) grup da açılınca liste değişiyor mu?
ks 'cargo clippy -q -- -W clippy::pedantic -W clippy::unwrap_used -W clippy::nursery 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c | sort -rn'
ks 'cargo clippy -q -- -W clippy::nursery 2>&1 | grep -A3 "redundant clone"'
# Bütün lint grupları açıkken (all = correctness+suspicious+style+complexity+perf, ayrıca pedantic, restriction, nursery, cargo): 7. ve 10. satırlar
ks 'cargo clippy -q --message-format=short -- -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery -W clippy::cargo 2>&1 | grep -E "^src/main.rs:(7|10):" | sed -E "s/^src\/main.rs:([0-9]+):[0-9]+: warning: /satır \1: /" | sort | uniq'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo run -q
2 kayıt, toplam 48 bayt, ilk: ELMA
$ cargo test -q 2>&1 | grep "test result"
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
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
$ cargo clippy -q -- -W clippy::pedantic -W clippy::unwrap_used -W clippy::nursery 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c | sort -rn
      2 unwrap_used
      2 needless_pass_by_value
      1 redundant_clone
      1 missing_panics_doc
      1 iter_next_slice
      1 implicit_hasher
      1 cast_possible_truncation
      1 cast_lossless
$ cargo clippy -q -- -W clippy::nursery 2>&1 | grep -A3 "redundant clone"
warning: redundant clone
  --> src/main.rs:10:42
   |
10 |         onbellek.lock().unwrap().insert(k.clone(), uzunluk as usize);
$ cargo clippy -q --message-format=short -- -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery -W clippy::cargo 2>&1 | grep -E "^src/main.rs:(7|10):" | sed -E "s/^src\/main.rs:([0-9]+):[0-9]+: warning: /satır \1: /" | sort | uniq
satır 10: redundant clone: help: remove this
satır 10: used `unwrap()` on a `Result` value
satır 10: using a potentially dangerous silent `as` conversion
satır 7: this identifier consists of only a single character
```

## 115-araclarla-dogrulama

- Video: `115-araclarla-dogrulama`
- Örnek yolu: `deneyler/mufredat-ornekleri/115-araclarla-dogrulama/`
- Kaynaklar: `Cargo.toml`, `bariz/Cargo.toml`, `bariz/src/lib.rs`, `src/lib.rs`
- Toolchain: stable 1.100.0, nightly (miri / -Z bayrakları)
- Bağımlılıklar (Cargo.lock'ta çözülen): `autocfg 1.5.1`, `bit-set 0.11.1`, `bit-vec 0.10.2`, `bitflags 2.13.2`, `borsh 1.8.1`, `borsh-derive 1.8.1`, `bytes 1.12.2`, `cfg-if 1.0.5`, `cfg_aliases 0.2.2`, `chacha20 0.10.2`, `core_detect 1.0.0`, `cpufeatures 0.3.1`, `equivalent 1.0.3`, `errno 0.3.15`, `fastrand 2.5.0`, `fnv 1.0.7`, `getrandom 0.4.3`, `hashbrown 0.17.1`, `indexmap 2.14.2`, `libc 0.2.191`, `linux-raw-sys 0.12.1`, `memchr 2.8.3`, `num-traits 0.2.19`, `once_cell 1.21.4`, `proc-macro-crate 3.5.0`, `proc-macro2 1.0.107`, `proptest 1.12.0`, `quick-error 1.2.3`, `quote 1.0.47`, `r-efi 6.0.0`, `rand 0.10.4`, `rand_core 0.10.1`, `rand_xorshift 0.5.0`, `regex-syntax 0.8.12`, `rustix 1.1.6`, `rusty-fork 0.3.1`, `serde 1.0.229`, `serde_core 1.0.229`, `serde_derive 1.0.229`, `syn 3.0.10`, `tempfile 3.27.0`, `toml_datetime 1.1.3+spec-1.1.0`, `toml_edit 0.25.19+spec-1.1.0`, `toml_parser 1.1.5+spec-1.1.0`, `unarray 0.1.4`, `unicode-ident 1.0.27`, `wait-timeout 0.2.1`, `windows-link 0.2.1`, `windows-sys 0.61.2`, `winnow 1.0.5`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: Miri'nin etiket ve ayırma numaraları; süreler. proptest tohumu sabit (PROPTEST_RNG_SEED=1): karşı örnek herkeste aynı çıkar
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
rm -rf proptest-regressions
export PROPTEST_DISABLE_FAILURE_PERSISTENCE=1
# 0) Bariz hâli: derleyicinin varsayılan "deny" denetimi yakalar
cd bariz && k cargo build -q; cd ..
# 1) Dolaylı hâli derlenir; örnek testler geçer, farklılık (diferansiyel) testi patlar
ks 'PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep -E -A4 "minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"'
ks 'PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep "test result"'
# Kendin dene: başka bir tohumla küçültme başka (daha uzun) bir karşı örnekte durabilir
ks 'PROPTEST_RNG_SEED=2 cargo test -q medyan_basvuruyla 2>&1 | grep -E -A5 "minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"'
# Küçültme adımları (PROPTEST_VERBOSE=2, aynı tohum): her denenen girdi ve sonucu, sırayla
ks 'PROPTEST_RNG_SEED=1 PROPTEST_VERBOSE=2 cargo test -q medyan_basvuruyla 2>&1 | awk "/Next test input/ {g=\$0; sub(/.*v = /, \"\", g)} /Test case passed/ {print g \" → geçti\"} /Test case failed/ {print g \" → başarısız\"}"'
# Kendin dene: medyan_hizli'yi düzeltmek (girdi &[i32], select_nth_unstable &mut ister: önce kopya); özellik testi geçer
ks 'rm -rf /tmp/medyan-kopya && mkdir /tmp/medyan-kopya && cp -r Cargo.toml Cargo.lock src /tmp/medyan-kopya/ && cd /tmp/medyan-kopya && sed -i "s/    if v.is_empty() { None } else { Some(v\[v.len() \/ 2\]) }/    if v.is_empty() { None } else { let mut k = v.to_vec(); let orta = k.len() \/ 2; Some(*k.select_nth_unstable(orta).1) }/" src/lib.rs && grep -n "select_nth_unstable" src/lib.rs && PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep "test result" | head -n 1'
# 2) Miri: "geçen" sayaç testindeki tanımsız davranışı yakalar
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
$ PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep -E -A4 "minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"
minimal failing input: v = [
    53,
    0,
]
	successes: 0
$ PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep "test result"
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ PROPTEST_RNG_SEED=2 cargo test -q medyan_basvuruyla 2>&1 | grep -E -A5 "minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"
minimal failing input: v = [
    0,
    0,
    -1,
    0,
]
$ PROPTEST_RNG_SEED=1 PROPTEST_VERBOSE=2 cargo test -q medyan_basvuruyla 2>&1 | awk "/Next test input/ {g=\$0; sub(/.*v = /, \"\", g)} /Test case passed/ {print g \" → geçti\"} /Test case failed/ {print g \" → başarısız\"}"
[20, 79, 40, 52] → başarısız
[79, 40, 52] → başarısız
[40, 52] → geçti
[79, 40, 52] → başarısız
[79, 52] → başarısız
[79] → geçti
[79, 52] → başarısız
[39, 52] → geçti
[59, 52] → başarısız
[49, 52] → geçti
[54, 52] → başarısız
[52, 52] → geçti
[53, 52] → başarısız
[53, 26] → başarısız
[53, 13] → başarısız
[53, 6] → başarısız
[53, 3] → başarısız
[53, 1] → başarısız
[53, 0] → başarısız
$ rm -rf /tmp/medyan-kopya && mkdir /tmp/medyan-kopya && cp -r Cargo.toml Cargo.lock src /tmp/medyan-kopya/ && cd /tmp/medyan-kopya && sed -i "s/    if v.is_empty() { None } else { Some(v\[v.len() \/ 2\]) }/    if v.is_empty() { None } else { let mut k = v.to_vec(); let orta = k.len() \/ 2; Some(*k.select_nth_unstable(orta).1) }/" src/lib.rs && grep -n "select_nth_unstable" src/lib.rs && PROPTEST_RNG_SEED=1 cargo test -q 2>&1 | grep "test result" | head -n 1
14:    if v.is_empty() { None } else { let mut k = v.to_vec(); let orta = k.len() / 2; Some(*k.select_nth_unstable(orta).1) }
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
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
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: iş parçacığı kimlikleri, test süreleri
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Önce sözleşme: imza + belge + testler; gövde todo!()
cd 1-sozlesme && ks 'cargo test -q 2>&1 | grep -E "not yet implemented|test result" | sort | uniq -c'; cd ..
# Sonra gerçekleştirim (ajan ya da insan): aynı testler
cd 2-uygulama && ks 'cargo test -q 2>&1 | grep "test result" | head -n 1'; cd ..
# Kendin dene: sözleşmeye altıncı madde (genişlik 0 ise boş liste); bugünkü gerçekleştirim bu testi geçemez
ks 'rm -rf /tmp/sarmala-kopya && mkdir /tmp/sarmala-kopya && cp -r 2-uygulama/Cargo.toml 2-uygulama/Cargo.lock 2-uygulama/src /tmp/sarmala-kopya/ && cd /tmp/sarmala-kopya && sed -i "\$ d" src/lib.rs && printf "    #[test]\n    fn genislik_sifir_bos_liste() {\n        assert_eq!(sarmala(\"bir iki\", 0), Vec::<String>::new());\n    }\n}\n" >> src/lib.rs && cargo test -q 2>&1 | grep -E "^test result|left:|right:"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test -q 2>&1 | grep -E "not yet implemented|test result" | sort | uniq -c
      5 not yet implemented: henüz yazılmadı: önce sözleşme ve testler
      1 test result: FAILED. 0 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo test -q 2>&1 | grep "test result" | head -n 1
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ rm -rf /tmp/sarmala-kopya && mkdir /tmp/sarmala-kopya && cp -r 2-uygulama/Cargo.toml 2-uygulama/Cargo.lock 2-uygulama/src /tmp/sarmala-kopya/ && cd /tmp/sarmala-kopya && sed -i "\$ d" src/lib.rs && printf "    #[test]\n    fn genislik_sifir_bos_liste() {\n        assert_eq!(sarmala(\"bir iki\", 0), Vec::<String>::new());\n    }\n}\n" >> src/lib.rs && cargo test -q 2>&1 | grep -E "^test result|left:|right:"
  left: ["bir", "iki"]
 right: []
test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## 117-ambar-tasarim

- Video: `117-ambar-tasarim`
- Örnek yolu: `deneyler/mufredat-ornekleri/117-ambar-tasarim/`
- Kaynaklar: `ambar/Cargo.toml`, `ambar/src/bin/ambar.rs`, `ambar/src/gunluk.rs`, `ambar/src/lib.rs`, `ambar/src/motor.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: panik satırındaki iş parçacığı kimliği (parantez içindeki sayı) her çalıştırmada değişir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod ambar/ dizininde.
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep "^test "'
k cargo build -q --release
rm -rf calisma
# Her komut ayrı bir süreç: açılışta günlüğü baştan okuyup dizini yeniden kurar.
k ./target/release/ambar yaz calisma dil rust
k ./target/release/ambar yaz calisma yil 2015
k ./target/release/ambar yaz calisma dil Rust
k ./target/release/ambar sil calisma yil
k ./target/release/ambar yaz calisma şehir İzmir
k ./target/release/ambar listele calisma
k ./target/release/ambar oku calisma yil
# Günlüğün baytları (onaltılık; sağda ASCII)
k od -A d -t x1z -v calisma/gunluk.ambar
# Aynı beş komut bir kez daha: canlı durum aynı kalır, günlük büyür
echo "# aynı beş komut bir kez daha çalıştırıldı"
for satir in "yaz dil rust" "yaz yil 2015" "yaz dil Rust" "sil yil" "yaz şehir İzmir"; do
  set -- $satir
  ./target/release/ambar "$1" calisma "${@:2}"
done
k ./target/release/ambar listele calisma
# Bu adımın okuyucusu günlüğün sağlam olduğunu varsayar: son kaydın 3 baytı eksik olursa?
k truncate -s -3 calisma/gunluk.ambar
k ./target/release/ambar listele calisma
rm -rf calisma
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep "^test "
test gunluk::testler::silme_kaydi_uzunluk_alaninda_isaretlenir ... ok
test gunluk::testler::uzunluk_bayt_sayar_karakter_degil ... ok
test motor::testler::yeniden_acinca_ayni_durum_kurulur ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo build -q --release
$ ./target/release/ambar yaz calisma dil rust
$ ./target/release/ambar yaz calisma yil 2015
$ ./target/release/ambar yaz calisma dil Rust
$ ./target/release/ambar sil calisma yil
$ ./target/release/ambar yaz calisma şehir İzmir
$ ./target/release/ambar listele calisma
günlük: 5 kayıt, 76 bayt; canlı anahtar: 2
  dil = Rust
  şehir = İzmir
$ ./target/release/ambar oku calisma yil
yok
[çıkış kodu: 1]
$ od -A d -t x1z -v calisma/gunluk.ambar
0000000 03 00 00 00 04 00 00 00 64 69 6c 72 75 73 74 03  >........dilrust.<
0000016 00 00 00 04 00 00 00 79 69 6c 32 30 31 35 03 00  >.......yil2015..<
0000032 00 00 04 00 00 00 64 69 6c 52 75 73 74 03 00 00  >......dilRust...<
0000048 00 ff ff ff ff 79 69 6c 06 00 00 00 06 00 00 00  >.....yil........<
0000064 c5 9f 65 68 69 72 c4 b0 7a 6d 69 72              >..ehir..zmir<
0000076
# aynı beş komut bir kez daha çalıştırıldı
$ ./target/release/ambar listele calisma
günlük: 10 kayıt, 152 bayt; canlı anahtar: 2
  dil = Rust
  şehir = İzmir
$ truncate -s -3 calisma/gunluk.ambar
$ ./target/release/ambar listele calisma

thread 'main' (35149) panicked at src/gunluk.rs:53:29:
range end index 152 out of range for slice of length 149
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[çıkış kodu: 101]
```

## 118-ambar-depolama-motoru

- Video: `118-ambar-depolama-motoru`
- Örnek yolu: `deneyler/mufredat-ornekleri/118-ambar-depolama-motoru/`
- Kaynaklar: `ambar/Cargo.toml`, `ambar/src/bin/ambar.rs`, `ambar/src/crc.rs`, `ambar/src/gunluk.rs`, `ambar/src/lib.rs`, `ambar/src/motor.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod ambar/ dizininde (117'ye göre farkı: diff -ru).
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep "^test "'
k cargo build -q --release
# CRC-32'nin yayımlanmış denetim değeri ve bağımsız bir araçla (GNU cksum) karşılaştırma
k ./target/release/ambar crc 123456789
ks 'printf 123456789 | cksum -a crc32b'
k ./target/release/ambar crc 123456788
rm -rf calisma
k ./target/release/ambar doldur calisma 1000
ks 'stat -c "%s bayt" calisma/gunluk.ambar'
# 501. kaydın başlığı (18 890. bayttan 16 bayt): başlık CRC'si, anahtar uzunluğu (12), değer uzunluğu (10), veri CRC'si
k od -A d -t x1 -j 18890 -N 16 calisma/gunluk.ambar
# 1) Yarım yazma (torn write): son kaydın son 5 baytı diske hiç ulaşmamış gibi
k truncate -s -5 calisma/gunluk.ambar
k ./target/release/ambar durum calisma
ks 'stat -c "%s bayt" calisma/gunluk.ambar'
# 2) Sessiz bozulma: 501. kaydın anahtarında tek bir bayt değişsin
ks 'rm -rf calisma && ./target/release/ambar doldur calisma 1000'
ks 'printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18914 conv=notrunc status=none'
k ./target/release/ambar durum calisma
ks 'stat -c "%s bayt" calisma/gunluk.ambar'
# 3) Aynı bozulma 501. kaydın değer uzunluğu alanında: önce ikinci, sonra üçüncü baytı
ks 'rm -rf calisma && ./target/release/ambar doldur calisma 1000'
ks 'printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18899 conv=notrunc status=none'
k od -A d -t x1 -j 18890 -N 16 calisma/gunluk.ambar
k ./target/release/ambar durum calisma
ks 'rm -rf calisma && ./target/release/ambar doldur calisma 1000'
ks 'printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18900 conv=notrunc status=none'
k ./target/release/ambar durum calisma
ks 'stat -c "%s bayt" calisma/gunluk.ambar'
# Bozuk kayıttan itibaren her şeyi atmak operatörün açık kararı olmalı:
k ./target/release/ambar kirp calisma
k ./target/release/ambar durum calisma
rm -rf calisma
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep "^test "
test crc::testler::her_tek_bit_degisimi_crcyi_degistirir ... ok
test crc::testler::yayimlanmis_denetim_degeri ... ok
test gunluk::testler::kuyruktaki_yarim_kayit_kirpilabilir ... ok
test gunluk::testler::ortadaki_kaydin_hicbir_tek_bit_bozulmasi_kirpilmaz ... ok
test gunluk::testler::ortadaki_veri_bozulmasi_kuyruk_sayilmaz ... ok
test gunluk::testler::uzunluk_bozulmasi_baslik_crcsine_takilir ... ok
test motor::testler::ortadaki_bozulmada_acilmaz_ve_dosyaya_dokunmaz ... ok
test motor::testler::yeniden_acinca_ayni_durum_kurulur ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
$ cargo build -q --release
$ ./target/release/ambar crc 123456789
cbf43926 (3421780262)
$ printf 123456789 | cksum -a crc32b
3421780262 9
$ ./target/release/ambar crc 123456788
bcf309b0 (3170044336)
$ ./target/release/ambar doldur calisma 1000
1000 kayıt yazıldı
$ stat -c "%s bayt" calisma/gunluk.ambar
37890 bayt
$ od -A d -t x1 -j 18890 -N 16 calisma/gunluk.ambar
0018890 42 2a 30 5d 0c 00 00 00 0a 00 00 00 1f 97 81 31
0018906
$ truncate -s -5 calisma/gunluk.ambar
$ ./target/release/ambar durum calisma
geçerli kayıt: 999 (37852 bayt); kırpılan kuyruk: 33 bayt
kurtarma notu: 37852. baytta yarım kayıt (33/38 bayt; başlık sağlam)
canlı anahtar: 999
$ stat -c "%s bayt" calisma/gunluk.ambar
37852 bayt
$ rm -rf calisma && ./target/release/ambar doldur calisma 1000
1000 kayıt yazıldı
$ printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18914 conv=notrunc status=none
$ ./target/release/ambar durum calisma
hata: günlük bozuk: 18890. baytta veri CRC'si tutmuyor; arkasında 18962 bayt daha var; dosyaya dokunulmadı
[çıkış kodu: 1]
$ stat -c "%s bayt" calisma/gunluk.ambar
37890 bayt
$ rm -rf calisma && ./target/release/ambar doldur calisma 1000
1000 kayıt yazıldı
$ printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18899 conv=notrunc status=none
$ od -A d -t x1 -j 18890 -N 16 calisma/gunluk.ambar
0018890 42 2a 30 5d 0c 00 00 00 0a 58 00 00 1f 97 81 31
0018906
$ ./target/release/ambar durum calisma
hata: günlük bozuk: 18890. baytta başlık CRC'si tutmuyor (uzunluk alanları bozuk); dosyada kalan 19000 bayt; dosyaya dokunulmadı
[çıkış kodu: 1]
$ rm -rf calisma && ./target/release/ambar doldur calisma 1000
1000 kayıt yazıldı
$ printf "X" | dd of=calisma/gunluk.ambar bs=1 seek=18900 conv=notrunc status=none
$ ./target/release/ambar durum calisma
hata: günlük bozuk: 18890. baytta başlık CRC'si tutmuyor (uzunluk alanları bozuk); dosyada kalan 19000 bayt; dosyaya dokunulmadı
[çıkış kodu: 1]
$ stat -c "%s bayt" calisma/gunluk.ambar
37890 bayt
$ ./target/release/ambar kirp calisma
19000 bayt atıldı; kalan kayıt: 500
$ ./target/release/ambar durum calisma
geçerli kayıt: 500 (18890 bayt); kırpılan kuyruk: 0 bayt
canlı anahtar: 500
```

## 119-ambar-ag-protokolu

- Video: `119-ambar-ag-protokolu`
- Örnek yolu: `deneyler/mufredat-ornekleri/119-ambar-ag-protokolu/`
- Kaynaklar: `ambar/Cargo.toml`, `ambar/src/bin/ambar.rs`, `ambar/src/crc.rs`, `ambar/src/gunluk.rs`, `ambar/src/lib.rs`, `ambar/src/motor.rs`, `ambar/src/protokol.rs`, `ambar/src/sunucu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod ambar/ dizininde (118'e göre farkı: diff -ru).
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
# Dokümantasyon testi: motoru kilitsiz paylaşan kod derlenmemeli (compile_fail); derleyicinin sözü:
ks 'cargo test --doc -- --nocapture 2>&1 | grep -E "^error|^ *--> |^ *= help|^test "'
k cargo build -q --release
rm -rf calisma
./target/release/ambar sunucu calisma 127.0.0.1:7001 2> sunucu.log &
SP=$!
sleep 0.5
k ./target/release/ambar istemci 127.0.0.1:7001 "yaz dil rust" "oku dil" "yaz selam merhaba dünya" "oku selam" "sil dil" "oku dil" "say" "uçur hepsini"
# Protokol düz metin: özel bir istemci olmadan, bash'in /dev/tcp yönlendirmesiyle de konuşulur
ks 'exec 3<>/dev/tcp/127.0.0.1/7001; printf "oku selam\nsay\n" >&3; head -n 2 <&3'
# Çerçeveyi satır sonu belirler: değerin içindeki "\n" isteği ikiye böler
ks 'exec 3<>/dev/tcp/127.0.0.1/7001; printf "yaz not birinci satır\nikinci satır\n" >&3; head -n 2 <&3'
# Satır sınırı: hiç "\n" göndermeden tam 1 049 616 bayt (anahtar 1 KiB + değer 1 MiB + 16)
ks 'exec 3<>/dev/tcp/127.0.0.1/7001; head -c 1049616 /dev/zero | tr "\0" a >&3; head -n 1 <&3'
kill $SP
k cat sunucu.log
# Sunucu durdu; veri günlükte. Yeni bir süreç aynı durumu kurar:
k ./target/release/ambar listele calisma
rm -rf calisma sunucu.log
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test crc::testler::her_tek_bit_degisimi_crcyi_degistirir ... ok
test crc::testler::yayimlanmis_denetim_degeri ... ok
test gunluk::testler::kuyruktaki_yarim_kayit_kirpilabilir ... ok
test gunluk::testler::ortadaki_kaydin_hicbir_tek_bit_bozulmasi_kirpilmaz ... ok
test gunluk::testler::ortadaki_veri_bozulmasi_kuyruk_sayilmaz ... ok
test gunluk::testler::uzunluk_bozulmasi_baslik_crcsine_takilir ... ok
test motor::testler::ortadaki_bozulmada_acilmaz_ve_dosyaya_dokunmaz ... ok
test motor::testler::yeniden_acinca_ayni_durum_kurulur ... ok
test protokol::testler::degerde_bosluk_olabilir ... ok
test protokol::testler::eksik_arguman_ve_bilinmeyen_komut ... ok
test protokol::testler::komut_adi_ascii_harf_duyarsiz ... ok
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
$ cargo test --doc -- --nocapture 2>&1 | grep -E "^error|^ *--> |^ *= help|^test "
error[E0596]: cannot borrow data in an `Arc` as mutable
  --> src/sunucu.rs:11:28
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<Motor>`
error: aborting due to 1 previous error
test src/sunucu.rs - sunucu (line 5) - compile fail ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
$ cargo build -q --release
$ ./target/release/ambar istemci 127.0.0.1:7001 "yaz dil rust" "oku dil" "yaz selam merhaba dünya" "oku selam" "sil dil" "oku dil" say "uçur hepsini"
> yaz dil rust
< tamam
> oku dil
< değer rust
> yaz selam merhaba dünya
< tamam
> oku selam
< değer merhaba dünya
> sil dil
< tamam
> oku dil
< yok
> say
< sayı 1
> uçur hepsini
< hata: bilinmeyen komut: "uçur hepsini"
$ exec 3<>/dev/tcp/127.0.0.1/7001; printf "oku selam\nsay\n" >&3; head -n 2 <&3
değer merhaba dünya
sayı 1
$ exec 3<>/dev/tcp/127.0.0.1/7001; printf "yaz not birinci satır\nikinci satır\n" >&3; head -n 2 <&3
tamam
hata: bilinmeyen komut: "ikinci satır"
$ exec 3<>/dev/tcp/127.0.0.1/7001; head -c 1049616 /dev/zero | tr "\0" a >&3; head -n 1 <&3
hata: satır çok uzun
$ cat sunucu.log
günlükten 0 kayıt okundu
ambar dinliyor: 127.0.0.1:7001
$ ./target/release/ambar listele calisma
günlük: 4 kayıt; canlı anahtar: 2
  not = birinci satır
  selam = merhaba dünya
```

## 120-ambar-eszamanlilik-ve-dayaniklilik

- Video: `120-ambar-eszamanlilik-ve-dayaniklilik`
- Örnek yolu: `deneyler/mufredat-ornekleri/120-ambar-eszamanlilik-ve-dayaniklilik/`
- Kaynaklar: `ambar/Cargo.toml`, `ambar/src/bin/ambar.rs`, `ambar/src/crc.rs`, `ambar/src/gunluk.rs`, `ambar/src/lib.rs`, `ambar/src/motor.rs`, `ambar/src/protokol.rs`, `ambar/src/sunucu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süre ve yazma/s; onaylanan ve kurtarılan kayıt sayıları (sunucunun öldürüldüğü ana bağlı)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod ambar/ dizininde (119'a göre farkı: diff -ru).
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
# 1) 8 istemci aynı anda yazıyor: tek bir yazma bile kayboluyor mu?
./target/release/ambar sunucu calisma/eszamanli 127.0.0.1:7002 yok 2>/dev/null &
SP=$!
sleep 0.5
k ./target/release/ambar yuk 127.0.0.1:7002 8 2000
kill $SP
k ./target/release/ambar durum calisma/eszamanli
# 2) Çökme deneyi: sunucu "her yazmada fsync" ile çalışırken SIGKILL (kill -9)
./target/release/ambar sunucu calisma/cokme 127.0.0.1:7003 her 2>/dev/null &
SP=$!
sleep 0.5
./target/release/ambar yazici 127.0.0.1:7003 100000000 > calisma/yazici.txt &
YP=$!
sleep 2
{ kill -9 $SP; wait $SP; } 2>/dev/null
wait $YP
k cat calisma/yazici.txt
k ./target/release/ambar durum calisma/cokme
# 3) Aynı deney fsync olmadan: süreç ölür, çekirdeğin sayfa önbelleği yaşar
./target/release/ambar sunucu calisma/cokme_yok 127.0.0.1:7004 yok 2>/dev/null &
SP=$!
sleep 0.5
./target/release/ambar yazici 127.0.0.1:7004 100000000 > calisma/yazici_yok.txt &
YP=$!
sleep 2
{ kill -9 $SP; wait $SP; } 2>/dev/null
wait $YP
k cat calisma/yazici_yok.txt
k ./target/release/ambar durum calisma/cokme_yok
rm -rf calisma
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test crc::testler::her_tek_bit_degisimi_crcyi_degistirir ... ok
test crc::testler::yayimlanmis_denetim_degeri ... ok
test gunluk::testler::kuyruktaki_yarim_kayit_kirpilabilir ... ok
test gunluk::testler::ortadaki_kaydin_hicbir_tek_bit_bozulmasi_kirpilmaz ... ok
test gunluk::testler::ortadaki_veri_bozulmasi_kuyruk_sayilmaz ... ok
test gunluk::testler::uzunluk_bozulmasi_baslik_crcsine_takilir ... ok
test motor::testler::gunluk_sirasi_indeks_sirasiyla_ayni ... ok
test motor::testler::yeniden_acinca_ayni_durum_kurulur ... ok
test protokol::testler::degerde_bosluk_olabilir ... ok
test protokol::testler::eksik_arguman_ve_bilinmeyen_komut ... ok
test protokol::testler::komut_adi_ascii_harf_duyarsiz ... ok
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s
$ cargo build -q --release
$ ./target/release/ambar yuk 127.0.0.1:7002 8 2000
8 istemci × 2000 yazma = 16000 yazma, 0.16 s, 100020 yazma/s; sunucu: sayı 16000
$ ./target/release/ambar durum calisma/eszamanli
geçerli kayıt: 16000 (455120 bayt); kırpılan kuyruk: 0 bayt
canlı anahtar: 16000
$ cat calisma/yazici.txt
onaylanan yazma: 1895
$ ./target/release/ambar durum calisma/cokme
geçerli kayıt: 1896 (53874 bayt); kırpılan kuyruk: 0 bayt
canlı anahtar: 1896
$ cat calisma/yazici_yok.txt
onaylanan yazma: 96465
$ ./target/release/ambar durum calisma/cokme_yok
geçerli kayıt: 96465 (2882840 bayt); kırpılan kuyruk: 0 bayt
canlı anahtar: 96465
```

## 121-ambar-performans

- Video: `121-ambar-performans`
- Örnek yolu: `deneyler/mufredat-ornekleri/121-ambar-performans/`
- Kaynaklar: `ambar/Cargo.toml`, `ambar/src/bin/ambar.rs`, `ambar/src/crc.rs`, `ambar/src/gunluk.rs`, `ambar/src/lib.rs`, `ambar/src/motor.rs`, `ambar/src/protokol.rs`, `ambar/src/sunucu.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: yazma/s, gecikme ve fsync sayıları diske, yüke ve zamanlamaya bağlı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod (Ambar'ın bitmiş hâli) ambar/ dizininde (120'ye göre farkı: diff -ru).
cd ambar
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
rm -rf calisma && mkdir -p calisma
# Veri nerede? /work ana makinenin diskine bağlı (btrfs)
ks 'grep " /work " /proc/mounts | cut -d" " -f1-3'
port=7010
for kip in her toplu yok; do
  port=$((port + 1))
  ./target/release/ambar sunucu calisma/p_$kip 127.0.0.1:$port $kip 2>/dev/null &
  SP=$!
  sleep 0.5
  echo "# fsync kipi: $kip (veri: /work, ana makinenin diski)"
  k ./target/release/ambar yuk 127.0.0.1:$port 16 300
  kill $SP
done
# Tuzak: aynı "her" kipi konteynerin kendi kök dosya sisteminde (/tmp). Kök nasıl bağlı?
ks 'grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"'
./target/release/ambar sunucu /tmp/ambar_her 127.0.0.1:7019 her 2>/dev/null &
SP=$!
sleep 0.5
echo "# fsync kipi: her (veri: konteynerin /tmp'si)"
k ./target/release/ambar yuk 127.0.0.1:7019 16 300
kill $SP
rm -rf calisma /tmp/ambar_her
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test crc::testler::her_tek_bit_degisimi_crcyi_degistirir ... ok
test crc::testler::yayimlanmis_denetim_degeri ... ok
test gunluk::testler::kuyruktaki_yarim_kayit_kirpilabilir ... ok
test gunluk::testler::ortadaki_kaydin_hicbir_tek_bit_bozulmasi_kirpilmaz ... ok
test gunluk::testler::ortadaki_veri_bozulmasi_kuyruk_sayilmaz ... ok
test gunluk::testler::uzunluk_bozulmasi_baslik_crcsine_takilir ... ok
test motor::testler::gunluk_sirasi_indeks_sirasiyla_ayni ... ok
test motor::testler::toplu_kipte_onay_kalicilikten_sonra_gelir ... ok
test motor::testler::yeniden_acinca_ayni_durum_kurulur ... ok
test protokol::testler::degerde_bosluk_olabilir ... ok
test protokol::testler::eksik_arguman_ve_bilinmeyen_komut ... ok
test protokol::testler::komut_adi_ascii_harf_duyarsiz ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.63s
$ cargo build -q --release
$ grep " /work " /proc/mounts | cut -d" " -f1-3
/dev/mapper/root /work btrfs
# fsync kipi: her (veri: /work, ana makinenin diski)
$ ./target/release/ambar yuk 127.0.0.1:7011 16 300
16 istemci × 300 yazma = 4800 yazma, 4.82 s, 996 yazma/s
  gecikme: ortalama 16.03 ms, ortanca 16.03 ms, %99 21.69 ms; sunucu: sayı 4800, yazma 4800 fsync 4800
# fsync kipi: toplu (veri: /work, ana makinenin diski)
$ ./target/release/ambar yuk 127.0.0.1:7012 16 300
16 istemci × 300 yazma = 4800 yazma, 0.95 s, 5050 yazma/s
  gecikme: ortalama 3.17 ms, ortanca 3.12 ms, %99 4.14 ms; sunucu: sayı 4800, yazma 4800 fsync 300
# fsync kipi: yok (veri: /work, ana makinenin diski)
$ ./target/release/ambar yuk 127.0.0.1:7013 16 300
16 istemci × 300 yazma = 4800 yazma, 0.05 s, 92635 yazma/s
  gecikme: ortalama 0.17 ms, ortanca 0.15 ms, %99 0.75 ms; sunucu: sayı 4800, yazma 4800 fsync 0
$ grep " / " /proc/mounts | grep -oE "^overlay|fsync=[a-z]+"
overlay
fsync=volatile
# fsync kipi: her (veri: konteynerin /tmp'si)
$ ./target/release/ambar yuk 127.0.0.1:7019 16 300
16 istemci × 300 yazma = 4800 yazma, 0.04 s, 119279 yazma/s
  gecikme: ortalama 0.13 ms, ortanca 0.12 ms, %99 0.58 ms; sunucu: sayı 4800, yazma 4800 fsync 4800
```

## 122-http-ayristirici

- Video: `122-http-ayristirici`
- Örnek yolu: `deneyler/mufredat-ornekleri/122-http-ayristirici/`
- Kaynaklar: `kapi/Cargo.toml`, `kapi/src/ayristirici.rs`, `kapi/src/bin/kapi.rs`, `kapi/src/lib.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: tarama ölçümündeki süreler ve oran
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod kapi/ dizininde.
cd kapi
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
# Bir istek dört parça hâlinde geliyor; boş satır ("\r\n\r\n") son iki parçaya bölünmüş
k ./target/release/kapi parca-parca 'GET /belgeler/not' '.txt HTTP/1.1\r\nHo' 'st: kapi\r\n\r' '\n'
# Gövdeli istek: başlıklar bitince durum Govde'ye geçer, Content-Length kadar bayt beklenir
k ./target/release/kapi parca-parca 'POST /kayit HTTP/1.1\r\nContent-Length: 5\r\n\r\nmer' 'ha'
# Bozuk istek satırı
k ./target/release/kapi parca-parca 'MERHABA\r\n\r\n'
# Taramanın maliyeti: 8 KiB'a yakın bir başlık bayt bayt geliyor
k ./target/release/kapi tarama-olc
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test ayristirici::testler::ad_ile_iki_nokta_arasinda_bosluk_reddedilir ... ok
test ayristirici::testler::bayt_bayt_beslemek_ayni_sonucu_verir ... ok
test ayristirici::testler::bozuk_istek_satiri ... ok
test ayristirici::testler::content_length_yalniz_rakam ... ok
test ayristirici::testler::sinirsiz_baslik_reddedilir ... ok
test ayristirici::testler::tek_parcada_iki_istek ... ok
test ayristirici::testler::transfer_encoding_desteklenmiyor ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo build -q --release
$ ./target/release/kapi parca-parca "GET /belgeler/not" ".txt HTTP/1.1\r\nHo" "st: kapi\r\n\r" "\n"
besle("GET /belgeler/not") → eksik; tamponda 17 bayt, durum: Basliklar
besle(".txt HTTP/1.1\r\nHo") → eksik; tamponda 34 bayt, durum: Basliklar
besle("st: kapi\r\n\r") → eksik; tamponda 45 bayt, durum: Basliklar
besle("\n") → istek: GET /belgeler/not.txt HTTP/1.1, başlıklar: [("Host", "kapi")], gövde: 0 bayt
$ ./target/release/kapi parca-parca "POST /kayit HTTP/1.1\r\nContent-Length: 5\r\n\r\nmer" ha
besle("POST /kayit HTTP/1.1\r\nContent-Length: 5\r\n\r\nmer") → eksik; tamponda 3 bayt, durum: Govde { uzunluk: 5 }
besle("ha") → istek: POST /kayit HTTP/1.1, başlıklar: [("Content-Length", "5")], gövde: 5 bayt
$ ./target/release/kapi parca-parca "MERHABA\r\n\r\n"
besle("MERHABA\r\n\r\n") → hata: BozukIstekSatiri("MERHABA")
[çıkış kodu: 1]
$ ./target/release/kapi tarama-olc
8019 baytlık başlık bayt bayt: kaldığı yerden arama 0.144 ms, her baytta baştan arama 7.699 ms (54 kat)
```

## 123-http-sunucu

- Video: `123-http-sunucu`
- Örnek yolu: `deneyler/mufredat-ornekleri/123-http-sunucu/`
- Kaynaklar: `kapi/Cargo.toml`, `kapi/src/ayristirici.rs`, `kapi/src/bin/kapi.rs`, `kapi/src/dosya.rs`, `kapi/src/havuz.rs`, `kapi/src/lib.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod kapi/ dizininde (122'ye göre farkı: diff -ru).
cd kapi
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
./target/release/kapi havuz 127.0.0.1:8101 kok 8 2> sunucu.log &
SP=$!
sleep 0.5
ks 'curl -s -i http://127.0.0.1:8101/'
# HEAD: aynı başlıklar, gövde yok
ks 'curl -s -I http://127.0.0.1:8101/belgeler/not.txt'
ks 'curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/yok.html'
# Dizin geçişi denemesi: curl yolu "düzeltmesin" diye --path-as-is
ks 'curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/../../etc/passwd"'
# Yüzde kodlu ".." (%2e%2e): curl yolu kendisi düzeltip gönderir; --path-as-is ile olduğu gibi gider
ks 'curl -s -v -o /dev/null "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd" 2>&1 | grep "^> GET"'
ks 'curl -s -v --path-as-is -o /dev/null "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd" 2>&1 | grep "^> GET"'
ks 'curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd"'
# Bileşen denetiminin sınırı: kökün içindeki bir sembolik bağ dışarıyı gösterirse
k ln -s /etc kok/disari
ks 'curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/disari/os-release'
ks 'curl -s http://127.0.0.1:8101/disari/os-release | head -n 1'
k rm kok/disari
ks 'curl -s -i -X POST http://127.0.0.1:8101/'
# Kalıcı bağlantı: aynı TCP bağlantısında art arda iki istek, iki yanıt
ks 'exec 3<>/dev/tcp/127.0.0.1/8101; printf "GET /belgeler/not.txt HTTP/1.1\r\nHost: kapi\r\n\r\nGET /belgeler/not.txt HTTP/1.1\r\nHost: kapi\r\nConnection: close\r\n\r\n" >&3; cat <&3'
# Bozuk istek satırı
ks 'exec 3<>/dev/tcp/127.0.0.1/8101; printf "MERHABA\r\n\r\n" >&3; cat <&3'
kill $SP
k cat sunucu.log
rm -f sunucu.log kok/disari
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test ayristirici::testler::ad_ile_iki_nokta_arasinda_bosluk_reddedilir ... ok
test ayristirici::testler::bayt_bayt_beslemek_ayni_sonucu_verir ... ok
test ayristirici::testler::bozuk_istek_satiri ... ok
test ayristirici::testler::content_length_yalniz_rakam ... ok
test ayristirici::testler::sinirsiz_baslik_reddedilir ... ok
test ayristirici::testler::tek_parcada_iki_istek ... ok
test ayristirici::testler::transfer_encoding_desteklenmiyor ... ok
test dosya::testler::content_length_bayt_sayar ... ok
test dosya::testler::kokun_disina_cikan_yollar_reddedilir ... ok
test dosya::testler::kokun_icindeki_yollar ... ok
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo build -q --release
$ curl -s -i http://127.0.0.1:8101/
HTTP/1.1 200 OK
Content-Length: 23
Content-Type: text/html; charset=utf-8

<h1>Merhaba Kapı</h1>
$ curl -s -I http://127.0.0.1:8101/belgeler/not.txt
HTTP/1.1 200 OK
Content-Length: 18
Content-Type: text/plain; charset=utf-8

$ curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/yok.html
404
$ curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/../../etc/passwd"
403
$ curl -s -v -o /dev/null "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd" 2>&1 | grep "^> GET"
> GET /etc/passwd HTTP/1.1
$ curl -s -v --path-as-is -o /dev/null "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd" 2>&1 | grep "^> GET"
> GET /%2e%2e/%2e%2e/etc/passwd HTTP/1.1
$ curl -s --path-as-is -o /dev/null -w "%{http_code}\n" "http://127.0.0.1:8101/%2e%2e/%2e%2e/etc/passwd"
404
$ ln -s /etc kok/disari
$ curl -s -o /dev/null -w "%{http_code}\n" http://127.0.0.1:8101/disari/os-release
200
$ curl -s http://127.0.0.1:8101/disari/os-release | head -n 1
PRETTY_NAME="Debian GNU/Linux 13 (trixie)"
$ rm kok/disari
$ curl -s -i -X POST http://127.0.0.1:8101/
HTTP/1.1 405 Method Not Allowed
Content-Length: 21
Content-Type: text/plain; charset=utf-8
Allow: GET, HEAD

yalnizca GET ve HEAD
$ exec 3<>/dev/tcp/127.0.0.1/8101; printf "GET /belgeler/not.txt HTTP/1.1\r\nHost: kapi\r\n\r\nGET /belgeler/not.txt HTTP/1.1\r\nHost: kapi\r\nConnection: close\r\n\r\n" >&3; cat <&3
HTTP/1.1 200 OK
Content-Length: 18
Content-Type: text/plain; charset=utf-8

gizli olmayan not
HTTP/1.1 200 OK
Content-Length: 18
Content-Type: text/plain; charset=utf-8
Connection: close

gizli olmayan not
$ exec 3<>/dev/tcp/127.0.0.1/8101; printf "MERHABA\r\n\r\n" >&3; cat <&3
HTTP/1.1 400 Bad Request
Content-Length: 28
Content-Type: text/plain; charset=utf-8
Connection: close

BozukIstekSatiri("MERHABA")
$ cat sunucu.log
havuz sunucusu 8 işçiyle dinliyor: 127.0.0.1:8101
```

## 124-http-eszamanlilik-ve-yuk

- Video: `124-http-eszamanlilik-ve-yuk`
- Örnek yolu: `deneyler/mufredat-ornekleri/124-http-eszamanlilik-ve-yuk/`
- Kaynaklar: `kapi/Cargo.toml`, `kapi/src/ayristirici.rs`, `kapi/src/bin/kapi.rs`, `kapi/src/dosya.rs`, `kapi/src/havuz.rs`, `kapi/src/lib.rs`, `kapi/src/zaman_uyumsuz.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: istek/s, bağlantı süreleri ve yanıt süreleri (makineye ve zamanlamaya bağlı)
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod (Kapı'nın bitmiş hâli) kapi/ dizininde (123'e göre farkı: diff -ru).
cd kapi
ks 'cargo test --lib 2>&1 | grep -E "^test result"'
k cargo build -q --release
./target/release/kapi havuz 127.0.0.1:8101 kok 8 2>/dev/null &
P1=$!
./target/release/kapi async 127.0.0.1:8102 kok 2>/dev/null &
P2=$!
sleep 0.5
echo "# 8 işçili havuz:"
k ./target/release/kapi yuk 127.0.0.1:8101 100 200
echo "# tokio (her bağlantıya bir görev):"
k ./target/release/kapi yuk 127.0.0.1:8102 100 200
echo "# 8 yavaş istemci (başlığı bitirmeyen) varken; sunucularda son süre yok:"
k ./target/release/kapi yavas 127.0.0.1:8101 8
k ./target/release/kapi yavas 127.0.0.1:8102 8
kill $P1 $P2
echo "# aynı deneme, sunucularda 1000 ms istek son süresiyle:"
./target/release/kapi havuz 127.0.0.1:8103 kok 8 1000 2>/dev/null &
P1=$!
./target/release/kapi async 127.0.0.1:8104 kok 1000 2>/dev/null &
P2=$!
sleep 0.5
k ./target/release/kapi yavas 127.0.0.1:8103 8
k ./target/release/kapi yavas 127.0.0.1:8104 8
kill $P1 $P2
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib 2>&1 | grep -E "^test result"
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo build -q --release
# 8 işçili havuz:
$ ./target/release/kapi yuk 127.0.0.1:8101 100 200
100 bağlantı × 200 istek: 244608 istek/s; bağlantının bitme süresi en kısa 6 ms, ortanca 45 ms, en uzun 77 ms
# tokio (her bağlantıya bir görev):
$ ./target/release/kapi yuk 127.0.0.1:8102 100 200
100 bağlantı × 200 istek: 304796 istek/s; bağlantının bitme süresi en kısa 32 ms, ortanca 57 ms, en uzun 62 ms
# 8 yavaş istemci (başlığı bitirmeyen) varken; sunucularda son süre yok:
$ ./target/release/kapi yavas 127.0.0.1:8101 8
8 yavaş bağlantı açıkken normal bir istek: 3006 ms → yanıt yok (Resource temporarily unavailable (os error 11))
  yavaş bağlantılardan 0/8 tanesi sunucu tarafından 408 ile kapatıldı
$ ./target/release/kapi yavas 127.0.0.1:8102 8
8 yavaş bağlantı açıkken normal bir istek: 0 ms → HTTP/1.1 200 OK
  yavaş bağlantılardan 0/8 tanesi sunucu tarafından 408 ile kapatıldı
# aynı deneme, sunucularda 1000 ms istek son süresiyle:
$ ./target/release/kapi yavas 127.0.0.1:8103 8
8 yavaş bağlantı açıkken normal bir istek: 717 ms → HTTP/1.1 200 OK
  yavaş bağlantılardan 8/8 tanesi sunucu tarafından 408 ile kapatıldı
$ ./target/release/kapi yavas 127.0.0.1:8104 8
8 yavaş bağlantı açıkken normal bir istek: 0 ms → HTTP/1.1 200 OK
  yavaş bağlantılardan 8/8 tanesi sunucu tarafından 408 ile kapatıldı
```

## 125-yorumlayici-lexer

- Video: `125-yorumlayici-lexer`
- Örnek yolu: `deneyler/mufredat-ornekleri/125-yorumlayici-lexer/`
- Kaynaklar: `minik/Cargo.toml`, `minik/src/bin/minik.rs`, `minik/src/lexer.rs`, `minik/src/lib.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod minik/ dizininde.
cd minik
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
k cat ornekler/ifade.mk
k ./target/release/minik token ornekler/ifade.mk
# En uzun eşleşme, Türkçe harfli ad, yorum ve karakter sayan sütunlar
k cat ornekler/esitlik.mk
k ./target/release/minik token ornekler/esitlik.mk
k ./target/release/minik token ornekler/garip_karakter.mk
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test lexer::testler::anahtar_sozcuk_once_ad_gibi_okunur ... ok
test lexer::testler::en_uzun_eslesme ... ok
test lexer::testler::sutun_karakter_sayar_bayt_degil ... ok
test lexer::testler::yorum_atlanir_ve_hata_konumu_bildirilir ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
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
$ cat ornekler/esitlik.mk
let sayı = 10; // yorumlar atlanır
print(sayı >= 10 == true);
$ ./target/release/minik token ornekler/esitlik.mk
 1:1  Let
 1:5  Ad("sayı")
 1:10 Esit
 1:12 Sayi(10)
 1:14 NoktaliVirgul
 2:1  Print
 2:6  SolParantez
 2:7  Ad("sayı")
 2:12 BuyukEsit
 2:15 Sayi(10)
 2:18 EsitEsit
 2:21 True
 2:25 SagParantez
 2:26 NoktaliVirgul
 3:1  Son
$ ./target/release/minik token ornekler/garip_karakter.mk
hata: satır 1, sütun 11: beklenmeyen karakter '@'
[çıkış kodu: 1]
```

## 126-yorumlayici-parser

- Video: `126-yorumlayici-parser`
- Örnek yolu: `deneyler/mufredat-ornekleri/126-yorumlayici-parser/`
- Kaynaklar: `minik/Cargo.toml`, `minik/src/ast.rs`, `minik/src/bin/minik.rs`, `minik/src/lexer.rs`, `minik/src/lib.rs`, `minik/src/parser.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod minik/ dizininde (125'e göre farkı: diff -ru).
cd minik
ks 'cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "'
k cargo build -q --release
# Öncelik ağacın biçiminde: 2 + (3 * (4 - 1))
k ./target/release/minik ast ornekler/ifade.mk
# Pratt ayrıştırmanın adımları: her girinti bir ifade(en_az) çağrısı
k cat ornekler/oncelik.mk
k ./target/release/minik iz ornekler/oncelik.mk
k cat ornekler/fib.mk
k ./target/release/minik ast ornekler/fib.mk
k ./target/release/minik ast ornekler/hatali.mk
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test lexer::testler::anahtar_sozcuk_once_ad_gibi_okunur ... ok
test lexer::testler::en_uzun_eslesme ... ok
test lexer::testler::sutun_karakter_sayar_bayt_degil ... ok
test lexer::testler::yorum_atlanir_ve_hata_konumu_bildirilir ... ok
test parser::testler::hata_konumu_bildirilir ... ok
test parser::testler::oncelik_ve_sola_birlesme ... ok
test parser::testler::parantez_onceligi_degistirir ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo build -q --release
$ ./target/release/minik ast ornekler/ifade.mk
(let x (+ 2 (* 3 (- 4 1))))
(print x)
$ cat ornekler/oncelik.mk
print(1 + 2 * 3 - 4);
$ ./target/release/minik iz ornekler/oncelik.mk
ifade(en_az = 0)
  önek: 1
  '+' gücü 5 > 0: sağ işleneni oku
  ifade(en_az = 5)
    önek: 2
    '*' gücü 6 > 5: sağ işleneni oku
    ifade(en_az = 6)
      önek: 3
      '-' gücü 5 ≤ 6: dur
    kuruldu: (* 2 3)
    '-' gücü 5 ≤ 5: dur
  kuruldu: (+ 1 (* 2 3))
  '-' gücü 5 > 0: sağ işleneni oku
  ifade(en_az = 5)
    önek: 4
  kuruldu: (- (+ 1 (* 2 3)) 4)
(print (- (+ 1 (* 2 3)) 4))
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
- Kaynaklar: `koruma_sayfasi.rs`, `minik/Cargo.toml`, `minik/src/ast.rs`, `minik/src/bin/minik.rs`, `minik/src/lexer.rs`, `minik/src/lib.rs`, `minik/src/parser.rs`, `minik/src/yorumlayici.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: yığın taşması satırlarındaki iş parçacığı kimlikleri (parantez içindeki sayılar); koruma sayfası satırlarındaki adresler
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
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
```

- Gerçek çıktı (`cikti.txt`):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test lexer::testler::anahtar_sozcuk_once_ad_gibi_okunur ... ok
test lexer::testler::en_uzun_eslesme ... ok
test lexer::testler::sutun_karakter_sayar_bayt_degil ... ok
test lexer::testler::yorum_atlanir_ve_hata_konumu_bildirilir ... ok
test parser::testler::hata_konumu_bildirilir ... ok
test parser::testler::oncelik_ve_sola_birlesme ... ok
test parser::testler::parantez_onceligi_degistirir ... ok
test yorumlayici::testler::derinlik_siniri ... ok
test yorumlayici::testler::kapsam_ve_ozyineleme ... ok
test yorumlayici::testler::kisa_devre_sag_tarafi_degerlendirmez ... ok
test yorumlayici::testler::tasma_sessizce_sarmaz ... ok
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
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
$ cat ornekler/kisa_devre.mk
fn bol(a, b) {
    return a / b;
}
// && sağ tarafı yalnız sol taraf true ise değerlendirir
print(false && bol(1, 0) == 0);
print(true || bol(1, 0) == 0);
print(true && bol(1, 0) == 0);
$ ./target/release/minik calistir ornekler/kisa_devre.mk
false
true
hata: çalışma hatası: sıfıra bölme
[çıkış kodu: 1]
$ cat ornekler/sonsuz.mk
fn sonsuz(n) {
    return sonsuz(n + 1);
}
print(sonsuz(0));
$ ./target/release/minik calistir ornekler/sonsuz.mk
hata: çalışma hatası: çağrı derinliği sınırı aşıldı (2000)
[çıkış kodu: 1]
$ ./target/release/minik calistir --derinlik 60000 ornekler/sonsuz.mk
hata: çalışma hatası: çağrı derinliği sınırı aşıldı (60000)
[çıkış kodu: 1]
$ ./target/release/minik calistir --derinlik 80000 ornekler/sonsuz.mk

thread 'yorumlayici' (161) has overflowed its stack
fatal runtime error: stack overflow, aborting
[çıkış kodu: 134 = 128 + 6 (SIGABRT)]
$ ./target/release/minik calistir --derinlik 1000000 ornekler/sonsuz.mk

thread 'yorumlayici' (164) has overflowed its stack
fatal runtime error: stack overflow, aborting
[çıkış kodu: 134 = 128 + 6 (SIGABRT)]
$ rustc -O koruma_sayfasi.rs -o koruma_sayfasi
$ ./koruma_sayfasi
yığının altındaki eşleme: 7fda8d0d9000-7fda8d0da000 ---p 00000000 00:00 0
iş parçacığının yığını:   7fda8d0da000-7fda8d1da000 rw-p 00000000 00:00 0
```

## 128-yorumlayici-bytecode-vm

- Video: `128-yorumlayici-bytecode-vm`
- Örnek yolu: `deneyler/mufredat-ornekleri/128-yorumlayici-bytecode-vm/`
- Kaynaklar: `minik/Cargo.toml`, `minik/src/ast.rs`, `minik/src/bin/minik.rs`, `minik/src/derleyici.rs`, `minik/src/lexer.rs`, `minik/src/lib.rs`, `minik/src/parser.rs`, `minik/src/vm.rs`, `minik/src/yorumlayici.rs`
- Toolchain: stable 1.100.0
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: süreler ve hızlanma oranı; callgrind satırlarındaki süreç kimliği (==N==); komut sayıları küçük oranda değişebilir
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# Kontrol noktası: bu videonun sonundaki kod (Minik'in bitmiş hâli) minik/ dizininde (127'ye göre farkı: diff -ru).
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
```

- Gerçek çıktı (kısaltılmış; 105 satırın tamamı `cikti.txt` içinde):

```text
$ cargo test --lib -- --test-threads=1 2>&1 | grep -E "^test "
test fark_testi::ornek_programlarda_iki_motor_ayni ... ok
test fark_testi::tanimsiz_ad_iki_motorda_farkli_anda_yakalanir ... ok
test lexer::testler::anahtar_sozcuk_once_ad_gibi_okunur ... ok
test lexer::testler::en_uzun_eslesme ... ok
test lexer::testler::sutun_karakter_sayar_bayt_degil ... ok
test lexer::testler::yorum_atlanir_ve_hata_konumu_bildirilir ... ok
test parser::testler::hata_konumu_bildirilir ... ok
test parser::testler::oncelik_ve_sola_birlesme ... ok
test parser::testler::parantez_onceligi_degistirir ... ok
test yorumlayici::testler::derinlik_siniri ... ok
test yorumlayici::testler::kapsam_ve_ozyineleme ... ok
test yorumlayici::testler::kisa_devre_sag_tarafi_degerlendirmez ... ok
test yorumlayici::testler::tasma_sessizce_sarmaz ... ok
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
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
… (16 satır atlandı)
14
$ diff <(./target/release/minik calistir ornekler/fib.mk) <(./target/release/minik vm ornekler/fib.mk) && echo "fib.mk: iki motorun çıktısı aynı"
fib.mk: iki motorun çıktısı aynı
$ cat ornekler/tanimsiz.mk
… (2 satır atlandı)
$ ./target/release/minik calistir ornekler/tanimsiz.mk
1
hata: çalışma hatası: tanımsız değişken: y
… (1 satır atlandı)
$ ./target/release/minik vm ornekler/tanimsiz.mk
hata: tanımsız değişken: y
… (10 satır atlandı)
ağaç dolaşan yorumlayıcı:    161.4 ms; 5084966 ifade düğümü, 1906863 HashMap ad araması, 1271242 yeni kapsam
bayt kod + yığın makinesi:    34.1 ms; 6356210 komut; ad araması yok (yuva numaraları derleme anında)
hızlanma: 4.73×
$ valgrind --tool=callgrind --callgrind-out-file=cg.agac ./target/release/minik calistir ornekler/fib20.mk 2>&1 | grep -E "^[0-9]+$|Collected"
6765
==39392== Collected : 68448669
$ valgrind --tool=callgrind --callgrind-out-file=cg.vm ./target/release/minik vm ornekler/fib20.mk 2>&1 | grep -E "^[0-9]+$|Collected"
6765
==39396== Collected : 14273044
$ callgrind_annotate --inclusive=no cg.agac | grep -E "PROGRAM TOTALS|::ifade.2|hash_one|Sip13Rounds|malloc.c:(malloc|free) |::ikili " | sed -E "s/ \[.*//; s/[?]{3}://"
68,448,669 (100.0%)  PROGRAM TOTALS
16,253,695 (23.75%)  <minik::yorumlayici::Yorumlayici>::ifade'2
 7,125,474 (10.41%)  <std::hash::random::RandomState as core::hash::BuildHasher>::hash_one::<&alloc::string::String>
 6,633,012 ( 9.69%)  <core::hash::sip::Hasher<core::hash::sip::Sip13Rounds> as core::hash::Hasher>::write
 4,014,891 ( 5.87%)  ./malloc/./malloc/malloc.c:free
 2,650,342 ( 3.87%)  ./malloc/./malloc/malloc.c:malloc
 1,970,190 ( 2.88%)  <std::hash::random::RandomState as core::hash::BuildHasher>::hash_one::<&str>
 1,882,582 ( 2.75%)  minik::yorumlayici::ikili
$ callgrind_annotate --inclusive=no cg.vm | grep -E "PROGRAM TOTALS|::calistir |::ikili " | sed -E "s/ \[.*//; s/[?]{3}://"
14,273,044 (100.0%)  PROGRAM TOTALS
11,985,396 (83.97%)  minik::vm::calistir
 1,882,582 (13.19%)  minik::yorumlayici::ikili
```

## 129-yolun-sonu

- Video: `129-yolun-sonu`
- Örnek yolu: `deneyler/mufredat-ornekleri/129-yolun-sonu/`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/ambar/` — kaynaklar: `Cargo.toml`, `src/bin/ambar.rs`, `src/crc.rs`, `src/gunluk.rs`, `src/lib.rs`, `src/motor.rs`, `src/protokol.rs`, `src/sunucu.rs`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/kapi/` — kaynaklar: `Cargo.toml`, `src/ayristirici.rs`, `src/bin/kapi.rs`, `src/dosya.rs`, `src/havuz.rs`, `src/lib.rs`, `src/zaman_uyumsuz.rs`
- Ortak proje: `deneyler/mufredat-ornekleri/ortak-projeler/minik/` — kaynaklar: `Cargo.toml`, `src/ast.rs`, `src/bin/minik.rs`, `src/derleyici.rs`, `src/lexer.rs`, `src/lib.rs`, `src/parser.rs`, `src/vm.rs`, `src/yorumlayici.rs`
- Toolchain: stable 1.100.0
- Bağımlılıklar (Cargo.lock'ta çözülen): `bitflags 2.13.2`, `bytes 1.12.2`, `cfg-if 1.0.5`, `errno 0.3.15`, `libc 0.2.191`, `lock_api 0.4.14`, `mio 1.2.5`, `parking_lot 0.12.5`, `parking_lot_core 0.9.12`, `pin-project-lite 0.2.18`, `proc-macro2 1.0.107`, `quote 1.0.47`, `redox_syscall 0.5.18`, `scopeguard 1.2.0`, `signal-hook-registry 1.4.8`, `smallvec 1.16.6`, `socket2 0.6.5`, `syn 3.0.10`, `tokio 1.54.1`, `tokio-macros 2.7.2`, `unicode-ident 1.0.27`, `wasi 0.11.1+wasi-snapshot-preview1`, `windows-link 0.2.1`, `windows-sys 0.61.2`
- Son çalıştırma (UTC): çalıştırılmadı
- Değişken çıktı: strace satırlarının başındaki iş parçacığı kimlikleri, dosya tanımlayıcı numaraları, istemci portu (sin_port) ve "accept4 resumed" satırındaki adres
- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):

```bash
# 1) Bitirme projelerinin on iki kontrol noktasında testler
ks 'for c in ../11[7-9]-*/ambar/Cargo.toml ../12[01]-*/ambar/Cargo.toml ../12[2-4]-*/kapi/Cargo.toml ../12[5-8]-*/minik/Cargo.toml; do d=${c%/Cargo.toml}; printf "%-42s %s\n" "${d#../}" "$(cd $d && cargo test -q --lib 2>&1 | grep -o "[0-9]* passed; [0-9]* failed")"; done'
# ortak-projeler/ altındaki bitmiş projeler, son kontrol noktalarının aynısı mı?
ks 'diff -r -x target ../121-ambar-performans/ambar ../ortak-projeler/ambar && diff -r -x target ../124-http-eszamanlilik-ve-yuk/kapi ../ortak-projeler/kapi && diff -r -x target ../128-yorumlayici-bytecode-vm/minik ../ortak-projeler/minik && echo "ortak-projeler: son kontrol noktalarıyla aynı"'
# 2) Bir isteğin çekirdeğe dokunan adımları: Ambar'a tek bir "yaz" ve sistem çağrıları
cd ../ortak-projeler/ambar
cargo build -q --release
rm -rf calisma
echo "# her yazmada fsync kipi"
strace -f -o strace.txt -e trace=accept4,recvfrom,write,sendto,fdatasync ./target/release/ambar sunucu calisma 127.0.0.1:7029 her 2>/dev/null &
SP=$!
sleep 1
ks 'exec 3<>/dev/tcp/127.0.0.1/7029; printf "yaz dil Rust\n" >&3; head -n 1 <&3'
sleep 0.2
kill $(pgrep -P $SP)   # izlenen sunucuyu durdur; strace onunla birlikte çıkar
wait $SP 2>/dev/null
ks 'grep -v "write(2," strace.txt | grep -E "accept4|recvfrom|write|sendto|fdatasync"'
k od -A d -t x1z -v calisma/gunluk.ambar
rm -rf calisma strace.txt
echo "# toplu kesinleştirme kipi"
strace -f -o strace.txt -e trace=accept4,recvfrom,write,sendto,fdatasync ./target/release/ambar sunucu calisma 127.0.0.1:7030 toplu 2>/dev/null &
SP=$!
sleep 1
ks 'exec 3<>/dev/tcp/127.0.0.1/7030; printf "yaz dil Rust\n" >&3; head -n 1 <&3'
sleep 0.2
kill $(pgrep -P $SP)
wait $SP 2>/dev/null
ks 'grep -v "write(2," strace.txt | grep -E "recvfrom\([0-9]+, \"yaz|write\(|sendto|fdatasync"'
rm -rf calisma strace.txt
cd ../../129-yolun-sonu
# 3) Üç bitmiş projenin Rust satır sayısı ve bu müfredatın deney dizinleri
ks 'wc -l ../ortak-projeler/*/src/*.rs ../ortak-projeler/*/src/bin/*.rs | tail -n 1'
ks 'printf "deney dizini: %s\n" "$(ls -d ../[0-9][0-9][0-9]-*/ | wc -l)"'
```

- Gerçek çıktı (`cikti.txt`):

```text
$ for c in ../11[7-9]-*/ambar/Cargo.toml ../12[01]-*/ambar/Cargo.toml ../12[2-4]-*/kapi/Cargo.toml ../12[5-8]-*/minik/Cargo.toml; do d=${c%/Cargo.toml}; printf "%-42s %s\n" "${d#../}" "$(cd $d && cargo test -q --lib 2>&1 | grep -o "[0-9]* passed; [0-9]* failed")"; done
117-ambar-tasarim/ambar                    3 passed; 0 failed
118-ambar-depolama-motoru/ambar            8 passed; 0 failed
119-ambar-ag-protokolu/ambar               11 passed; 0 failed
120-ambar-eszamanlilik-ve-dayaniklilik/ambar 11 passed; 0 failed
121-ambar-performans/ambar                 12 passed; 0 failed
122-http-ayristirici/kapi                  7 passed; 0 failed
123-http-sunucu/kapi                       10 passed; 0 failed
124-http-eszamanlilik-ve-yuk/kapi          10 passed; 0 failed
125-yorumlayici-lexer/minik                4 passed; 0 failed
126-yorumlayici-parser/minik               7 passed; 0 failed
127-yorumlayici-degerlendirici/minik       11 passed; 0 failed
128-yorumlayici-bytecode-vm/minik          13 passed; 0 failed
$ diff -r -x target ../121-ambar-performans/ambar ../ortak-projeler/ambar && diff -r -x target ../124-http-eszamanlilik-ve-yuk/kapi ../ortak-projeler/kapi && diff -r -x target ../128-yorumlayici-bytecode-vm/minik ../ortak-projeler/minik && echo "ortak-projeler: son kontrol noktalarıyla aynı"
ortak-projeler: son kontrol noktalarıyla aynı
# her yazmada fsync kipi
$ exec 3<>/dev/tcp/127.0.0.1/7029; printf "yaz dil Rust\n" >&3; head -n 1 <&3
tamam
$ grep -v "write(2," strace.txt | grep -E "accept4|recvfrom|write|sendto|fdatasync"
42112 accept4(4, {sa_family=AF_INET, sin_port=htons(58874), sin_addr=inet_addr("127.0.0.1")}, [128 => 16], SOCK_CLOEXEC) = 5
42112 accept4(4,  <unfinished ...>
42115 recvfrom(7, "yaz dil Rust\n", 8192, 0, NULL, NULL) = 13
42115 write(3, "\335O\317d\3\0\0\0\4\0\0\0\324\201\350\361dilRust", 23) = 23
42115 fdatasync(3)                      = 0
42115 sendto(5, "tamam\n", 6, MSG_NOSIGNAL, NULL, 0) = 6
42115 recvfrom(7, "", 8192, 0, NULL, NULL) = 0
42112 <... accept4 resumed>0x7ffdc03ccf18, [128], SOCK_CLOEXEC) = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
$ od -A d -t x1z -v calisma/gunluk.ambar
0000000 dd 4f cf 64 03 00 00 00 04 00 00 00 d4 81 e8 f1  >.O.d............<
0000016 64 69 6c 52 75 73 74                             >dilRust<
0000023
# toplu kesinleştirme kipi
$ exec 3<>/dev/tcp/127.0.0.1/7030; printf "yaz dil Rust\n" >&3; head -n 1 <&3
tamam
$ grep -v "write(2," strace.txt | grep -E "recvfrom\([0-9]+, \"yaz|write\(|sendto|fdatasync"
42131 recvfrom(8, "yaz dil Rust\n", 8192, 0, NULL, NULL) = 13
42131 write(3, "\335O\317d\3\0\0\0\4\0\0\0\324\201\350\361dilRust", 23) = 23
42128 fdatasync(4)                      = 0
42131 sendto(6, "tamam\n", 6, MSG_NOSIGNAL, NULL, 0) = 6
$ wc -l ../ortak-projeler/*/src/*.rs ../ortak-projeler/*/src/bin/*.rs | tail -n 1
  2798 total
$ printf "deney dizini: %s\n" "$(ls -d ../[0-9][0-9][0-9]-*/ | wc -l)"
deney dizini: 129
```

