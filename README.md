# Rust: Bitten Mimariye — deney kodları

Bu depo, **"Rust: Bitten Mimariye"** video serisinde ekranda gösterilen her komutun ve çıktının kaynağıdır. Seride gösterilen her sonuç burada kayıtlı bir deneyden gelir; aynı ortamı kurup sonuçları kendiniz doğrulayabilirsiniz.

## Laboratuvar

Deneyler, resmî Rust imajı üzerine kurulmuş yalıtılmış bir Podman konteynerinde çalışır.

- `laboratuvar/Containerfile`: `docker.io/library/rust:1.100.0` (rustc/cargo 1.100.0, Debian 13) + clippy, rustfmt, nightly + miri, valgrind, strace, gdb, hyperfine.
- `laboratuvar/rr.sh`: komutları bu konteynerde çalıştırır; `deneyler/` dizinini konteynerde `/work` olarak bağlar.

```bash
podman build -t localhost/nkl-rust:1.100.0 -f laboratuvar/Containerfile laboratuvar
```

```bash
laboratuvar/rr.sh . rustc -V
```

Not: `rr.sh` konteyneri `--cap-add=SYS_PTRACE --security-opt seccomp=unconfined` ile açar; strace, valgrind ve gdb deneyleri bunu gerektirir.

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

001'in ek deneyleri:

```bash
laboratuvar/rr.sh video-001 bash calistir.sh
```

## Sonuçlarınız farklı çıkarsa

Bellek adresleri, süreler, süreç kimlikleri her çalıştırmada değişir. Önbellek, sistem çağrısı ve ağ ölçümleri donanıma ve çekirdeğe bağlıdır (referans makine: AMD Ryzen 5 5600, Linux 7.2). Tanımsız davranış gösteren C örnekleri farklı derleyici/kütüphane sürümlerinde farklı sonuç verebilir; videoda da söylendiği gibi, farklı bir sonuç görürseniz paylaşın.
