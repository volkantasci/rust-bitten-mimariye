#@ video: 099-hir-ve-tip-denetimi
# HIR: sözdizimsel şekerler (for, ?) açılmış hâli
ks 'rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn topla/,/^}/p"'
ks 'rustc +nightly --edition 2024 -Zunpretty=hir seker.rs | sed -n "/fn oku/,/^}/p"'
