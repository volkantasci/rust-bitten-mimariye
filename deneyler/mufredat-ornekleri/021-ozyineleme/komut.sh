#@ video: 021-ozyineleme
k cargo build -q
k ./target/debug/ozyineleme
# Ana iş parçacığının yığını 8 MiB (ulimit -s); 1 KiB'lik çerçevelerle 100 bin seviye sığmaz
k ulimit -s
k ./target/debug/ozyineleme derin 1000
k ./target/debug/ozyineleme derin 100000
# Daha büyük yığınla yeni bir iş parçacığı
k ./target/debug/ozyineleme buyuk-yigin 100000
# Kuyruk çağrısı garantisi: açık `become` bu sürümde kararlı mı?
cd kuyruk-cagrisi && ks 'cargo build -q 2>&1 | head -n 8'; cd ..
