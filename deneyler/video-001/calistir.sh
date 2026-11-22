#!/usr/bin/env bash
# Konteyner içinde çalışır. Depo kökünden: laboratuvar/rr.sh video-001 bash calistir.sh
# komut.sh'yi ana deneyin yardımcılarıyla (k / ks) çalıştırır, çıktıyı cikti.txt'ye yazar.
# Kullanım: bash calistir.sh [-k]   (-k: target/ dizinlerini silme)
set -uo pipefail
cd /work/video-001
TEMIZLE=1
if [ "${1:-}" = "-k" ]; then TEMIZLE=0; fi
( bash -c 'source ../mufredat-ornekleri/_araclar/ortak.sh; source ./komut.sh' ) > cikti.txt 2>&1
date -u +%Y-%m-%dT%H:%M:%SZ > .son-calistirma
echo "cikti.txt: $(wc -l < cikti.txt) satır ($(cat .son-calistirma))"
if [ "$TEMIZLE" = 1 ]; then
  find . -name target -type d -prune -exec rm -rf {} +
fi
