# Rust: Bitten Mimariye — deney kodları

Bu depo, **"Rust: Bitten Mimariye"** video serisinde ekranda gösterilen her komutun ve çıktının kaynağıdır. Seride gösterilen her sonuç burada kayıtlı bir deneyden gelir; aynı ortamı kurup sonuçları kendiniz doğrulayabilirsiniz.

## Laboratuvar

Deneyler, resmî Rust imajı üzerine kurulmuş yalıtılmış bir Podman konteynerinde çalışır.

- `laboratuvar/Containerfile`: `docker.io/library/rust:1.100.0` (rustc/cargo 1.100.0, Debian 13) + clippy, rustfmt, tarihe sabitlenmiş nightly (`nightly-2026-11-20`, rustc 1.102.0-nightly 6905905bb) + Miri ve rust-src, valgrind, strace, gdb, hyperfine. Kayıtlardaki Miri, sanitizer ve `-Z` çıktıları bu nightly sürümüne aittir; deneyler `+nightly` ile çağırdığı için sabit sürüm "nightly" adıyla kurulur.
- `laboratuvar/rr.sh`: komutları bu konteynerde çalıştırır; `deneyler/` dizinini konteynerde `/work` olarak bağlar.

```bash
podman build -t localhost/nkl-rust:1.100.0 -f laboratuvar/Containerfile laboratuvar
```

```bash
laboratuvar/rr.sh . rustc -V
```

Not: `rr.sh` konteyneri `--cap-add=SYS_PTRACE --security-opt seccomp=unconfined` ile açar; strace, valgrind ve gdb deneyleri bunu gerektirir. Çöken deneyler (panik, yığın taşması) ana makinede çekirdek dökümü bırakmasın diye `--ulimit core=1:1` ile çalışır.

İmaj daha önce kurulduysa yeniden kurun: eski imajdaki nightly tarihsizdi.

## Deneyler

- `deneyler/mufredat-ornekleri/NNN-…/`: serinin her videosu için bir dizin. Her birinde kaynak kod, çalıştırılan komutlar (`komut.sh`) ve gerçek çıktı (`cikti.txt`) var. Tüm kayıtların özeti `DOGRULAMA.md`.
- `deneyler/mufredat-ornekleri/ortak-projeler/`: bitirme projeleri (ambar, kapı, minik).
- `deneyler/video-001/`: 001 numaralı videonun ek deneyleri.

Bütün örnekleri yeniden çalıştırmak (çıktıları ve `DOGRULAMA.md`'yi yeniden üretir):

```bash
deneyler/mufredat-ornekleri/calistir.sh
```

Tek bir örnek:

```bash
deneyler/mufredat-ornekleri/calistir.sh 004-bit-ve-bayt
```

002'nin ek deneyleri (laboratuvar içi; ana makine tarafı `deneyler/video-002/ana-makine.sh`, konteynerin dışında, yalnız Linux'ta aynen çalışır):

```bash
laboratuvar/rr.sh video-002 bash calistir.sh
```

003'ün ek deneyleri (ELF başlığı, çıktının baytları, sistem çağrılarının yığın izleri, satır sonu olmadan `print!`):

```bash
laboratuvar/rr.sh video-003 bash calistir.sh
```

004'ün ek deneyleri (basamak değerleri, tip boyutları, bellekteki baytlar, bayt sırasının kaynakları, ters okuma, ELF başlığının makine alanı):

```bash
laboratuvar/rr.sh video-004 bash calistir.sh
```

001'in ek deneyleri:

```bash
laboratuvar/rr.sh video-001 bash calistir.sh
```

## Sonuçlarınız farklı çıkarsa

Bellek adresleri, süreler, süreç kimlikleri ve rastgele dağılımlar her çalıştırmada değişir; kayıtlarda bu satırlar `#@ degisken` ile işaretlidir. Önbellek, sistem çağrısı, iş parçacığı ve ağ ölçümleri donanıma ve çekirdeğe bağlıdır. Referans makine: x86-64, AMD Ryzen 5 5600 (6 çekirdek / 12 mantıksal işlemci), Linux 7.2, glibc 2.41, Podman 6.1. Laboratuvar imajı mimariden bağımsız yazılmıştır (arm64'te denenmedi); makine kodu, önbellek ve bellek sıralaması gösteren deneylerin çıktısı x86-64 içindir; bazı eş zamanlılık deneyleri en az 3 mantıksal işlemci ister (ilgili videoda söylenir). Ağa bağlı birkaç satır (crates.io) çevrimdışı çalıştırmada hata verir.

## Yayımlanmış 001 ve eski deneyler

Seri müfredatı 2026-11-24'te yeniden düzenlendi: deneyler yalnız Rust'tadır (FFI videosundaki C tarafı hariç), her videoda "Kendin dene" adımı kayıtlıdır. Yayımlanan 001 videosunun gösterdiği deney değişmeden şuradadır:

- `deneyler/mufredat-ornekleri/_kullanilmiyor/001-neden-rust-neden-bilim--yayimlanan-001/`
- `deneyler/video-001/`

`_kullanilmiyor/` altındaki öteki dizinler revizyonda yerini yenisine bırakan deneylerdir; her birinde nedenini anlatan `NEDEN.md` vardır.
