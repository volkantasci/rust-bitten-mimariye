#!/usr/bin/env bash
# Konteyner içinde çalışır. Proje kökünden: scripts/rust/rr.sh video-004 bash calistir.sh
# (açık depoda: laboratuvar/rr.sh video-004 bash calistir.sh)
# komut.sh'yi müfredat deneylerinin yardımcılarıyla (k / ks) çalıştırır, çıktıyı cikti.txt'ye yazar.
# Ana deneyin programı kopyalanır: ../mufredat-ornekleri/004-bit-ve-bayt
# Kullanım: bash calistir.sh [-k]   (-k: calisma/ dizinini silme)
set -uo pipefail
cd /work/video-004
TEMIZLE=1
if [ "${1:-}" = "-k" ]; then TEMIZLE=0; fi
( bash -c 'source ../mufredat-ornekleri/_araclar/ortak.sh; source ./komut.sh' ) > cikti.txt 2>&1
date -u +%Y-%m-%dT%H:%M:%SZ > .son-calistirma
echo "cikti.txt: $(wc -l < cikti.txt) satır ($(cat .son-calistirma))"
if [ "$TEMIZLE" = 1 ]; then
  rm -rf calisma
fi
