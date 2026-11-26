#!/usr/bin/env python3
"""strace -k çıktısını özetler: her sistem çağrısı için sıra numarası, çağrının adı ve
çağrı anında yığın izinde hangi kodun bulunduğu:
  main       : programın main fonksiyonu (Rust'ın çalışma zamanı hazırlığı ve bizim kodumuz)
  _start     : programın giriş noktası, ama main yok (main döndükten sonraki çıkış)
  yükleyici  : yalnız dinamik yükleyici ve C kütüphanesi (programın kendi kodu henüz başlamamış)
Kullanım: python3 yigin_ozeti.py <strace -k -o ile yazılmış dosya>
Yığın izi satırları " > " ile başlar; öbür satırlar çağrının kendisidir.
"+++ exited ..." satırı çağrı değildir, sayılmaz."""
import re
import sys

cagrilar = []  # [ad, main, _start, yükleyici]
with open(sys.argv[1], encoding="utf-8", errors="replace") as f:
    for satir in f:
        if satir.startswith(" > "):
            if cagrilar:
                if "(main+" in satir:
                    cagrilar[-1][1] = True
                if "(_start+" in satir:
                    cagrilar[-1][2] = True
                if "ld-linux" in satir:
                    cagrilar[-1][3] = True
            continue
        if satir.startswith("+++"):
            continue
        m = re.match(r"([a-z_0-9]+)\(", satir)
        if m:
            cagrilar.append([m.group(1), False, False, False])

for i, (ad, main, start, yukleyici) in enumerate(cagrilar, 1):
    if main:
        yer = "main"
    elif start:
        yer = "_start"
    elif yukleyici:
        yer = "yükleyici"
    else:
        yer = "-"
    print(f"{i:2d} {ad:<18} {yer}")
