#@ video: 065-bellek-ayiricilar
#@ degisken: blok adımları ve yeniden kullanım sırası sistemin ayırıcısına (burada glibc malloc) bağlıdır
k cargo run -q --release
# Kaynak (nightly rust-src): System ayırıcısı Unix'te küçük hizalamalar için libc'nin malloc'unu çağırır
ks 'S=$(rustc +nightly --print sysroot)/lib/rustlib/src/rust/library; grep -n "libc::malloc(layout.size())\|libc::realloc(ptr as \*mut libc::c_void, new_size)" $S/std/src/sys/alloc/unix.rs'
