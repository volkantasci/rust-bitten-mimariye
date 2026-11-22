#@ video: 122-http-ayristirici
#@ ortak: ortak-projeler/kapi
cd ../ortak-projeler/kapi
ks 'cargo test -q --lib -- --list 2>/dev/null | grep ": test$"'
ks 'cargo test -q --lib 2>&1 | grep "test result"'
k cargo build -q --release
# Durum makinesi parça parça beslenirken
k ./target/release/kapi parca-parca
