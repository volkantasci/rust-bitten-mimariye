#!/usr/bin/env bash
# Müfredat örneklerini Podman konteynerinde (localhost/nkl-rust:1.100.0) çalıştırır,
# her örneğin cikti.txt dosyasını ve DOGRULAMA.md'yi yeniden üretir.
# Kullanım (herhangi bir dizinden):
#   deneyler/mufredat-ornekleri/calistir.sh            # tüm örnekler
#   deneyler/mufredat-ornekleri/calistir.sh 005-tam-sayilar 006-kayan-nokta
#   deneyler/mufredat-ornekleri/calistir.sh -k 005-tam-sayilar   # target/ silinmesin
set -euo pipefail
BURASI="$(cd "$(dirname "$0")" && pwd)"
KOK="$(cd "$BURASI/../.." && pwd)"
podman image inspect --format '{{.Id}}' localhost/nkl-rust:1.100.0 > "$BURASI/_araclar/imaj-kimligi.txt"
"$KOK/laboratuvar/rr.sh" mufredat-ornekleri bash _araclar/hepsi.sh "$@"
