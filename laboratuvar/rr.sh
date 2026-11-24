#!/usr/bin/env bash
# Rust komutlarını yalıtılmış Podman konteynerinde çalıştırır.
# Kullanım: laboratuvar/rr.sh <deneyler/ altındaki alt dizin veya .> <komut...>
#   ör: laboratuvar/rr.sh video-001 bash calistir.sh
set -euo pipefail
KOK="$(cd "$(dirname "$0")/.." && pwd)"
IMG="localhost/nkl-rust:1.100.0"
SUB="${1:-.}"; shift || true
exec podman run --rm --name "nkl-rust-$$" \
  --cap-add=SYS_PTRACE --security-opt seccomp=unconfined --ulimit core=1:1 \
  -v "$KOK/deneyler:/work:Z" \
  -v nkl-cargo-registry:/usr/local/cargo/registry \
  -w "/work/$SUB" "$IMG" "${@:-bash}"
