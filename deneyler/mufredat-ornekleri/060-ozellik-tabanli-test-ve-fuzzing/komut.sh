#@ video: 060-ozellik-tabanli-test-ve-fuzzing
#@ degisken: proptest rastgele girdilerle başlar; küçültülmüş (shrink) örnek ve süreler her çalıştırmada değişebilir
rm -rf proptest-regressions
ks 'cargo test -q 2>&1 | grep -E "^---- |minimal failing input|successes|^test result"'
rm -rf proptest-regressions
