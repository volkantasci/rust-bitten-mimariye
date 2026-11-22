#!/usr/bin/env python3
"""DOGRULAMA.md üretici.

Her örnek dizinindeki komut.sh, kaynak dosyalar, Cargo.lock ve cikti.txt'den doğrulama
kaydını yazar. Elle düzenlenmez; _araclar/hepsi.sh sonunda otomatik çalışır.

komut.sh içinde tanınan yorumlar:
  #@ video: <NNN-kisa-ad>      örneğin ait olduğu video (yoksa dizin adı)
  #@ degisken: <açıklama>      çıktı her çalıştırmada değişir (adres, süre, sıra ...)
  #@ goster: <regex>           uzun çıktıda bu desene uyan satırlar da kayda girer
  #@ ortak: <yol>              örnek, ortak bir projeyi (ör. ortak-projeler/ambar) kullanıyor
"""
import pathlib
import re

KOK = pathlib.Path(__file__).resolve().parent.parent
TAM_SINIR = 90        # bu kadar satıra kadar çıktı tamamen yazılır
BAS, SON = 45, 20     # daha uzunsa: ilk BAS + "goster" eşleşmeleri + son SON satır
ATLA = {"target", "calisma", ".onbellek"}
UZANTI = {".rs", ".c", ".h", ".py", ".toml", ".sh"}


def kaynak_dosyalar(d):
    sonuc = []
    for yol in sorted(d.rglob("*")):
        parcalar = yol.relative_to(d).parts
        if any(p in ATLA for p in parcalar) or not yol.is_file():
            continue
        if yol.name in ("komut.sh", "cikti.txt", "Cargo.lock") or yol.name.startswith("."):
            continue
        if yol.suffix in UZANTI:
            sonuc.append("/".join(parcalar))
    return sonuc


def bagimliliklar(d):
    sonuc = set()
    for kilit in d.rglob("Cargo.lock"):
        if any(p in ATLA for p in kilit.relative_to(d).parts):
            continue
        metin = kilit.read_text()
        for blok in metin.split("[[package]]")[1:]:
            ad = re.search(r'^name = "([^"]+)"', blok, re.M)
            surum = re.search(r'^version = "([^"]+)"', blok, re.M)
            kaynak = re.search(r'^source = "registry\+', blok, re.M)
            if ad and surum and kaynak:
                sonuc.add(f"{ad.group(1)} {surum.group(1)}")
    return sorted(sonuc)


def toolchain(komut):
    t = ["stable 1.100.0"]
    if "+nightly" in komut:
        t.append("nightly (miri / -Z bayrakları)")
    if re.search(r"\bgcc\b", komut):
        t.append("gcc 14.2.0")
    if "python3" in komut:
        t.append("python3 (konteyner)")
    return ", ".join(t)


def kisalt(satirlar, desenler):
    if len(satirlar) <= TAM_SINIR:
        return satirlar, False
    secili = set(range(BAS)) | set(range(len(satirlar) - SON, len(satirlar)))
    for i, s in enumerate(satirlar):
        if any(re.search(p, s) for p in desenler):
            secili.add(i)
    cikis, onceki = [], -1
    for i in sorted(secili):
        if i != onceki + 1:
            cikis.append(f"… ({i - onceki - 1} satır atlandı)")
        cikis.append(satirlar[i])
        onceki = i
    return cikis, True


def bolum(d):
    komut = (d / "komut.sh").read_text()
    cikti_yolu = d / "cikti.txt"
    cikti = cikti_yolu.read_text(errors="replace").rstrip("\n").split("\n") if cikti_yolu.exists() else []
    video = re.search(r"^#@ video: *(.+)$", komut, re.M)
    degisken = re.findall(r"^#@ degisken: *(.+)$", komut, re.M)
    desenler = re.findall(r"^#@ goster: *(.+)$", komut, re.M)
    zaman = (d / ".son-calistirma").read_text().strip() if (d / ".son-calistirma").exists() else "çalıştırılmadı"
    satirlar, kisaldi = kisalt(cikti, desenler)
    deps = bagimliliklar(d)
    ortaklar = [KOK / o.strip() for o in re.findall(r"^#@ ortak: *(.+)$", komut, re.M)]
    for o in ortaklar:
        deps = sorted(set(deps) | set(bagimliliklar(o)))
    out = [f"## {d.name}", ""]
    out.append(f"- Video: `{video.group(1).strip() if video else d.name}`")
    out.append(f"- Örnek yolu: `deneyler/mufredat-ornekleri/{d.name}/`")
    kaynak = kaynak_dosyalar(d)
    if kaynak:
        out.append("- Kaynaklar: " + ", ".join(f"`{k}`" for k in kaynak))
    for o in ortaklar:
        out.append(f"- Ortak proje: `deneyler/mufredat-ornekleri/{o.relative_to(KOK)}/` — kaynaklar: "
                   + ", ".join(f"`{k}`" for k in kaynak_dosyalar(o)))
    out.append(f"- Toolchain: {toolchain(komut)}")
    if deps:
        out.append("- Bağımlılıklar (Cargo.lock'ta çözülen): " + ", ".join(f"`{x}`" for x in deps))
    out.append(f"- Son çalıştırma (UTC): {zaman}")
    for aciklama in degisken:
        out.append(f"- Değişken çıktı: {aciklama}")
    out.append("- Çalıştırılan betik (`komut.sh`; `k`/`ks` yardımcıları `_araclar/ortak.sh` içinde):")
    out.append("")
    out.append("```bash")
    out.extend(s for s in komut.rstrip("\n").split("\n") if not s.startswith("#@"))
    out.append("```")
    out.append("")
    baslik = "Gerçek çıktı" + (f" (kısaltılmış; {len(cikti)} satırın tamamı `cikti.txt` içinde)" if kisaldi else " (`cikti.txt`)")
    out.append(f"- {baslik}:")
    out.append("")
    out.append("```text")
    out.extend(satirlar)
    out.append("```")
    out.append("")
    return out


def main():
    ortam = (KOK / "_araclar" / "ortam.txt")
    imaj = (KOK / "_araclar" / "imaj-kimligi.txt")
    satirlar = [
        "# Müfredat örnekleri — doğrulama kaydı",
        "",
        "Bu dosya `_araclar/dogrulama_uret.py` tarafından üretilir; elle düzenlenmez.",
        "Her bölüm: örnek yolu, kaynaklar, toolchain, çalıştırılan betik ve konteynerdeki gerçek çıktı.",
        "",
        "Yeniden üretmek için (proje kökünden):",
        "",
        "```bash",
        "deneyler/mufredat-ornekleri/calistir.sh                 # tüm örnekler",
        "deneyler/mufredat-ornekleri/calistir.sh 004-bit-ve-bayt # tek örnek",
        "```",
        "",
        "Adres, süre ve iş parçacığı sırası gibi değerler her çalıştırmada değişir; bu örnekler",
        "aşağıda \"Değişken çıktı\" diye işaretlidir. Müfredatta gösterilen değerler bu kayıttaki",
        "çalıştırmadan kopyalanmıştır: `Rust/mufredat.md`, `_araclar/mufredat_uret.py` ile bu",
        "dizindeki `cikti.txt` dosyalarından üretilir ve `_araclar/kontrol.py` eşleşmeyi denetler.",
        "Bilerek çöken deneyler ana makineye çekirdek dökümü göndermesin diye `RLIMIT_CORE=1` ile",
        "çalışır (`_araclar/ortak.sh`). Ortak projeler (Ambar, Kapı, Minik) `ortak-projeler/` altındadır.",
        "",
        "## Ortam (son çalıştırma)",
        "",
        "```text",
    ]
    if imaj.exists():
        satirlar.append("imaj: localhost/nkl-rust:1.100.0  kimlik: " + imaj.read_text().strip())
    if ortam.exists():
        satirlar.extend(ortam.read_text().rstrip("\n").split("\n"))
    satirlar += ["```", ""]
    dizinler = sorted(p for p in KOK.iterdir() if p.is_dir() and re.match(r"\d{3}-", p.name) and (p / "komut.sh").exists())
    satirlar.append("## İçindekiler")
    satirlar.append("")
    for d in dizinler:
        satirlar.append(f"- [{d.name}](#{d.name})")
    satirlar.append("")
    for d in dizinler:
        satirlar.extend(bolum(d))
    (KOK / "DOGRULAMA.md").write_text("\n".join(satirlar) + "\n")
    print(f"DOGRULAMA.md yazıldı: {len(dizinler)} örnek")


if __name__ == "__main__":
    main()
