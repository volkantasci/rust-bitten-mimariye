#@ video: 036-panik-ve-kurtarilamaz-hatalar
#@ degisken: süreç kimliği (PID) ve panik satırındaki iş parçacığı kimliği (ana iş parçacığında aynı sayı); geri izdeki adres/çerçeve sayısı
#@ goster: panik::bol|panik::main|panicked at
k cargo run -q -- 2
k cargo run -q -- 0
ks 'RUST_BACKTRACE=1 cargo run -q -- 0 2>&1 | grep -E -A1 "panicked at|panik::"'
# panic = "abort" profili: çözülme yok, süreç SIGABRT ile biter
k cargo run -q --release -- 0
