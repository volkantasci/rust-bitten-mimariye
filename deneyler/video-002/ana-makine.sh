#!/usr/bin/env bash
# 002: ana makine tarafı (konteynerin DIŞINDA, ana makinede çalıştırılır). Laboratuvar tarafı: mufredat-ornekleri/002-laboratuvar-kurulumu ve video-002/komut.sh.
# Değişken: çekirdek sürümü ve dağıtım makineye göre; podman top'taki HPID her çalıştırmada değişebilir.
# İlk ad alanlarının numaraları çekirdekte sabit tanımlı (include/uapi/linux/nsfs.h: PID_NS_INIT_INO = 0xEFFFFFFC = 4026531836 …);
# kabuk ilk ad alanlarındaysa bu değerler önyüklemeden bağımsızdır (eski çekirdeklerde bazıları farklı olabilir; doğrulanmadı).
# Not: /proc/self/cgroup bilerek yok (oturum adı gibi kişisel yapılandırma bilgisi içerir).
k() { echo "\$ $*"; bash -c "$*"; }
k 'uname -r'
k 'grep -E "^(NAME|VERSION_ID)=" /etc/os-release'
k "find /proc/self/ns -type l -printf '%f -> %l\n' | sort"
# Laboratuvar imajından geçici bir konteyner (ilk süreci sleep); podman top aynı süreci konteynerdeki (PID) ve
# ana makinedeki (HPID) numarasıyla gösterir. Konteyner hemen silinir.
podman run -d --rm --name nkl-rust-ana-makine localhost/nkl-rust:1.100.0 sleep 60 > /dev/null
k 'podman top nkl-rust-ana-makine pid hpid comm'
podman rm -f -t 0 nkl-rust-ana-makine > /dev/null 2>&1
