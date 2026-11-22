#@ video: 114-inceleme-listesi
k cargo run -q
# Varsayılan clippy ne diyor? (lint adları uyarıların yardım bağlantısından)
ks 'cargo clippy -q 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c'
# İnceleme için sıkı mod: pedantic + unwrap_used
ks 'cargo clippy -q -- -W clippy::pedantic -W clippy::unwrap_used 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c | sort -rn'
