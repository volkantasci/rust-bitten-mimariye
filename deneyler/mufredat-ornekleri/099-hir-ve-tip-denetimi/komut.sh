#@ video: 099-hir-ve-tip-denetimi
# HIR: sözdizimsel şekerler (for, ?) açılmış hâli
ks 'rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn topla/,/^}/p"'
ks 'rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn oku/,/^}/p"'
# Tip denetimi açılmış kod üzerinde: hata mesajları açılımdaki trait'leri (IntoIterator, FromResidual) anıyor
ks 'rustc --edition 2024 --crate-type=lib -o /tmp/tip_hatasi.rlib tip_hatasi.rs 2>&1'
