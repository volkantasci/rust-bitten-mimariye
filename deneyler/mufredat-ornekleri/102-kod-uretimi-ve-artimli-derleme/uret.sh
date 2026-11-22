#!/usr/bin/env bash
# 20000 küçük fonksiyonlu bir crate üretir (derleme süresi deneyi için)
set -e
mkdir -p buyuk/src
cat > buyuk/Cargo.toml <<'T'
[package]
name = "buyuk"
version = "0.1.0"
edition = "2024"

[dependencies]
T
{
  for i in $(seq 0 19999); do
    echo "#[inline(never)] pub fn f$i(x: u64) -> u64 { (x ^ $i).wrapping_mul(0x9E3779B97F4A7C15).rotate_left($((i % 63 + 1))) }"
  done
  echo "fn main() {"
  echo "    let mut x = 1u64;"
  for i in $(seq 0 19999); do echo "    x = f$i(x);"; done
  echo "    println!(\"{x}\");"
  echo "}"
} > buyuk/src/main.rs
