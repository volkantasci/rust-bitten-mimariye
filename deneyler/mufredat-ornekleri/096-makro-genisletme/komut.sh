#@ video: 096-makro-genisletme
# Makrolar genişletildikten sonra derleyicinin gördüğü kod (yalnızca nightly'de: -Z bayrağı)
k cargo +nightly rustc -q -- -Zunpretty=expanded
