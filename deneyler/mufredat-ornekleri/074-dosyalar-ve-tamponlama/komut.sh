#@ video: 074-dosyalar-ve-tamponlama
#@ degisken: süreler; dosya tanımlayıcı numarası
k cargo build -q --release
# Tamponsuz yazmanın ilk sistem çağrıları: dosya açılır (fd 3), her satır parça parça yazılır
ks 'strace -e trace=openat,write ./target/release/tamponlama tamponsuz 2>&1 | grep -A6 "tamponsuz.txt"'
# 100 000 satır: kaç write çağrısı?
ks 'strace -c -e trace=write ./target/release/tamponlama tamponsuz 2>&1 | tail -n 4'
ks 'strace -c -e trace=write ./target/release/tamponlama tamponlu 2>&1 | tail -n 4'
k hyperfine -N --warmup 2 --runs 10 --style basic "./target/release/tamponlama tamponsuz" "./target/release/tamponlama tamponlu"
ks 'wc -c /tmp/tamponsuz.txt /tmp/tamponlu.txt && cmp /tmp/tamponsuz.txt /tmp/tamponlu.txt && echo "iki dosya aynı"'
# Kaynak (nightly rust-src): BufWriter/BufReader varsayılan tampon boyutu
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n -A3 "pub const DEFAULT_BUF_SIZE" $S/alloc/src/io/read.rs'
# Kendin dene: 64 KiB'lık tampon
ks 'strace -c -e trace=write ./target/release/tamponlama tamponlu64 2>&1 | tail -n 4'
