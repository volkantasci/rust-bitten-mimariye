#@ video: 095-bildirimsel-makrolar
#@ degisken: hijyen işaretlerindeki numaralar (ad kimliği ve söz dizimi bağlamı) derleyici sürümüne göre değişebilir
k cargo run -q
# Genişletilmiş kod ve hijyen işaretleri (yalnızca nightly): aynı ad x, iki farklı söz dizimi bağlamı (#…)
ks 'cargo +nightly rustc -q -- -Zunpretty=expanded,hygiene 2>&1 | grep -A4 "iki_kati!(x) ="'
# Kendin dene: adı çağıran versin ($ad:ident); iki x de bizim bağlamımızda (#0), sonuç 2 * 2
ks 'rm -rf /tmp/makro-kopya && mkdir /tmp/makro-kopya && cp -r Cargo.toml Cargo.lock src /tmp/makro-kopya/ && cd /tmp/makro-kopya && sed -i "s/(\$e:expr) => {{/(\$ad:ident, \$e:expr) => {{/; s/let x = 2;/let \$ad = 2;/; s/x \* \$e/\$ad * \$e/; s/iki_kati!(x))/iki_kati!(x, x))/; s/(makronun x.i 2, bizim x.imiz 10)/(adı çağıran verdi: tek bir x)/" src/main.rs && cargo run -q | grep iki_kati && cargo +nightly rustc -q -- -Zunpretty=expanded,hygiene 2>&1 | grep -A4 "iki_kati!(x) ="'
