#@ video: 114-inceleme-listesi
k cargo run -q
# Ajanın testi geçiyor (testte yalnız kısa kayıtlar var)
ks 'cargo test -q 2>&1 | grep "test result"'
# Varsayılan clippy ne diyor? (lint adları uyarıların yardım bağlantısından)
ks 'cargo clippy -q 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c'
# İnceleme için sıkı mod: pedantic + unwrap_used
ks 'cargo clippy -q -- -W clippy::pedantic -W clippy::unwrap_used 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c | sort -rn'
# Kendin dene: deneysel (nursery) grup da açılınca liste değişiyor mu?
ks 'cargo clippy -q -- -W clippy::pedantic -W clippy::unwrap_used -W clippy::nursery 2>&1 | grep -oE "index.html#[a-z_]+" | sed "s/index.html#//" | sort | uniq -c | sort -rn'
ks 'cargo clippy -q -- -W clippy::nursery 2>&1 | grep -A3 "redundant clone"'
# Bütün lint grupları açıkken (all = correctness+suspicious+style+complexity+perf, ayrıca pedantic, restriction, nursery, cargo): 7. ve 10. satırlar
ks 'cargo clippy -q --message-format=short -- -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery -W clippy::cargo 2>&1 | grep -E "^src/main.rs:(7|10):" | sed -E "s/^src\/main.rs:([0-9]+):[0-9]+: warning: /satır \1: /" | sort | uniq'
