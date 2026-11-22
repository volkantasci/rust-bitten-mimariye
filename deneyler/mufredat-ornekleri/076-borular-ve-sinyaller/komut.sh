#@ video: 076-borular-ve-sinyaller
#@ degisken: iş parçacığı kimliği
k cargo build -q
k ./target/debug/borular
# Okuyan taraf boruyu erken kapatırsa: Rust SIGPIPE'ı yok sayar, yazma EPIPE hatası döner, println! paniğe dönüşür
ks './target/debug/borular cok-yaz | head -n 2'
