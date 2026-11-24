#@ video: 060-ozellik-tabanli-test-ve-fuzzing
#@ degisken: proptest rastgele girdilerle başlar; küçültülmüş (shrink) çiftin sayıları ve süreler her çalıştırmada değişir, toplamları değişmez
rm -rf proptest-regressions
ks 'cargo test -q > /tmp/test.txt 2>&1; grep -m1 -B1 "attempt to add with overflow" /tmp/test.txt; grep -E "minimal failing input|^test result" /tmp/test.txt'
# Beklenen satırlar yoksa (ör. derleme ya da ağ hatası) test çıktısının sonu kayda düşsün
ks 'grep -q "minimal failing input" /tmp/test.txt || tail -n 20 /tmp/test.txt'
# Küçültülmüş çiftin toplamı: taşma sınırı 2^32 = 4294967296
ks 'grep -o "a = [0-9]*, b = [0-9]*" /tmp/test.txt | head -1 | tr -dc "0-9 \n" | awk "{print \"a + b =\", \$1 + \$2}"'
# proptest bulduğu karşı örneğin tohumunu bir regresyon dosyasına yazar; sonraki çalıştırma önce onu dener
ks 'ls proptest-regressions && grep -c "^cc " proptest-regressions/*.txt'
rm -rf proptest-regressions
