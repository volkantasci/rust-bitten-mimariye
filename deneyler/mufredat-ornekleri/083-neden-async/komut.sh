#@ video: 083-neden-async
#@ degisken: süreler ve bellek ölçümleri
# Bu konteynerde açılabilecek en fazla süreç + iş parçacığı sayısı (cgroup sınırı)
k cat /sys/fs/cgroup/pids.max
k cargo build -q --release
k ./target/release/neden_async is-parcacigi 1000
k ./target/release/neden_async gorev 1000
k ./target/release/neden_async gorev 100000
# Sanal bellek nereden geliyor? C kütüphanesinin iş parçacığı başına bellek alanlarını (arena) teke indir: geriye yığınlar kalır
ks 'MALLOC_ARENA_MAX=1 ./target/release/neden_async is-parcacigi 1000'
# Kendin dene: iş parçacığı yığınını 64 KiB'a indir (arenalar teke indirilmiş olarak ve tek başına)
ks 'MALLOC_ARENA_MAX=1 RUST_MIN_STACK=65536 ./target/release/neden_async is-parcacigi 1000'
ks 'RUST_MIN_STACK=65536 ./target/release/neden_async is-parcacigi 1000'
# Sınıra çarpmak: 5000 iş parçacığı açılamaz
ks './target/release/neden_async is-parcacigi 5000 2>&1 | grep -E "panicked|failed|Resource" | head -n 3'
