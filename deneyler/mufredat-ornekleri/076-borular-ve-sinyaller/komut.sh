#@ video: 076-borular-ve-sinyaller
#@ degisken: iş parçacığı kimliği; sa_restorer adresi; boru kapasitesi (varsayılan 65 536 bayt; kullanıcının toplam boru sayfaları pipe-user-pages-soft sınırını aşmışsa 8192)
k cargo build -q
k ./target/debug/borular
# Rust çalışma zamanı, main'den önce SIGPIPE'ı "yok say" olarak ayarlar
ks 'strace -e trace=rt_sigaction ./target/debug/borular kapasite 2>&1 | grep SIGPIPE'
# Boru tamponunun kapasitesi ve okuma ucu kapanınca yazma
# Kullanıcı başına boru sayfası yumuşak sınırı (pipe(7)): aşılınca yeni borular 2 sayfaya (8192 bayt) iner
k cat /proc/sys/fs/pipe-user-pages-soft
k ./target/debug/borular kapasite
# Okuyan taraf boruyu erken kapatırsa: yazma EPIPE hatası döner, println! paniğe dönüşür
ks './target/debug/borular cok-yaz | head -n 2'
# Kendin dene: hatayı yakalayıp sessizce bitirmek
ks './target/debug/borular dikkatli-yaz | head -n 2; echo "yazanın çıkış kodu: ${PIPESTATUS[0]}"'
