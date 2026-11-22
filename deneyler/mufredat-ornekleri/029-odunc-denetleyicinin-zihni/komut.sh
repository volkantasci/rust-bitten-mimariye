#@ video: 029-odunc-denetleyicinin-zihni
cd nll && k cargo run -q; cd ..
cd temkinli && k cargo build -q
# Yeni nesil denetleyici Polonius (yalnızca nightly'de, deneysel bayrakla)
ks 'rustc +nightly -Z help | grep -i polonius'
ks 'RUSTFLAGS="-Zpolonius=next" cargo +nightly run -q --target-dir target/polonius'
