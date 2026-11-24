#!/usr/bin/env bash
# Aynı 20000 küçük fonksiyonla iki crate üretir (derleme süresi deneyi için):
#   buyuk/   : hepsi tek modülde (kök modül)
#   moduler/ : MODUL modüle bölünmüş (varsayılan 100; her modülde 20000 / MODUL fonksiyon)
# Kendin dene: MODUL=10 bash ../uret.sh  (20000'i tam bölen bir sayı seçin)
set -e
M=${MODUL:-100}
K=$((20000 / M))
yaz_toml() {
  mkdir -p "$1/src"
  printf '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2024"\n\n[dependencies]\n' "$1" > "$1/Cargo.toml"
}
govde() { echo "#[inline(never)] pub fn f$1(x: u64) -> u64 { (x ^ $1).wrapping_mul(0x9E3779B97F4A7C15).rotate_left($(($1 % 63 + 1))) }"; }

yaz_toml buyuk
{
  for i in $(seq 0 19999); do govde "$i"; done
  echo "fn main() {"
  echo "    let mut x = 1u64;"
  for i in $(seq 0 19999); do echo "    x = f$i(x);"; done
  echo "    println!(\"{x}\");"
  echo "}"
} > buyuk/src/main.rs

yaz_toml moduler
{
  for m in $(seq 0 $((M - 1))); do
    echo "mod m$m {"
    for j in $(seq 0 $((K - 1))); do printf '    '; govde $((m * K + j)); done
    echo "}"
  done
  echo "fn main() {"
  echo "    let mut x = 1u64;"
  for i in $(seq 0 19999); do echo "    x = m$((i / K))::f$i(x);"; done
  echo "    println!(\"{x}\");"
  echo "}"
} > moduler/src/main.rs
