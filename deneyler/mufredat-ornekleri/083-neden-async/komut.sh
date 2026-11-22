#@ video: 083-neden-async
#@ degisken: süreler ve bellek ölçümleri
k cat /sys/fs/cgroup/pids.max
k cargo build -q --release
k ./target/release/neden_async is-parcacigi 1000
k ./target/release/neden_async gorev 1000
k ./target/release/neden_async gorev 100000
# Bu konteynerde süreç/iş parçacığı sınırı var: 5000 iş parçacığı açılamaz
ks './target/release/neden_async is-parcacigi 5000 2>&1 | grep -E "panicked|failed|Resource" | head -n 3'
