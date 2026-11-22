#@ video: 115-araclarla-dogrulama
#@ degisken: proptest'in bulduğu en küçük karşı örnek ve süreler değişebilir
rm -rf proptest-regressions
# 0) Bariz hâli: derleyicinin varsayılan "deny" uyarısı yakalar
cd bariz && k cargo build -q; cd ..
# 1) Dolaylı hâli derlenir; örnek testler geçer, farklılık (diferansiyel) testi patlar
ks 'cargo test -q 2>&1 | grep -E -A4 "^---- |minimal failing input" | grep -vE "^(--|thread|note:|\s*$)"; cargo test -q 2>&1 | grep "test result"'
# 2) miri: "geçen" sayaç testindeki tanımsız davranışı yakalar
ks 'cargo +nightly miri test -q sayac_artiyor 2>&1 | grep -E "Undefined Behavior|-->|test result" | head -n 4'
rm -rf proptest-regressions
