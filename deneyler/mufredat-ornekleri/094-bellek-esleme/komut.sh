#@ video: 094-bellek-esleme
#@ degisken: sayfa hatası sayıları (çekirdek sürümü, dosya sistemi ve önbellek durumuna bağlı) ve eşleme adresleri (ASLR)
k cargo build -q --release
# Aynı 64 MiB'lık seyrek dosya üç kez eşlenir; her seferinde dosya baştan yazılır (sayfa önbelleği soğuk)
k ./target/release/mmap
# Programın çekirdekten istediği: mmap (MAP_POPULATE bayrağı) ve madvise (MADV_RANDOM)
ks 'strace -e trace=mmap,madvise,munmap ./target/release/mmap 2>&1 | grep -E "67108864"'
