#@ video: 107-derleyici-ayarlari-lto-pgo
#@ degisken: süreler
k cargo build -q --release
k cargo build -q --profile kucuk
k cargo build -q --profile hizli
k cargo build -q --profile kucuk-semboller
ks 'stat -c "%8s bayt  %n" target/release/ayarlar target/kucuk/ayarlar target/kucuk-semboller/ayarlar target/hizli/ayarlar'
k hyperfine -N --warmup 3 --runs 30 --style basic ./target/release/ayarlar ./target/kucuk/ayarlar ./target/hizli/ayarlar
# PGO (profil güdümlü optimizasyon) için llvm-profdata gerekir: bu imajda kurulu mu?
ks 'rustup component list --installed | grep -c llvm-tools; command -v llvm-profdata || echo "llvm-profdata yok"'
