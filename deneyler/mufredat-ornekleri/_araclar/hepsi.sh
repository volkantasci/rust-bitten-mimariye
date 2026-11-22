#!/usr/bin/env bash
# Konteyner içinde çalışır (/work/mufredat-ornekleri).
# Her örnek dizinindeki komut.sh'yi çalıştırır, çıktıyı o dizindeki cikti.txt'ye yazar,
# sonunda DOGRULAMA.md'yi yeniden üretir.
# Kullanım: bash _araclar/hepsi.sh [-k] [örnek-dizini ...]
#   -k : derleme dizinlerini (target/, calisma/) silme (geliştirme sırasında hız için)
set -uo pipefail
cd /work/mufredat-ornekleri
TEMIZLE=1
if [ "${1:-}" = "-k" ]; then TEMIZLE=0; shift; fi

# miri'nin derlediği std (sysroot) her --rm konteynerde yeniden derlenmesin
export XDG_CACHE_HOME=/work/mufredat-ornekleri/.onbellek
mkdir -p "$XDG_CACHE_HOME"

{
  echo "tarih: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  rustc -Vv
  cargo -V
  rustc +nightly -V
  cargo +nightly miri --version
  gcc --version | head -1
  lscpu | grep -E 'Model name|^L[123]'
  echo "çekirdek: $(uname -r)"
  echo "sayfa boyutu: $(getconf PAGESIZE)"
} > _araclar/ortam.txt 2>&1

if [ $# -gt 0 ]; then dizinler=("$@"); else dizinler=([0-9][0-9][0-9]-*/); fi
for d in "${dizinler[@]}"; do
  d=${d%/}
  [ -f "$d/komut.sh" ] || { echo "atlandı (komut.sh yok): $d"; continue; }
  printf '>>> %s ... ' "$d"
  bas=$(date +%s)
  ( cd "$d" && bash -c 'source ../_araclar/ortak.sh; source ./komut.sh' ) > "$d/cikti.txt" 2>&1
  sure=$(( $(date +%s) - bas ))
  date -u +%Y-%m-%dT%H:%M:%SZ > "$d/.son-calistirma"
  echo "${sure} sn, $(wc -l < "$d/cikti.txt") satır"
  if [ "$TEMIZLE" = 1 ]; then
    find "$d" \( -name target -o -name calisma \) -type d -prune -exec rm -rf {} +
  fi
done

if [ "$TEMIZLE" = 1 ]; then
  find ortak-projeler -maxdepth 2 \( -name target -o -name calisma \) -type d -prune -exec rm -rf {} + 2>/dev/null
fi

python3 _araclar/dogrulama_uret.py
