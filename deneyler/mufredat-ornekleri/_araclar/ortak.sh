# shellcheck shell=bash
# Müfredat örnekleri için ortak yardımcılar.
# hepsi.sh her örneğin komut.sh dosyasını bu dosyayla birlikte, örnek dizininde çalıştırır.
#   k  komut arg...     : komutu "$ komut" diye yazar, çalıştırır (stdout+stderr), sıfır olmayan çıkışı yazar
#   ks 'kabuk satırı'   : boru/yönlendirme içeren satırlar için aynısı (bash -c ile)
export CARGO_TERM_COLOR=never
export LANG=C.UTF-8
unset RUST_BACKTRACE

# Bilerek çöken deneyler (abort, yığın taşması, double free) ana makinedeki
# systemd-coredump'a çekirdek dökümü göndermesin: RLIMIT_CORE=1 olduğunda çekirdek,
# boru tipi core_pattern için dökümü başlatmadan iptal eder.
prlimit --pid $$ --core=1:1

_durum() {
  local rc=$1
  if [ "$rc" -gt 128 ]; then
    printf '[çıkış kodu: %d = 128 + %d (SIG%s)]\n' "$rc" $((rc - 128)) "$(kill -l $((rc - 128)))"
  elif [ "$rc" -ne 0 ]; then
    printf '[çıkış kodu: %d]\n' "$rc"
  fi
}
# Gösterimde boşluk/özel karakter içeren argümanlar çift tırnakla yazılır
_goster() {
  local a out=""
  for a in "$@"; do
    case $a in
      *[[:space:]\'\"\$\`\\\|\&\;\<\>\(\)\*\?]*) out+=" \"${a//\"/\\\"}\"" ;;
      *) out+=" $a" ;;
    esac
  done
  printf '$%s\n' "$out"
}
k()  { _goster "$@"; { "$@" 2>&1; } 2>/dev/null; _durum $?; return 0; }
ks() { printf '$ %s\n' "$1"; { bash -c "$1" 2>&1; } 2>/dev/null; _durum $?; return 0; }
